use libc::{ c_uchar, c_uint, c_int };
use std::mem;

pub type cc_t = c_uchar;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

pub const NCCS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; NCCS],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
impl Default for termios {
    fn default() -> termios { unsafe { mem::zeroed() } }
}

// c_cc characters
pub const VINTR: c_uchar = 0;
pub const VQUIT: c_uchar = 1;
pub const VERASE: c_uchar = 2;
pub const VKILL: c_uchar = 3;
pub const VEOF: c_uchar = 4;
pub const VTIME: c_uchar = 5;
pub const VMIN: c_uchar = 6;
pub const VSWTC: c_uchar = 7;
pub const VSTART: c_uchar = 8;
pub const VSTOP: c_uchar = 9;
pub const VSUSP: c_uchar = 10;
pub const VEOL: c_uchar = 11;
pub const VREPRINT: c_uchar = 12;
pub const VDISCARD: c_uchar = 13;
pub const VWERASE: c_uchar = 14;
pub const VLNEXT: c_uchar = 15;
pub const VEOL2: c_uchar = 16;

// c_iflag bits
pub const IGNBRK: tcflag_t = 0o000001;
pub const BRKINT: tcflag_t = 0o000002;
pub const IGNPAR: tcflag_t = 0o000004;
pub const PARMRK: tcflag_t = 0o000010;
pub const INPCK: tcflag_t = 0o000020;
pub const ISTRIP: tcflag_t = 0o000040;
pub const INLCR: tcflag_t = 0o000100;
pub const IGNCR: tcflag_t = 0o000200;
pub const ICRNL: tcflag_t = 0o000400;
pub const IUCLC: tcflag_t = 0o001000;
pub const IXON: tcflag_t = 0o002000;
pub const IXANY: tcflag_t = 0o004000;
pub const IXOFF: tcflag_t = 0o010000;
pub const IMAXBEL: tcflag_t = 0o020000;
pub const IUTF8: tcflag_t = 0o040000;

// c_oflag bits
pub const OPOST: tcflag_t  = 0o000001;
pub const OLCUC: tcflag_t  = 0o000002;
pub const ONLCR: tcflag_t  = 0o000004;
pub const OCRNL: tcflag_t  = 0o000010;
pub const ONOCR: tcflag_t  = 0o000020;
pub const ONLRET: tcflag_t  = 0o000040;
pub const OFILL: tcflag_t  = 0o000100;
pub const OFDEL: tcflag_t  = 0o000200;
pub const NLDLY: tcflag_t  = 0o000400;
pub const NL0: tcflag_t  = 0o000000;
pub const NL1: tcflag_t  = 0o000400;
pub const CRDLY: tcflag_t  = 0o003000;
pub const CR0: tcflag_t  = 0o000000;
pub const CR1: tcflag_t  = 0o001000;
pub const CR2: tcflag_t  = 0o002000;
pub const CR3: tcflag_t  = 0o003000;
pub const TABDLY: tcflag_t  = 0o014000;
pub const TAB0: tcflag_t  = 0o000000;
pub const TAB1: tcflag_t  = 0o004000;
pub const TAB2: tcflag_t  = 0o010000;
pub const TAB3: tcflag_t  = 0o014000;
pub const BSDLY: tcflag_t  = 0o020000;
pub const BS0: tcflag_t  = 0o000000;
pub const BS1: tcflag_t  = 0o020000;
pub const FFDLY: tcflag_t  = 0o100000;
pub const FF0: tcflag_t  = 0o000000;
pub const FF1: tcflag_t  = 0o100000;

pub const VTDLY: tcflag_t = 0o040000;
pub const VT0: tcflag_t = 0o000000;
pub const VT1: tcflag_t = 0o040000;

pub const XTABS: tcflag_t = 0o014000;

pub const CBAUD: tcflag_t = 0o010017;

/// hang up
pub const B0: tcflag_t = 0o000000;
pub const B50: tcflag_t = 0o000001;
pub const B75: tcflag_t = 0o000002;
pub const B110: tcflag_t = 0o000003;
pub const B134: tcflag_t = 0o000004;
pub const B150: tcflag_t = 0o000005;
pub const B200: tcflag_t = 0o000006;
pub const B300: tcflag_t = 0o000007;
pub const B600: tcflag_t = 0o000010;
pub const B1200: tcflag_t = 0o000011;
pub const B1800: tcflag_t = 0o000012;
pub const B2400: tcflag_t = 0o000013;
pub const B4800: tcflag_t = 0o000014;
pub const B9600: tcflag_t = 0o000015;
pub const B19200: tcflag_t = 0o000016;
pub const B38400: tcflag_t = 0o000017;
pub const EXTA: tcflag_t = B19200;
pub const EXTB: tcflag_t = B38400;
pub const CSIZE: tcflag_t = 0o000060;
pub const CS5: tcflag_t = 0o000000;
pub const CS6: tcflag_t = 0o000020;
pub const CS7: tcflag_t = 0o000040;
pub const CS8: tcflag_t = 0o000060;
pub const CSTOPB: tcflag_t = 0o000100;
pub const CREAD: tcflag_t = 0o000200;
pub const PARENB: tcflag_t = 0o000400;
pub const PARODD: tcflag_t = 0o001000;
pub const HUPCL: tcflag_t = 0o002000;
pub const CLOCAL: tcflag_t = 0o004000;
pub const CBAUDEX: tcflag_t = 0o010000;
pub const B57600: tcflag_t = 0o010001;
pub const B115200: tcflag_t = 0o010002;
pub const B230400: tcflag_t = 0o010003;
pub const B460800: tcflag_t = 0o010004;
pub const B500000: tcflag_t = 0o010005;
pub const B576000: tcflag_t = 0o010006;
pub const B921600: tcflag_t = 0o010007;
pub const B1000000: tcflag_t = 0o010010;
pub const B1152000: tcflag_t = 0o010011;
pub const B1500000: tcflag_t = 0o010012;
pub const B2000000: tcflag_t = 0o010013;
pub const B2500000: tcflag_t = 0o010014;
pub const B3000000: tcflag_t = 0o010015;
pub const B3500000: tcflag_t = 0o010016;
pub const B4000000: tcflag_t = 0o010017;
pub const __MAX_BAUD: tcflag_t = B4000000;
/// input baud rate (not used)
pub const CIBAUD: tcflag_t = 0o02003600000;
/// mark or space (stick) parity
pub const CMSPAR: tcflag_t = 0o10000000000;
/// flow control
pub const CRTSCTS: tcflag_t = 0o20000000000;

// c_lflag bits
pub const ISIG: tcflag_t = 0o000001;
pub const ICANON: tcflag_t = 0o000002;
pub const XCASE: tcflag_t = 0o000004;
pub const ECHO: tcflag_t = 0o000010;
pub const ECHOE: tcflag_t = 0o000020;
pub const ECHOK: tcflag_t = 0o000040;
pub const ECHONL: tcflag_t = 0o000100;
pub const NOFLSH: tcflag_t = 0o000200;
pub const TOSTOP: tcflag_t = 0o000400;
pub const ECHOCTL: tcflag_t = 0o001000;
pub const ECHOPRT: tcflag_t = 0o002000;
pub const ECHOKE: tcflag_t = 0o004000;
pub const FLUSHO: tcflag_t = 0o010000;
pub const PENDIN: tcflag_t = 0o040000;
pub const IEXTEN: tcflag_t = 0o100000;
pub const EXTPROC: tcflag_t = 0o200000;

// tcflow() and TCXONC use these
pub const TCOOFF: c_int = 0;
pub const TCOON: c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION: c_int = 3;

// tcflush() and TCFLSH use these
pub const TCIFLUSH: c_int = 0;
pub const TCOFLUSH: c_int = 1;
pub const TCIOFLUSH: c_int = 2;

// tcsetattr uses these
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

extern {
    // Return the output baud rate stored in *TERMIOS_P.
    pub fn cfgetospeed(__termios_p: *const termios) -> speed_t;

    // Return the input baud rate stored in *TERMIOS_P.
    pub fn cfgetispeed(__termios_p: *const termios) -> speed_t;

    // Set the output baud rate stored in *TERMIOS_P to SPEED.
    pub fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> c_int;

    // Set the input baud rate stored in *TERMIOS_P to SPEED.
    pub fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> c_int;

    // Set both the input and output baud rates in *TERMIOS_OP to SPEED.
    pub fn cfsetspeed(__termios_p: *mut termios, __speed: speed_t) -> c_int;

    // Put the state of FD into *TERMIOS_P.
    pub fn tcgetattr(__fd: c_int, __termios_p: *mut termios) -> c_int;

    // Set the state of FD to *TERMIOS_P.
    pub fn tcsetattr(__fd: c_int, __optional_actions: c_int, __termios_p: *const termios) -> c_int;

    // Set *TERMIOS_P to indicate raw mode.
    pub fn cfmakeraw(__termios_p: *mut termios);

    // Send zero bits on FD.
    pub fn tcsendbreak(__fd: c_int, __duration: c_int) -> c_int;

    // Wait for pending output to be written on FD.
    pub fn tcdrain(__fd: c_int) -> c_int;

    // Flush pending data on FD.
    pub fn tcflush(__fd: c_int, __queue_selector: c_int) -> c_int;

    // Suspend or restart transmission on FD.
    pub fn tcflow(__fd: c_int, __action: c_int) -> c_int;
}