//! Hardware control of terminal

use std::fmt;
use std::ops::{Index, IndexMut};

use Termios;
use raw::{cc_t, tcflag_t, self};
use self::CSIZE::*;
use self::Char::*;
use self::Flag::*;
use traits::{Clear, Contains, Get, GetFrom, Set};

/// Control flags
#[derive(Copy)]
#[repr(C)]
pub struct Flags(tcflag_t);

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:?}", CSIZE::from_raw(self.0 & CSIZE_MASK)));

        for &flag in &FLAGS {
            let value = flag.to_raw();

            if self.0 & value == value {
                try!(write!(f, " | {:?}", flag))
            }
        }

        Ok(())
    }
}

const FLAGS: [Flag; 7] = [
    CLOCAL,
    CREAD,
    CRTSCTS,
    CSTOPB,
    HUPCL,
    PARENB,
    PARODD,
];

/// Standard control flags
#[derive(Copy, Debug)]
pub enum Flag {
    /// Ignore modem status lines
    CLOCAL,
    /// Enable receiver
    CREAD,
    /// RTS/CTS full-duplex flow control
    CRTSCTS,
    /// Send 2 STOP bits
    CSTOPB,
    /// Hang up on last close
    HUPCL,
    /// Parity enable
    PARENB,
    /// Odd parity, else even
    PARODD,
}

impl Flag {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            CLOCAL => raw::CLOCAL,
            CREAD => raw::CREAD,
            CRTSCTS => raw::CRTSCTS,
            CSTOPB => raw::CSTOPB,
            HUPCL => raw::HUPCL,
            PARENB => raw::PARENB,
            PARODD => raw::PARODD,
        }
    }
}

impl Clear<Flag> for Termios {
    fn clear(&mut self, flag: Flag) {
        self.cflag.0 &= !flag.to_raw()
    }
}

impl Contains<Flag> for Termios {
    fn contains(&self, flag: Flag) -> bool {
        let flag = flag.to_raw();

        self.cflag.0 & flag == flag
    }
}

impl Set<Flag> for Termios {
    fn set(&mut self, flag: Flag) {
        self.cflag.0 |= flag.to_raw()
    }
}

/// Character size
#[derive(Copy, Debug)]
pub enum CSIZE {
    /// 5 bits (pseudo)
    CS5,
    /// 6 bits
    CS6,
    /// 7 bits
    CS7,
    /// 8 bits
    CS8,
}

impl CSIZE {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            CS5 => raw::CS5,
            CS6 => raw::CS6,
            CS7 => raw::CS7,
            CS8 => raw::CS8,
        }
    }

    fn from_raw(csize: tcflag_t) -> CSIZE {
        match csize {
            raw::CS5 => CS5,
            raw::CS6 => CS6,
            raw::CS7 => CS7,
            raw::CS8 => CS8,
            _ => panic!("Unknown CSIZE state: {}", csize),
        }
    }
}

const CSIZE_MASK: tcflag_t = raw::CS5 | raw::CS6 | raw::CS7 | raw::CS8;

impl Get for Termios {}

impl GetFrom<Termios> for CSIZE {
    fn get_from(termios: &Termios) -> CSIZE {
        CSIZE::from_raw(termios.cflag.0 & CSIZE_MASK)
    }
}

impl Set<CSIZE> for Termios {
    fn set(&mut self, csize: CSIZE) {
        self.cflag.0 &= !CSIZE_MASK;
        self.cflag.0 |= csize.to_raw();
    }
}

/// Control chars
#[derive(Copy)]
#[repr(C)]
pub struct Chars([cc_t; raw::NCCS as usize]);

impl Index<Char> for Chars {
    type Output = cc_t;

    fn index(&self, &char: &Char) -> &cc_t {
        &self.0[char.to_raw()]
    }
}

impl IndexMut<Char> for Chars {
    fn index_mut(&mut self, &char: &Char) -> &mut cc_t {
        &mut self.0[char.to_raw()]
    }
}

impl fmt::Debug for Chars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;

        for &char in &CHARS {
            if is_first {
                is_first = false;

                try!(write!(f, "{:?}: {:?}", char, self[char]));
            } else {
                try!(write!(f, ", {:?}: {:?}", char, self[char]));
            }
        }

        Ok(())
    }
}

const CHARS: [Char; 16] = [
    VDISCARD,
    VEOF,
    VEOL2,
    VEOL,
    VERASE,
    VINTR,
    VKILL,
    VLNEXT,
    VMIN,
    VQUIT,
    VREPRINT,
    VSTART,
    VSTOP,
    VSUSP,
    VTIME,
    VWERASE,
];

/// Standard control chars
#[allow(missing_docs)]
#[derive(Copy, Debug)]
pub enum Char {
    VDISCARD,
    VEOF,
    VEOL2,
    VEOL,
    VERASE,
    VINTR,
    VKILL,
    VLNEXT,
    VMIN,
    VQUIT,
    VREPRINT,
    VSTART,
    VSTOP,
    VSUSP,
    VTIME,
    VWERASE,
}

impl Char {
    fn to_raw(&self) -> usize {
        (match *self {
            VDISCARD => raw::VDISCARD,
            VEOF => raw::VEOF,
            VEOL => raw::VEOL,
            VEOL2 => raw::VEOL2,
            VERASE => raw::VERASE,
            VINTR => raw::VINTR,
            VKILL => raw::VKILL,
            VLNEXT => raw::VLNEXT,
            VMIN => raw::VMIN,
            VQUIT => raw::VQUIT,
            VREPRINT => raw::VREPRINT,
            VSTART => raw::VSTART,
            VSTOP => raw::VSTOP,
            VSUSP => raw::VSUSP,
            VTIME => raw::VTIME,
            VWERASE => raw::VWERASE,
        }) as usize
    }
}
