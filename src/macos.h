#include <stdint.h>
#include <termios.h>

// Special control characters
enum {
    __CC = (uintptr_t)(-1),
    VEOF_ = VEOF,
    VEOL_ = VEOL,
    VEOL2_ = VEOL2,
    VERASE_ = VERASE,
    VWERASE_ = VWERASE,
    VKILL_ = VKILL,
    VREPRINT_ = VREPRINT,
    VINTR_ = VINTR,
    VQUIT_ = VQUIT,
    VSUSP_ = VSUSP,
    VDSUSP_ = VDSUSP,
    VSTART_ = VSTART,
    VSTOP_ = VSTOP,
    VLNEXT_ = VLNEXT,
    VDISCARD_ = VDISCARD,
    VMIN_ = VMIN,
    VTIME_ = VTIME,
    VSTATUS_ = VSTATUS,
    NCCS_ = NCCS,
}

// Input flags
enum {
    __IFLAG = (tcflag_t)(-1),
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
    IXOFF_ = IXOFF,
    IXANY_ = IXANY,
    IMAXBEL_ = IMAXBEL,
    IUTF8_ = IUTF8,
};

// Output flags
enum {
    __OFLAG = (tcflag_t)(-1),
    OPOST_ = OPOST,
    ONLCR_ = ONLCR,
    OXTABS_ = OXTABS,
    ONOEOT_ = ONOEOT,
    // unimplemented features
    OCRNL_ = OCRNL,
    ONOCR_ = ONOCR,
    ONLRET_ = ONLRET,
    OFILL_ = OFILL,
    NLDLY_ = NLDLY,
    TABDLY_ = TABDLY,
    CRDLY_ = CRDLY,
    FFDLY_ = FFDLY,
    BSDLY_ = BSDLY,
    VTDLY_ = VTDLY,
    OFDEL_ = OFDEL,
};

// Control flags
enum {
    __CFLAG = (tcflag_t)(-1),
    CIGNORE_ = CIGNORE,
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
    CCTS_OFLOW_ = CCTS_OFLOW,
    CRTSCTS_ = CRTSCTS,
    CRTS_IFLOW_ = CRTS_IFLOW,
    CDTR_IFLOW_ = CDTR_IFLOW,
    CDSR_OFLOW_ = CDSR_OFLOW,
    CCAR_OFLOW_ = CCAR_OFLOW,
    MDMBUF_ = MDMBUF,
};

// Local flags
enum {
    __LFLAG = (tcflag_t)(-1),
    ECHOKE_ = ECHOKE,
    ECHOE_ = ECHOE,
    ECHOK_ = ECHOK,
    ECHO_ = ECHO,
    ECHONL_ = ECHONL,
    ECHOPRT_ = ECHOPRT,
    ECHOCTL_ = ECHOCTL,
    ISIG_ = ISIG,
    ICANON_ = ICANON,
    ALTWERASE_ = ALTWERASE,
    IEXTEN_ = IEXTEN,
    EXTPROC_ = EXTPROC,
    TOSTOP_ = TOSTOP,
    FLUSHO_ = FLUSHO,
    NOKERNINFO_ = NOKERNINFO,
    PENDIN_ = PENDIN,
    NOFLSH_ = NOFLSH,
};

// tcflow
enum {
    __TCFLOW = (int)(-1),
    TCOOFF_ = TCOOFF,
    TCOON_ = TCOON,
    TCIOFF_ = TCIOFF,
    TCION_ = TCION,
};

// tcflush
enum {
    __TCFLUSH = (int)(-1),
    TCIFLUSH_ = TCIFLUSH,
    TCOFLUSH_ = TCOFLUSH,
    TCIOFLUSH_ = TCIOFLUSH,
};

// tcsetattr
enum {
    __TCSETATTR = (int)(-1),
    TCSANOW_ = TCSANOW,
    TCSADRAIN_ = TCSADRAIN,
    TCSAFLUSH_ = TCSAFLUSH,
    TCSASOFT_ = TCSASOFT,
};

// Standard speeds
enum {
    __SPEED = (speed_t)(-1),
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
    B7200_ = B7200,
    B14400_ = B14400,
    B28800_ = B28800,
    B57600_ = B57600,
    B76800_ = B76800,
    B115200_ = B115200,
    B230400_ = B230400,
    EXTA_ = EXTA,
    EXTB_ = EXTB,
};
