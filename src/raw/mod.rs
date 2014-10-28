//! Raw access to the `Termios` structure and its flags

use std::default::Default;
use std::fmt;

pub use self::ffi::cc_t;
pub use self::ffi::speed_t;
pub use self::ffi::tcflag_t;

pub use self::ffi::Struct_termios as Termios;

pub use self::ffi::{
    cfgetospeed,
    cfgetispeed,
    cfsetospeed,
    cfsetispeed,
    cfsetspeed,
    tcgetattr,
    tcsetattr,
    cfmakeraw,
    tcsendbreak,
    tcdrain,
    tcflush,
    tcflow,
    tcgetsid,
};

impl Default for Termios {
    #[cfg(target_os = "linux")]
    fn default() -> Termios {
        Termios {
            c_cc: [0, ..NCCS],
            c_cflag: 0,
            c_iflag: 0,
            c_ispeed: 0,
            c_lflag: 0,
            c_line: 0,
            c_oflag: 0,
            c_ospeed: 0,
        }
    }

    #[cfg(target_os = "macos")]
    fn default() -> Termios {
        Termios {
            c_cc: [0, ..NCCS],
            c_cflag: 0,
            c_iflag: 0,
            c_ispeed: 0,
            c_lflag: 0,
            c_oflag: 0,
            c_ospeed: 0,
        }
    }
}

// XXX (Show) Formatting may change
impl fmt::Show for Termios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "iflag:\t{}", self.c_iflag));
        try!(writeln!(f, "oflag:\t{}", self.c_oflag));
        try!(writeln!(f, "cflag:\t{}", self.c_cflag));
        try!(writeln!(f, "lflag:\t{}", self.c_lflag));
        try!(writeln!(f, "cc:\t{}", self.c_cc[]));
        try!(writeln!(f, "ispeed:\t{}", self.c_ispeed));
        try!(write!(f, "ospeed:\t{}", self.c_ospeed));
        Ok(())
    }
}

#[allow(dead_code, non_camel_case_types)]
mod ffi;

// FIXME (concat_idents!) All these macros should use only one input

macro_rules! cc {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: uint = self::ffi::$ffi as uint;)+
    }
}

cc! {
    VINTR_ = VINTR,
    VQUIT_ = VQUIT,
    VERASE_ = VERASE,
    VKILL_ = VKILL,
    VEOF_ = VEOF,
    VTIME_ = VTIME,
    VMIN_ = VMIN,
    VSTART_ = VSTART,
    VSTOP_ = VSTOP,
    VSUSP_ = VSUSP,
    VEOL_ = VEOL,
    VREPRINT_ = VREPRINT,
    VDISCARD_ = VDISCARD,
    VWERASE_ = VWERASE,
    VLNEXT_ = VLNEXT,
    VEOL2_ = VEOL2,
    NCCS_ = NCCS,
}

#[cfg(target_os = "linux")]
cc! {
  VSWTC_ = VSWTC,
}

#[cfg(target_os = "macos")]
cc! {
  VDSUSP_ = VDSUSP,
  VSTATUS_ = VSTATUS,
}

macro_rules! iflag {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: tcflag_t = self::ffi::$ffi;)+
    }
}

iflag! {
    IGNBRK_ = IGNBRK,
    BRKINT_ = BRKINT,
    IGNPAR_ = IGNPAR,
    PARMRK_ = PARMRK,
    INPCK_ = INPCK,
    ISTRIP_ = ISTRIP,
    INLCR_ = INLCR,
    IGNCR_ = IGNCR,
    ICRNL_ = ICRNL,
    IXON_ = IXON,
    IXANY_ = IXANY,
    IXOFF_ = IXOFF,
    IMAXBEL_ = IMAXBEL,
    IUTF8_ = IUTF8,
}

#[cfg(target_os = "linux")]
iflag! {
    IUCLC_ = IUCLC,
}

macro_rules! oflag {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: tcflag_t = self::ffi::$ffi;)+
    }
}

oflag! {
    OPOST_ = OPOST,
    ONLCR_ = ONLCR,
    OCRNL_ = OCRNL,
    ONOCR_ = ONOCR,
    ONLRET_ = ONLRET,
    OFILL_ = OFILL,
    OFDEL_ = OFDEL,
}

#[cfg(target_os = "linux")]
oflag! {
    OLCUC_ = OLCUC,
}

#[cfg(target_os = "macos")]
oflag! {
  OXTABS_ = OXTABS,
  ONOEOT_ = ONOEOT,
  NLDLY_ = NLDLY,
  TABDLY_ = TABDLY,
  CRDLY_ = CRDLY,
  FFDLY_ = FFDLY,
  BSDLY_ = BSDLY,
  VTDLY_ = VTDLY,
}

macro_rules! cflag {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: tcflag_t = self::ffi::$ffi;)+
    }
}

cflag! {
    CSIZE_ = CSIZE,
    CS5_ = CS5,
    CS6_ = CS6,
    CS7_ = CS7,
    CS8_ = CS8,
    CSTOPB_ = CSTOPB,
    CREAD_ = CREAD,
    PARENB_ = PARENB,
    PARODD_ = PARODD,
    HUPCL_ = HUPCL,
    CLOCAL_ = CLOCAL,
    CRTSCTS_ = CRTSCTS,
}

#[cfg(target_os = "linux")]
cflag! {
    CIBAUD_ = CIBAUD,
    CMSPAR_ = CMSPAR,
}

#[cfg(target_os = "macos")]
cflag! {
    CIGNORE_ = CIGNORE,
    CCTS_OFLOW_ = CCTS_OFLOW,
    CRTS_IFLOW_ = CRTS_IFLOW,
    CDTR_IFLOW_ = CDTR_IFLOW,
    CDSR_OFLOW_ = CDSR_OFLOW,
    CCAR_OFLOW_ = CCAR_OFLOW,
    MDMBUF_ = MDMBUF,
}

macro_rules! lflag {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: tcflag_t = self::ffi::$ffi;)+
    }
}

lflag! {
    ISIG_ = ISIG,
    ICANON_ = ICANON,
    ECHO_ = ECHO,
    ECHOE_ = ECHOE,
    ECHOK_ = ECHOK,
    ECHONL_ = ECHONL,
    NOFLSH_ = NOFLSH,
    TOSTOP_ = TOSTOP,
    ECHOCTL_ = ECHOCTL,
    ECHOPRT_ = ECHOPRT,
    ECHOKE_ = ECHOKE,
    FLUSHO_ = FLUSHO,
    PENDIN_ = PENDIN,
    IEXTEN_ = IEXTEN,
    EXTPROC_ = EXTPROC,
}

#[cfg(target_os = "linux")]
lflag! {
    XCASE_ = XCASE,
}

#[cfg(target_os = "macos")]
lflag! {
    ALTWERASE_ = ALTWERASE,
    NOKERNINFO_ = NOKERNINFO,
}

macro_rules! tcflow {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: ::libc::c_int = self::ffi::$ffi;)+
    }
}

tcflow! {
    TCOOFF_ = TCOOFF,
    TCOON_ = TCOON,
    TCIOFF_ = TCIOFF,
    TCION_ = TCION,
}

macro_rules! tcflush {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: ::libc::c_int = self::ffi::$ffi;)+
    }
}

tcflush! {
    TCIFLUSH_ = TCIFLUSH,
    TCOFLUSH_ = TCOFLUSH,
    TCIOFLUSH_ = TCIOFLUSH,
}

macro_rules! tcsetattr {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: ::libc::c_int = self::ffi::$ffi;)+
    }
}

tcsetattr! {
    TCSANOW_ = TCSANOW,
    TCSADRAIN_ = TCSADRAIN,
    TCSAFLUSH_ = TCSAFLUSH,
}

#[cfg(target_os = "macos")]
tcsetattr! {
    TCSASOFT_ = TCSASOFT,
}

macro_rules! baud {
    ($($ffi:ident = $ident:ident,)+) => {$(
        pub const $ident: speed_t = self::ffi::$ffi;)+
    }
}

baud! {
    B0_ = B0,
    B50_ = B50,
    B75_ = B75,
    B110_ = B110,
    B134_ = B134,
    B150_ = B150,
    B200_ = B200,
    B300_ = B300,
    B600_ = B600,
    B1200_ = B1200,
    B1800_ = B1800,
    B2400_ = B2400,
    B4800_ = B4800,
    B9600_ = B9600,
    B19200_ = B19200,
    B38400_ = B38400,
    EXTA_ = EXTA,
    EXTB_ = EXTB,
    B57600_ = B57600,
    B115200_ = B115200,
    B230400_ = B230400,
}

#[cfg(target_os = "linux")]
baud! {
    B460800_ = B460800,
    B500000_ = B500000,
    B576000_ = B576000,
    B921600_ = B921600,
    B1000000_ = B1000000,
    B1152000_ = B1152000,
    B1500000_ = B1500000,
    B2000000_ = B2000000,
    B2500000_ = B2500000,
    B3000000_ = B3000000,
    B3500000_ = B3500000,
    B4000000_ = B4000000,
}

#[cfg(target_os = "macos")]
baud! {
    B7200_ = B7200,
    B14400_ = B14400,
    B28800_ = B28800,
    B76800_ = B76800,
}
