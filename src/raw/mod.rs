//! Raw access to the `Termios` structure and its flags

#[allow(dead_code, missing_docs, non_camel_case_types)]
mod ffi;

use std::fmt;

pub use self::ffi::cc_t;
pub use self::ffi::speed_t;
pub use self::ffi::tcflag_t;

pub use self::ffi::termios as Termios;

pub use self::ffi::{
    cfgetispeed,
    cfgetospeed,
    cfmakeraw,
    cfsetispeed,
    cfsetospeed,
    cfsetspeed,
    tcdrain,
    tcflow,
    tcflush,
    tcgetattr,
    tcsendbreak,
    tcsetattr,
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    cfmakesane,
    tcgetsid,
    tcsetsid,
};

// XXX (Debug) Formatting may change
impl fmt::Debug for Termios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "iflag:\t{:?}", self.c_iflag));
        try!(writeln!(f, "oflag:\t{:?}", self.c_oflag));
        try!(writeln!(f, "cflag:\t{:?}", self.c_cflag));
        try!(writeln!(f, "lflag:\t{:?}", self.c_lflag));
        try!(writeln!(f, "cc:\t{:?}", self.c_cc));
        try!(writeln!(f, "ispeed:\t{:?}", self.c_ispeed));
        try!(write!(f, "ospeed:\t{:?}", self.c_ospeed));
        Ok(())
    }
}

pub use self::ffi::{
    NCCS,
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
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    VDSUSP,
    VERASE2,
    VSTATUS,
};

#[cfg(target_os = "unix")]
pub use self::ffi::{
    VSWTC,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    VDSUSP,
    VSTATUS,
};

pub use self::ffi::{
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
};

#[cfg(target_os = "linux")]
pub use self::ffi::{
    IUCLC,
    IUTF8,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    IUTF8,
};

pub use self::ffi::{
    OCRNL,
    ONLCR,
    ONLRET,
    ONOCR,
    OPOST,
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    ONOEOT,
    TAB0,
    TAB3,
    TABDLY,
};

#[cfg(target_os = "linux")]
pub use self::ffi::{
    OFDEL,
    OFILL,
    OLCUC,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    BSDLY,
    CRDLY,
    FFDLY,
    NLDLY,
    OFDEL,
    OFILL,
    ONOEOT,
    OXTABS,
    TABDLY,
    VTDLY,
};

pub use self::ffi::{
    CLOCAL,
    CREAD,
    CRTSCTS,
    CS5,
    CS6,
    CS7,
    CS8,
    CSIZE,
    CSTOPB,
    HUPCL,
    PARENB,
    PARODD,
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    CCAR_OFLOW,
    CCTS_OFLOW,
    CDSR_OFLOW,
    CDTR_IFLOW,
    CIGNORE,
    CRTS_IFLOW,
};

#[cfg(target_os = "linux")]
pub use self::ffi::{
    CIBAUD,
    CMSPAR,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    CCAR_OFLOW,
    CCTS_OFLOW,
    CDSR_OFLOW,
    CDTR_IFLOW,
    CIGNORE,
    CRTS_IFLOW,
    MDMBUF,
};

pub use self::ffi::{
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
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    ALTWERASE,
    NOKERNINFO,
};

#[cfg(target_os = "linux")]
pub use self::ffi::{
    XCASE,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    ALTWERASE,
    NOKERNINFO,
};

pub use self::ffi::{
    TCIOFF,
    TCION,
    TCOOFF,
    TCOON,
};

pub use self::ffi::{
    TCIFLUSH,
    TCIOFLUSH,
    TCOFLUSH,
};

pub use self::ffi::{
    TCSADRAIN,
    TCSAFLUSH,
    TCSANOW,
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    TCSASOFT,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    TCSASOFT,
};

pub use self::ffi::{
    B0,
    B110,
    B115200,
    B1200,
    B134,
    B150,
    B1800,
    B19200,
    B200,
    B230400,
    B2400,
    B300,
    B38400,
    B4800,
    B50,
    B57600,
    B600,
    B75,
    B9600,
    EXTA,
    EXTB,
};

#[cfg(target_os = "freebsd")]
pub use self::ffi::{
    B14400,
    B28800,
    B460800,
    B7200,
    B76800,
    B921600,
};

#[cfg(target_os = "linux")]
pub use self::ffi::{
    B1000000,
    B1152000,
    B1500000,
    B2000000,
    B2500000,
    B3000000,
    B3500000,
    B4000000,
    B460800,
    B500000,
    B576000,
    B921600,
};

#[cfg(target_os = "macos")]
pub use self::ffi::{
    B14400,
    B28800,
    B7200,
    B76800,
};