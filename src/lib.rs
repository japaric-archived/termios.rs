#![deny(missing_docs, warnings)]
#![feature(io)]
#![feature(libc)]

//! Termios bindings + safe wrapper
//!
//! # Examples
//!
//! Check the [`Termios`](struct.Termios.html#method.fetch) struct.

extern crate libc;

use libc::c_int;
use std::default::Default;
use std::{fmt, io, mem};

use self::BaudRate::*;
use self::When::*;

pub mod control;
pub mod input;
pub mod local;
pub mod output;
pub mod prelude;
pub mod raw;
pub mod traits;

const FAILURE: c_int = -1;
const SUCCESS: c_int = 0;

/// Safe wrapper around `raw::Termios`
#[derive(Copy)]
#[repr(C)]
pub struct Termios {
    /// Input flags
    pub iflag: input::Flags,
    /// Output flags
    pub oflag: output::Flags,
    /// Control flags
    pub cflag: control::Flags,
    /// Local flags
    pub lflag: local::Flags,
    #[cfg(target_os = "linux")] _line: raw::cc_t,
    /// Control characters
    pub cc: control::Chars,
    _ispeed: raw::speed_t,
    _ospeed: raw::speed_t,
}

impl Termios {
    /// Returns the `Termios` structure associated with the `fd` (file descriptor)
    ///
    /// # Examples
    ///
    /// Inspect your TTY.
    ///
    /// ``` no_run
    /// // examples/stdin.rs
    /// extern crate libc;
    /// extern crate termios;
    ///
    /// use termios::prelude::*;
    ///
    /// fn main() {
    ///     println!("{:?}", Termios::fetch(libc::STDIN_FILENO).unwrap());
    /// }
    /// ```
    ///
    /// ``` text
    /// $ ./stdin
    /// iflag:  ICRNL | IXON
    /// oflag:  ONLCR | OPOST
    /// cflag:  CS8 | CREAD
    /// lflag:  ECHOCTL | ECHOE | ECHOKE | ECHOK | ECHO | ICANON | IEXTEN | ISIG
    /// cc:     VDISCARD: 15, VEOF: 4, VEOL2: 0, (..)
    /// ispeed: B38400
    /// ospeed: B38400
    ///
    /// $ echo 'this will panic!' | ./stdin
    /// called `Result::unwrap()` on an `Err` value: file descriptor is not a TTY
    /// ```
    ///
    /// The second call panicked because `./stdin`'s stdin was not connected to a TTY, the
    /// following ASCII art shows how the standard streams are wired in that case:
    ///
    /// ``` text
    ///        +-----------------+  `|`   +---------------------+
    /// TTY => |stdin      stdout| =====> |stdin          stdout| => TTY
    ///        |     `echo`      |        |     `./stdin`       |
    ///        |           stderr| => TTY |               stderr| => TTY
    ///        +-----------------+        +---------------------+
    /// ```
    pub fn fetch(fd: c_int) -> io::Result<Termios> {
        let mut termios: raw::Termios = Default::default();

        unsafe {
            match raw::tcgetattr(fd, &mut termios) {
                FAILURE => Err(io::Error::last_os_error()),
                SUCCESS => Ok(Termios::from_raw(termios)),
                _ => unreachable!(),
            }
        }
    }

    /// Updates the `Termios` structure associated with the `fd` (file descriptor)
    ///
    /// # Examples
    ///
    /// Unbuffered TTY input.
    ///
    /// Consider the following application:
    ///
    /// ``` no_run
    /// // examples/buffered.rs
    /// use std::io::{ReadExt, self};
    ///
    /// fn main() {
    ///     let stdin = io::stdin();
    ///
    ///     for byte in stdin.lock().bytes() {
    ///         match byte {
    ///             Err(e) => panic!("{}", e),
    ///             Ok(byte) => println!("Got {}", byte),
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// You'll expect the application to print back each byte as you type, however if you run the
    /// application and type: `a b c ENTER CTRL+D`, you get the following output:
    ///
    /// ``` text
    /// $ ./buffered
    /// abc
    /// Got 97
    /// Got 98
    /// Got 99
    /// Got 10
    /// ```
    ///
    /// You'll notice two things:
    ///
    /// - The application prints back what you type.
    /// - The application doesn't process your input until you hit ENTER.
    ///
    /// The difference between the observed behavior and your expectations is caused by the TTY
    /// (the terminal emulator you are using). By default, the TTY echoes back what you type (this
    /// is essential when you are typing commands in a shell!), and also buffers (line by line)
    /// your input before sending it to the application.
    ///
    /// If you want the application to match your expectations, then you'll have to modify the
    /// behavior of the TTY using `Termios`. The following code disables both echoing and line
    /// buffering:
    ///
    /// ``` no_run
    /// // examples/unbuffered.rs
    /// extern crate libc;
    /// extern crate termios;
    ///
    /// use std::io::{ReadExt, self};
    ///
    /// use termios::prelude::*;
    ///
    /// // a.k.a. "Ctrl+D"
    /// const END_OF_TRANSMISSION: u8 = 4;
    ///
    /// fn main() {
    ///     let old_termios = Termios::fetch(libc::STDIN_FILENO).unwrap();
    ///     let mut new_termios = old_termios;
    ///
    ///     // Disable line buffering
    ///     new_termios.clear(local::Flag::ICANON);
    ///
    ///     // Disable echo
    ///     new_termios.clear(local::Flag::ECHO);
    ///
    ///     // Change the behavior of the TTY
    ///     new_termios.update(libc::STDIN_FILENO, When::Now).unwrap();
    ///
    ///     let stdin = io::stdin();
    ///
    ///     for byte in stdin.lock().bytes() {
    ///         match byte {
    ///             Err(e) => panic!("{}", e),
    ///             Ok(END_OF_TRANSMISSION) => break,
    ///             Ok(byte) => println!("Got {}", byte),
    ///         }
    ///     }
    ///
    ///     // Restore the original behavior of the TTY
    ///     old_termios.update(libc::STDIN_FILENO, When::Now).unwrap();
    /// }
    /// ```
    ///
    /// If you run this example, you'll receive the `"Got XYZ"` message each time you press a key.
    pub fn update(&self, fd: c_int, when: When) -> io::Result<()> {
        unsafe {
            match raw::tcsetattr(fd, when.to_raw(), self.as_raw()) {
                FAILURE => Err(io::Error::last_os_error()),
                SUCCESS => Ok(()),
                _ => unreachable!(),
            }
        }
    }

    /// Puts `Termios` in "raw" mode
    ///
    /// # Examples
    ///
    /// Compare "cooked" and "raw" modes
    ///
    /// ``` no_run
    /// // examples/raw.rs
    /// extern crate libc;
    /// extern crate termios;
    ///
    /// use termios::prelude::*;
    ///
    /// fn main() {
    ///     let mut termios = Termios::fetch(libc::STDIN_FILENO).unwrap();
    ///     println!("Cooked:\n{:?}", termios);
    ///     termios.make_raw();
    ///     println!("\nRaw:\n{:?}", termios);
    /// }
    /// ```
    ///
    /// ``` text
    /// Cooked:
    /// iflag:  ICRNL | IXON
    /// oflag:  ONLCR | OPOST
    /// cflag:  CS8 | CREAD
    /// lflag:  ECHOCTL | ECHOE | ECHOKE | ECHOK | ECHO | ICANON | IEXTEN | ISIG
    /// cc:     VDISCARD: 15, VEOF: 4, VEOL2: 0, (..)
    /// ispeed: B38400
    /// ospeed: B38400
    ///
    /// Raw:
    /// iflag:
    /// oflag:  ONLCR
    /// cflag:  CS8 | CREAD
    /// lflag:  ECHOCTL | ECHOE | ECHOKE | ECHOK
    /// cc:     VDISCARD: 15, VEOF: 4, VEOL2: 0, (..)
    /// ispeed: B38400
    /// ospeed: B38400
    /// ```
    pub fn make_raw(&mut self) {
        unsafe {
            raw::cfmakeraw(self.as_raw_mut())
        }
    }

    /// Returns the input baud rate
    pub fn ispeed(&self) -> BaudRate {
        unsafe {
            BaudRate::from_raw(raw::cfgetispeed(self.as_raw()))
        }
    }

    /// Returns the output baud rate
    pub fn ospeed(&self) -> BaudRate {
        unsafe {
            BaudRate::from_raw(raw::cfgetospeed(self.as_raw()))
        }
    }

    /// Sets the input baud rate
    pub fn set_ispeed(&mut self, rate: BaudRate) {
        unsafe {
            match raw::cfsetispeed(self.as_raw_mut(), rate.to_raw()) {
                FAILURE => {
                    // NB This operation can only panic if self is null (impossible in safe code)
                    // or if `rate.to_raw()` is an invalid value (that would be a bug)
                    unreachable!();
                },
                SUCCESS => {},
                _ => unreachable!(),
            }
        }
    }

    /// Sets the output baud rate
    pub fn set_ospeed(&mut self, rate: BaudRate) {
        unsafe {
            match raw::cfsetospeed(self.as_raw_mut(), rate.to_raw()) {
                FAILURE => {
                    // NB This operation can only panic if self is null (impossible in safe code)
                    // or if `rate.to_raw()` is an invalid value (that would be a bug)
                    unreachable!();
                },
                SUCCESS => {},
                _ => unreachable!(),
            }
        }
    }

    /// Sets both the input and the output baud rates
    pub fn set_speed(&mut self, rate: BaudRate) {
        unsafe {
            match raw::cfsetspeed(self.as_raw_mut(), rate.to_raw()) {
                FAILURE => {
                    // NB This operation can only panic if self is null (impossible in safe code)
                    // or if `rate.to_raw()` is an invalid value (that would be a bug)
                    unreachable!();
                },
                SUCCESS => {},
                _ => unreachable!(),
            }
        }
    }

    /// Borrows the safe wrapper as its raw representation
    pub fn as_raw(&self) -> &raw::Termios {
        unsafe {
            mem::transmute(self)
        }
    }

    /// Mutably borrows the safe wrapper as its raw representation
    pub unsafe fn as_raw_mut(&mut self) -> &mut raw::Termios {
        mem::transmute(self)
    }

    /// Puts `raw::Termios` into a safe wrapper without performing any check
    pub unsafe fn from_raw(termios: raw::Termios) -> Termios {
        mem::transmute(termios)
    }

    /// Converts the safe wrapper into its raw representation
    pub fn into_raw(self) -> raw::Termios {
        unsafe {
            mem::transmute(self)
        }
    }
}

// XXX (Debug) Formatting may change
impl fmt::Debug for Termios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "iflag:\t{:?}", self.iflag));
        try!(writeln!(f, "oflag:\t{:?}", self.oflag));
        try!(writeln!(f, "cflag:\t{:?}", self.cflag));
        try!(writeln!(f, "lflag:\t{:?}", self.lflag));
        try!(writeln!(f, "cc:\t{:?}", self.cc));
        try!(writeln!(f, "ispeed:\t{:?}", self.ispeed()));
        try!(write!(f, "ospeed:\t{:?}", self.ospeed()));
        Ok(())
    }
}

/// Standard baud rates
#[allow(missing_docs)]
#[derive(Copy, Debug, PartialEq)]
pub enum BaudRate {
    B0,
    B50,
    B75,
    B110,
    B134,
    B150,
    B200,
    B300,
    B600,
    B1200,
    B1800,
    B2400,
    B4800,
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
    B230400,
}

impl BaudRate {
    fn from_raw(speed: raw::tcflag_t) -> BaudRate {
        use BaudRate::*;

        match speed {
            raw::B0 => B0,
            raw::B50 => B50,
            raw::B75 => B75,
            raw::B110 => B110,
            raw::B134 => B134,
            raw::B150 => B150,
            raw::B200 => B200,
            raw::B300 => B300,
            raw::B600 => B600,
            raw::B1200 => B1200,
            raw::B1800 => B1800,
            raw::B2400 => B2400,
            raw::B4800 => B4800,
            raw::B9600 => B9600,
            raw::B19200 => B19200,
            raw::B38400 => B38400,
            raw::B57600 => B57600,
            raw::B115200 => B115200,
            raw::B230400 => B230400,
            _ => panic!("Unknown baud rate flag: {}", speed),
        }
    }

    fn to_raw(&self) -> raw::speed_t {
        match *self {
            B0 => raw::B0,
            B50 => raw::B50,
            B75 => raw::B75,
            B110 => raw::B110,
            B134 => raw::B134,
            B150 => raw::B150,
            B200 => raw::B200,
            B300 => raw::B300,
            B600 => raw::B600,
            B1200 => raw::B1200,
            B1800 => raw::B1800,
            B2400 => raw::B2400,
            B4800 => raw::B4800,
            B9600 => raw::B9600,
            B19200 => raw::B19200,
            B38400 => raw::B38400,
            B57600 => raw::B57600,
            B115200 => raw::B115200,
            B230400 => raw::B230400,
        }
    }
}

/// When to update the underlying `raw::Termios` structure
pub enum When {
    /// Make change immediate
    Now,
    /// Drain output, then change
    AfterDrain,
    /// Drain output, flush input, then change
    AfterFlush,
}

impl When {
    fn to_raw(&self) -> c_int {
        match *self {
            Now => raw::TCSANOW,
            AfterDrain => raw::TCSADRAIN,
            AfterFlush => raw::TCSAFLUSH,
        }
    }
}

#[cfg(test)]
mod test {
    use libc;

    use Termios;

    #[test]
    fn stderr() {
        Termios::fetch(libc::STDERR_FILENO).unwrap();
    }

    #[test]
    fn stdin() {
        Termios::fetch(libc::STDIN_FILENO).unwrap();
    }

    #[test]
    fn stdout() {
        Termios::fetch(libc::STDOUT_FILENO).unwrap();
    }
}
