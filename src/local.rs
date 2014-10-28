//! Dumping ground for other state

use std::fmt;

use {Clear, Set};

use raw::{tcflag_t, mod};

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

        Ok(())
    }
}

static BITS: &'static [Bit] = &[
    ISIG, ICANON, ECHO, ECHOE, ECHOK, ECHONL, NOFLSH, TOSTOP, ECHOCTL, ECHOPRT, ECHOKE, FLUSHO,
    PENDIN, IEXTEN, EXTPROC,
];

#[deriving(Show)]
pub enum Bit {
    /// Enable signals INTR, QUIT, [D]SUSP
    ISIG,
    /// Canonalize input lines
    ICANON,
    /// Enable echoing
    ECHO,
    /// Visually erase chars
    ECHOE,
    /// Echo NL after line kill
    ECHOK,
    /// Echo NL even if ECHO is off
    ECHONL,
    /// Don't flush after interrupt
    NOFLSH,
    /// Stop background jobs from output
    TOSTOP,
    /// Echo control chars as ^(Char)
    ECHOCTL,
    /// Visual erase mode for hardcopy
    ECHOPRT,
    /// Visual erase for line kill
    ECHOKE,
    /// Output being flushed (state)
    FLUSHO,
    /// XXX retype pending input (state)
    PENDIN,
    /// Enable DISCARD and LNEXt
    IEXTEN,
    /// External processing
    EXTPROC,
}

impl Bit {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            ISIG => raw::ISIG,
            ICANON => raw::ICANON,
            ECHO => raw::ECHO,
            ECHOE => raw::ECHOE,
            ECHOK => raw::ECHOK,
            ECHONL => raw::ECHONL,
            NOFLSH => raw::NOFLSH,
            TOSTOP => raw::TOSTOP,
            ECHOCTL => raw::ECHOCTL,
            ECHOPRT => raw::ECHOPRT,
            ECHOKE => raw::ECHOKE,
            FLUSHO => raw::FLUSHO,
            PENDIN => raw::PENDIN,
            IEXTEN => raw::IEXTEN,
            EXTPROC => raw::EXTPROC,
        }
    }
}

impl Clear<Bit> for Flags {
    fn clear(&mut self, bit: Bit) {
        self.0 &= !(bit.to_raw())
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
