//! Software input processing

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
    IGNBRK, BRKINT, IGNPAR, PARMRK, INPCK, ISTRIP, INLCR, IGNCR, ICRNL, IXON, IXANY, IXOFF,
    IMAXBEL, IUTF8,
];

#[deriving(Show)]
pub enum Bit {
    /// Ignore BREAK condition
    IGNBRK,
    /// Map BREAK to SIGINTR
    BRKINT,
    /// Ignore (discard) parity errors
    IGNPAR,
    /// Mark parity and framing errors
    PARMRK,
    /// Enable checking of parity errors
    INPCK,
    /// Strip 8th bit off chars
    ISTRIP,
    /// Map NL into CR
    INLCR,
    /// Ignore CR
    IGNCR,
    /// Map CR to NL (ala CRMOD)
    ICRNL,
    /// Enable output flow control
    IXON,
    /// Any char will restart after stop
    IXANY,
    /// Enable input control flow
    IXOFF,
    /// Ring bell on input queue full
    IMAXBEL,
    /// Maintain state for UTF-8 VERASE
    IUTF8,
}

impl Bit {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            IGNBRK => raw::IGNBRK,
            BRKINT => raw::BRKINT,
            IGNPAR => raw::IGNPAR,
            PARMRK => raw::PARMRK,
            INPCK => raw::INPCK,
            ISTRIP => raw::ISTRIP,
            INLCR => raw::INLCR,
            IGNCR => raw::IGNCR,
            ICRNL => raw::ICRNL,
            IXON => raw::IXON,
            IXANY => raw::IXANY,
            IXOFF => raw::IXOFF,
            IMAXBEL => raw::IMAXBEL,
            IUTF8 => raw::IUTF8,
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
