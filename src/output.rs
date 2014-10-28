//! Software output processing

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

static BITS: &'static [Bit] = &[OPOST, ONLCR, OCRNL, ONOCR, ONLRET, OFILL, OFDEL];

#[deriving(Show)]
pub enum Bit {
    /// Enable following output processing
    OPOST,
    /// Map NL to CR-NL (ala CRMOD)
    ONLCR,
    /// Map CR to NL on output
    OCRNL,
    /// No CR output at column 0
    ONOCR,
    /// NL performs CR function
    ONLRET,
    /// Use fill characters for delay
    OFILL,
    /// Fill is DEL, else NUL
    OFDEL,
}

impl Bit {
    fn to_raw(&self) -> tcflag_t {
        match *self {
            OPOST => raw::OPOST,
            ONLCR => raw::ONLCR,
            OCRNL => raw::OCRNL,
            ONOCR => raw::ONOCR,
            ONLRET => raw::ONLRET,
            OFILL => raw::OFILL,
            OFDEL => raw::OFDEL,
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
