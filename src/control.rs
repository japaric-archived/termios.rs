//! Hardware control of terminal

use std::fmt;

use raw::{cc_t, tcflag_t, mod};
use {Clear, Get, GetFrom, Set};

#[repr(C)]
pub struct Flags(tcflag_t);

impl fmt::Show for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;

        for &bit in BITS.iter().filter(|&&bit| self.contains(bit)) {
            if is_first {
                is_first = false;

                try!(write!(f, "{}", bit));
            } else {
                try!(write!(f, " | {}", bit));
            }
        }

        for &csize in CSIZES.iter().filter(|&&csize| self.contains(csize)) {
            if is_first {
                is_first = false;

                try!(write!(f, "{}", csize));
            } else {
                try!(write!(f, " | {}", csize));
            }
        }

        Ok(())
    }
}

static BITS: &'static [Bit] = &[CSTOPB, CREAD, PARENB, PARODD, HUPCL, CLOCAL, CRTSCTS];

#[deriving(Show)]
pub enum Bit {
    /// Send 2 STOP bits
    CSTOPB,
    /// Enable receiver
    CREAD,
    /// Parity enable
    PARENB,
    /// Odd parity, else even
    PARODD,
    /// Hang up on last close
    HUPCL,
    /// Ignore modem status lines
    CLOCAL,
    /// RTS/CTS full-duplex flow control
    CRTSCTS,
}

impl Bit {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            CSTOPB => raw::CSTOPB,
            CREAD => raw::CREAD,
            PARENB => raw::PARENB,
            PARODD => raw::PARODD,
            HUPCL => raw::HUPCL,
            CLOCAL => raw::CLOCAL,
            CRTSCTS => raw::CRTSCTS,
        }
    }
}

impl Clear<Bit> for Flags {
    fn clear(&mut self, bit: Bit) {
        self.0 &= !bit.to_raw()
    }
}

impl Set<Bit> for Flags {
    fn contains(&self, bit: Bit) -> bool {
        let bit = bit.to_raw();

        self.0 & bit == bit
    }

    fn set(&mut self, bit: Bit) {
        self.0 |= bit.to_raw()
    }
}

#[deriving(FromPrimitive, Show)]
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
            _ => fail!("Unknown CSIZE state: {}", csize),
        }
    }
}

const CSIZE_MASK: tcflag_t = raw::CS8;
static CSIZES: &'static [CSIZE] = &[CS5, CS6, CS7, CS8];

impl Get for Flags {}

impl GetFrom<Flags> for CSIZE {
    fn get_from(flags: &Flags) -> CSIZE {
        CSIZE::from_raw(flags.0 & CSIZE_MASK)
    }
}

impl Set<CSIZE> for Flags {
    fn contains(&self, csize: CSIZE) -> bool {
        self.0 & CSIZE_MASK == csize.to_raw()
    }

    fn set(&mut self, csize: CSIZE) {
        self.0 &= !CSIZE_MASK;
        self.0 |= csize.to_raw();
    }
}

#[repr(C)]
pub struct Chars([cc_t, ..raw::NCCS]);

impl Index<Char, cc_t> for Chars {
    fn index(&self, &char: &Char) -> &cc_t {
        &self.0[char.to_raw()]
    }
}

impl IndexMut<Char, cc_t> for Chars {
    fn index_mut(&mut self, &char: &Char) -> &mut cc_t {
        &mut self.0[char.to_raw()]
    }
}

// FIXME (Show) This should be done without allocating the `Vec`
impl fmt::Show for Chars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        CHARS.iter().map(|&char| (char, self[char])).collect::<Vec<_>>().fmt(f)
    }
}

static CHARS: &'static [Char] = &[
    VINTR, VQUIT, VERASE, VKILL, VEOF, VTIME, VMIN, VSTART, VSTOP, VSUSP, VEOL, VREPRINT,
    VDISCARD, VWERASE, VLNEXT, VEOL2,
];

#[deriving(Show)]
pub enum Char {
    VINTR,
    VQUIT,
    VERASE,
    VKILL,
    VEOF,
    VTIME,
    VMIN,
    VSTART,
    VSTOP,
    VSUSP,
    VEOL,
    VREPRINT,
    VDISCARD,
    VWERASE,
    VLNEXT,
    VEOL2,
}

impl Char {
    fn to_raw(&self) -> uint {
        match *self {
            VINTR => raw::VINTR,
            VQUIT => raw::VQUIT,
            VERASE => raw::VERASE,
            VKILL => raw::VKILL,
            VEOF => raw::VEOF,
            VTIME => raw::VTIME,
            VMIN => raw::VMIN,
            VSTART => raw::VSTART,
            VSTOP => raw::VSTOP,
            VSUSP => raw::VSUSP,
            VEOL => raw::VEOL,
            VREPRINT => raw::VREPRINT,
            VDISCARD => raw::VDISCARD,
            VWERASE => raw::VWERASE,
            VLNEXT => raw::VLNEXT,
            VEOL2 => raw::VEOL2,
        }
    }
}
