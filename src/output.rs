//! Software output processing

use std::fmt;

use Termios;
use raw::{tcflag_t, self};
use self::Flag::*;
use traits::{Clear, Contains, Set};

/// Output flags
#[derive(Copy)]
#[repr(C)]
pub struct Flags(tcflag_t);

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;

        for &flag in &FLAGS {
            let value = flag.to_raw();

            if self.0 & value == value {
                if is_first {
                    is_first = false;

                    try!(write!(f, "{:?}", flag));
                } else {
                    try!(write!(f, " | {:?}", flag));
                }
            }
        }

        Ok(())
    }
}

const FLAGS: [Flag; 5] = [
    OCRNL,
    ONLCR,
    ONLRET,
    ONOCR,
    OPOST,
];

/// Standard output flags
#[derive(Copy, Debug)]
pub enum Flag {
    /// Map CR to NL on output
    OCRNL,
    /// Map NL to CR-NL (ala CRMOD)
    ONLCR,
    /// NL performs CR function
    ONLRET,
    /// No CR output at column 0
    ONOCR,
    /// Enable following output processing
    OPOST,
}

impl Flag {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            OCRNL => raw::OCRNL,
            ONLCR => raw::ONLCR,
            ONLRET => raw::ONLRET,
            ONOCR => raw::ONOCR,
            OPOST => raw::OPOST,
        }
    }
}

impl Clear<Flag> for Termios {
    fn clear(&mut self, flag: Flag) {
        self.oflag.0 &= !(flag.to_raw())
    }
}

impl Contains<Flag> for Termios {
    fn contains(&self, flag: Flag) -> bool {
        let flag = flag.to_raw();

        self.oflag.0 & flag == flag
    }
}

impl Set<Flag> for Termios {
    fn set(&mut self, flag: Flag) {
        self.oflag.0 |= flag.to_raw()
    }
}
