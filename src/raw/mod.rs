//! Raw access to the `Termios` structure and its flags

#[allow(dead_code, missing_docs, non_camel_case_types)]
mod ffi;

use std::fmt;

pub use self::ffi::cc_t;
pub use self::ffi::speed_t;
pub use self::ffi::tcflag_t;

pub use self::ffi::Struct_termios as Termios;

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

#[cfg(target_os = "linux")]
pub use self::ffi::{
    tcgetsid,
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

// FIXME (concat_idents!) All these macros should use only one input

macro_rules! cc {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

cc! {
    NCCS_ = NCCS,
    VDISCARD_ = VDISCARD,
    VEOF_ = VEOF,
    VEOL2_ = VEOL2,
    VEOL_ = VEOL,
    VERASE_ = VERASE,
    VINTR_ = VINTR,
    VKILL_ = VKILL,
    VLNEXT_ = VLNEXT,
    VMIN_ = VMIN,
    VQUIT_ = VQUIT,
    VREPRINT_ = VREPRINT,
    VSTART_ = VSTART,
    VSTOP_ = VSTOP,
    VSUSP_ = VSUSP,
    VTIME_ = VTIME,
    VWERASE_ = VWERASE,
}

#[cfg(target_os = "freebsd")]
cc! {
    VDSUSP_ = VDSUSP,
    VERASE2_ = VERASE2,
    VSTATUS_ = VSTATUS,
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
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

iflag! {
    BRKINT_ = BRKINT,
    ICRNL_ = ICRNL,
    IGNBRK_ = IGNBRK,
    IGNCR_ = IGNCR,
    IGNPAR_ = IGNPAR,
    IMAXBEL_ = IMAXBEL,
    INLCR_ = INLCR,
    INPCK_ = INPCK,
    ISTRIP_ = ISTRIP,
    IXANY_ = IXANY,
    IXOFF_ = IXOFF,
    IXON_ = IXON,
    PARMRK_ = PARMRK,
}

#[cfg(target_os = "linux")]
iflag! {
    IUCLC_ = IUCLC,
    IUTF8_ = IUTF8,
}

#[cfg(target_os = "macos")]
iflag! {
    IUTF8_ = IUTF8,
}

macro_rules! oflag {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

oflag! {
    OCRNL_ = OCRNL,
    ONLCR_ = ONLCR,
    ONLRET_ = ONLRET,
    ONOCR_ = ONOCR,
    OPOST_ = OPOST,
}

#[cfg(target_os = "freebsd")]
oflag! {
    ONOEOT_ = ONOEOT,
    TAB0_ = TAB0,
    TAB3_ = TAB3,
    TABDLY_ = TABDLY,
}

#[cfg(target_os = "linux")]
oflag! {
    OFDEL_ = OFDEL,
    OFILL_ = OFILL,
    OLCUC_ = OLCUC,
}

#[cfg(target_os = "macos")]
oflag! {
    BSDLY_ = BSDLY,
    CRDLY_ = CRDLY,
    FFDLY_ = FFDLY,
    NLDLY_ = NLDLY,
    OFDEL_ = OFDEL,
    OFILL_ = OFILL,
    ONOEOT_ = ONOEOT,
    OXTABS_ = OXTABS,
    TABDLY_ = TABDLY,
    VTDLY_ = VTDLY,
}

macro_rules! cflag {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

cflag! {
    CLOCAL_ = CLOCAL,
    CREAD_ = CREAD,
    CRTSCTS_ = CRTSCTS,
    CS5_ = CS5,
    CS6_ = CS6,
    CS7_ = CS7,
    CS8_ = CS8,
    CSIZE_ = CSIZE,
    CSTOPB_ = CSTOPB,
    HUPCL_ = HUPCL,
    PARENB_ = PARENB,
    PARODD_ = PARODD,
}

#[cfg(target_os = "freebsd")]
cflag! {
    CCAR_OFLOW_ = CCAR_OFLOW,
    CCTS_OFLOW_ = CCTS_OFLOW,
    CDSR_OFLOW_ = CDSR_OFLOW,
    CDTR_IFLOW_ = CDTR_IFLOW,
    CIGNORE_ = CIGNORE,
    CRTS_IFLOW_ = CRTS_IFLOW,
}

#[cfg(target_os = "linux")]
cflag! {
    CIBAUD_ = CIBAUD,
    CMSPAR_ = CMSPAR,
}

#[cfg(target_os = "macos")]
cflag! {
    CCAR_OFLOW_ = CCAR_OFLOW,
    CCTS_OFLOW_ = CCTS_OFLOW,
    CDSR_OFLOW_ = CDSR_OFLOW,
    CDTR_IFLOW_ = CDTR_IFLOW,
    CIGNORE_ = CIGNORE,
    CRTS_IFLOW_ = CRTS_IFLOW,
    MDMBUF_ = MDMBUF,
}

macro_rules! lflag {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

lflag! {
    ECHOCTL_ = ECHOCTL,
    ECHOE_ = ECHOE,
    ECHOKE_ = ECHOKE,
    ECHOK_ = ECHOK,
    ECHONL_ = ECHONL,
    ECHOPRT_ = ECHOPRT,
    ECHO_ = ECHO,
    EXTPROC_ = EXTPROC,
    FLUSHO_ = FLUSHO,
    ICANON_ = ICANON,
    IEXTEN_ = IEXTEN,
    ISIG_ = ISIG,
    NOFLSH_ = NOFLSH,
    PENDIN_ = PENDIN,
    TOSTOP_ = TOSTOP,
}

#[cfg(target_os = "freebsd")]
lflag! {
    ALTWERASE_ = ALTWERASE,
    NOKERNINFO_ = NOKERNINFO,
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
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

tcflow! {
    TCIOFF_ = TCIOFF,
    TCION_ = TCION,
    TCOOFF_ = TCOOFF,
    TCOON_ = TCOON,
}

macro_rules! tcflush {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

tcflush! {
    TCIFLUSH_ = TCIFLUSH,
    TCIOFLUSH_ = TCIOFLUSH,
    TCOFLUSH_ = TCOFLUSH,
}

macro_rules! tcsetattr {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

tcsetattr! {
    TCSADRAIN_ = TCSADRAIN,
    TCSAFLUSH_ = TCSAFLUSH,
    TCSANOW_ = TCSANOW,
}

#[cfg(target_os = "freebsd")]
tcsetattr! {
    TCSASOFT_ = TCSASOFT,
}

#[cfg(target_os = "macos")]
tcsetattr! {
    TCSASOFT_ = TCSASOFT,
}

macro_rules! baud {
    ($($ffi:ident = $ident:ident,)+) => {
        $(
            pub use self::ffi::$ffi as $ident;
        )+
    }
}

baud! {
    B0_ = B0,
    B110_ = B110,
    B115200_ = B115200,
    B1200_ = B1200,
    B134_ = B134,
    B150_ = B150,
    B1800_ = B1800,
    B19200_ = B19200,
    B200_ = B200,
    B230400_ = B230400,
    B2400_ = B2400,
    B300_ = B300,
    B38400_ = B38400,
    B4800_ = B4800,
    B50_ = B50,
    B57600_ = B57600,
    B600_ = B600,
    B75_ = B75,
    B9600_ = B9600,
    EXTA_ = EXTA,
    EXTB_ = EXTB,
}

#[cfg(target_os = "freebsd")]
baud! {
    B14400_ = B14400,
    B28800_ = B28800,
    B460800_ = B460800,
    B7200_ = B7200,
    B76800_ = B76800,
    B921600_ = B921600,
}

#[cfg(target_os = "linux")]
baud! {
    B1000000_ = B1000000,
    B1152000_ = B1152000,
    B1500000_ = B1500000,
    B2000000_ = B2000000,
    B2500000_ = B2500000,
    B3000000_ = B3000000,
    B3500000_ = B3500000,
    B4000000_ = B4000000,
    B460800_ = B460800,
    B500000_ = B500000,
    B576000_ = B576000,
    B921600_ = B921600,
}

#[cfg(target_os = "macos")]
baud! {
    B14400_ = B14400,
    B28800_ = B28800,
    B7200_ = B7200,
    B76800_ = B76800,
}
