//! Software input processing

use std::fmt;

use Termios;
use raw::{tcflag_t, self};
use self::Flag::*;
use traits::{Clear, Contains, Set};

/// Input flags
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

const FLAGS: [Flag; 13] = [
    BRKINT,
    ICRNL,
    IGNBRK,
    IGNCR,
    IGNPAR,
    IMAXBEL,
    INLCR,
    INPCK,
    ISTRIP,
    IXANY,
    IXOFF,
    IXON,
    PARMRK,
];

/// Standard input flags
#[derive(Copy, Debug)]
pub enum Flag {
    /// Map BREAK to SIGINTR
    BRKINT,
    /// Map CR to NL (ala CRMOD)
    ICRNL,
    /// Ignore BREAK condition
    IGNBRK,
    /// Ignore CR
    IGNCR,
    /// Ignore (discard) parity errors
    IGNPAR,
    /// Ring bell on input queue full
    IMAXBEL,
    /// Map NL into CR
    INLCR,
    /// Enable checking of parity errors
    INPCK,
    /// Strip 8th bit off chars
    ISTRIP,
    /// Any char will restart after stop
    IXANY,
    /// Enable input control flow
    IXOFF,
    /// Enable output flow control
    IXON,
    /// Mark parity and framing errors
    PARMRK,
}

impl Flag {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            BRKINT => raw::BRKINT,
            ICRNL => raw::ICRNL,
            IGNBRK => raw::IGNBRK,
            IGNCR => raw::IGNCR,
            IGNPAR => raw::IGNPAR,
            IMAXBEL => raw::IMAXBEL,
            INLCR => raw::INLCR,
            INPCK => raw::INPCK,
            ISTRIP => raw::ISTRIP,
            IXANY => raw::IXANY,
            IXOFF => raw::IXOFF,
            IXON => raw::IXON,
            PARMRK => raw::PARMRK,
        }
    }
}

impl Clear<Flag> for Termios {
    fn clear(&mut self, flag: Flag) {
        self.iflag.0 &= !flag.to_raw()
    }
}

impl Contains<Flag> for Termios {
    fn contains(&self, flag: Flag) -> bool {
        let flag = flag.to_raw();

        self.iflag.0 & flag == flag
    }
}

impl Set<Flag> for Termios {
    fn set(&mut self, flag: Flag) {
        self.iflag.0 |= flag.to_raw()
    }
}
