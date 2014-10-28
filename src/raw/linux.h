#include <termios.h>

// Special control characters
enum {
    VINTR_ = VINTR,
    VQUIT_ = VQUIT,
    VERASE_ = VERASE,
    VKILL_ = VKILL,
    VEOF_ = VEOF,
    VTIME_ = VTIME,
    VMIN_ = VMIN,
    VSWTC_ = VSWTC,
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
};

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
    IUCLC_ = IUCLC,
    IXON_ = IXON,
    IXANY_ = IXANY,
    IXOFF_ = IXOFF,
    IMAXBEL_ = IMAXBEL,
    IUTF8_ = IUTF8,
};

// Output flags
enum {
    __OFLAG = (tcflag_t)(-1),
    OPOST_ = OPOST,
    OLCUC_ = OLCUC,
    ONLCR_ = ONLCR,
    OCRNL_ = OCRNL,
    ONOCR_ = ONOCR,
    ONLRET_ = ONLRET,
    OFILL_ = OFILL,
    OFDEL_ = OFDEL,
};

// Control flags
enum {
    __CFLAG = (tcflag_t)(-1),
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
    CIBAUD_ = CIBAUD,
    CMSPAR_ = CMSPAR,
    CRTSCTS_ = CRTSCTS,
};

// Local flags
enum {
    __LFLAG = (tcflag_t)(-1),
    ISIG_ = ISIG,
    ICANON_ = ICANON,
    XCASE_ = XCASE,
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
    EXTA_ = EXTA,
    EXTB_ = EXTB,
    B57600_ = B57600,
    B115200_ = B115200,
    B230400_ = B230400,
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
};
