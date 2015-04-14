//! Dumping ground for other state

use std::fmt;

use Termios;
use raw::{tcflag_t, self};
use self::Flag::*;
use traits::{Clear, Contains, Set};

/// Local flags
#[derive(Clone, Copy)]
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

const FLAGS: [Flag; 15] = [
    ECHOCTL,
    ECHOE,
    ECHOKE,
    ECHOK,
    ECHONL,
    ECHOPRT,
    ECHO,
    EXTPROC,
    FLUSHO,
    ICANON,
    IEXTEN,
    ISIG,
    NOFLSH,
    PENDIN,
    TOSTOP,
];

/// Standard local flags
#[derive(Clone, Copy, Debug)]
pub enum Flag {
    /// Echo control chars as `^(Char)`
    ECHOCTL,
    /// Visually erase chars
    ECHOE,
    /// Visual erase for line kill
    ECHOKE,
    /// Echo NL after line kill
    ECHOK,
    /// Echo NL even if ECHO is off
    ECHONL,
    /// Visual erase mode for hardcopy
    ECHOPRT,
    /// Enable echoing
    ECHO,
    /// External processing
    EXTPROC,
    /// Output being flushed (state)
    FLUSHO,
    /// Canonalize input lines
    ICANON,
    /// Enable DISCARD and LNEXt
    IEXTEN,
    /// Enable signals INTR, QUIT, [D]SUSP
    ISIG,
    /// Don't flush after interrupt
    NOFLSH,
    /// XXX retype pending input (state)
    PENDIN,
    /// Stop background jobs from output
    TOSTOP,
}

impl Flag {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            ECHO => raw::ECHO,
            ECHOCTL => raw::ECHOCTL,
            ECHOE => raw::ECHOE,
            ECHOK => raw::ECHOK,
            ECHOKE => raw::ECHOKE,
            ECHONL => raw::ECHONL,
            ECHOPRT => raw::ECHOPRT,
            EXTPROC => raw::EXTPROC,
            FLUSHO => raw::FLUSHO,
            ICANON => raw::ICANON,
            IEXTEN => raw::IEXTEN,
            ISIG => raw::ISIG,
            NOFLSH => raw::NOFLSH,
            PENDIN => raw::PENDIN,
            TOSTOP => raw::TOSTOP,
        }
    }
}

impl Clear<Flag> for Termios {
    fn clear(&mut self, flag: Flag) {
        self.lflag.0 &= !flag.to_raw()
    }
}

impl Contains<Flag> for Termios {
    fn contains(&self, flag: Flag) -> bool {
        let flag = flag.to_raw();

        self.lflag.0 & flag == flag
    }
}

impl Set<Flag> for Termios {
    fn set(&mut self, flag: Flag) {
        self.lflag.0 |= flag.to_raw()
    }
}
