/* automatically generated by rust-bindgen 0.63.0 */

pub const IGNBRK: i32 = 1;
pub const BRKINT: i32 = 2;
pub const IGNPAR: i32 = 4;
pub const PARMRK: i32 = 8;
pub const INPCK: i32 = 16;
pub const ISTRIP: i32 = 32;
pub const INLCR: i32 = 64;
pub const IGNCR: i32 = 128;
pub const ICRNL: i32 = 256;
pub const IXANY: i32 = 2048;
pub const IBSHIFT: i32 = 16;
pub const TCOOFF: i32 = 0;
pub const TCOON: i32 = 1;
pub const TCIOFF: i32 = 2;
pub const TCION: i32 = 3;
pub const TCIFLUSH: i32 = 0;
pub const TCOFLUSH: i32 = 1;
pub const TCIOFLUSH: i32 = 2;
pub const IUCLC: i32 = 512;
pub const IXON: i32 = 1024;
pub const IXOFF: i32 = 4096;
pub const IMAXBEL: i32 = 8192;
pub const IUTF8: i32 = 16384;
pub const PARENB: i32 = 256;
pub const PARODD: i32 = 512;
pub const ISIG: i32 = 1;
pub const ICANON: i32 = 2;
pub const IEXTEN: i32 = 32768;
pub const TCSANOW: i32 = 0;
pub const TCSADRAIN: i32 = 1;
pub const TCSAFLUSH: i32 = 2;
pub const IOC_IN: i32 = 1073741824;
pub const IOC_OUT: i64 = 2147483648;
pub const IOC_INOUT: i64 = 3221225472;
pub const IOCSIZE_MASK: i32 = 1073676288;
pub const IOCSIZE_SHIFT: i32 = 16;
pub const TCGETS: i32 = 21505;
pub const TCSETS: i32 = 21506;
pub const TCSETSW: i32 = 21507;
pub const TCSETSF: i32 = 21508;
pub const TCGETA: i32 = 21509;
pub const TCSETA: i32 = 21510;
pub const TCSETAW: i32 = 21511;
pub const TCSETAF: i32 = 21512;
pub const TCSBRK: i32 = 21513;
pub const TCXONC: i32 = 21514;
pub const TCFLSH: i32 = 21515;
pub const TIOCEXCL: i32 = 21516;
pub const TIOCNXCL: i32 = 21517;
pub const TIOCSCTTY: i32 = 21518;
pub const TIOCGPGRP: i32 = 21519;
pub const TIOCSPGRP: i32 = 21520;
pub const TIOCOUTQ: i32 = 21521;
pub const TIOCSTI: i32 = 21522;
pub const TIOCGWINSZ: i32 = 21523;
pub const TIOCSWINSZ: i32 = 21524;
pub const TIOCMGET: i32 = 21525;
pub const TIOCMBIS: i32 = 21526;
pub const TIOCMBIC: i32 = 21527;
pub const TIOCMSET: i32 = 21528;
pub const TIOCGSOFTCAR: i32 = 21529;
pub const TIOCSSOFTCAR: i32 = 21530;
pub const TIOCINQ: i32 = 21531;
pub const TIOCLINUX: i32 = 21532;
pub const TIOCCONS: i32 = 21533;
pub const TIOCGSERIAL: i32 = 21534;
pub const TIOCSSERIAL: i32 = 21535;
pub const TIOCPKT: i32 = 21536;
pub const TIOCNOTTY: i32 = 21538;
pub const TIOCSETD: i32 = 21539;
pub const TIOCGETD: i32 = 21540;
pub const TCSBRKP: i32 = 21541;
pub const TIOCSBRK: i32 = 21543;
pub const TIOCCBRK: i32 = 21544;
pub const TIOCGSID: i32 = 21545;
pub const TIOCGRS485: i32 = 21550;
pub const TIOCSRS485: i32 = 21551;
pub const TCGETX: i32 = 21554;
pub const TCSETX: i32 = 21555;
pub const TCSETXF: i32 = 21556;
pub const TCSETXW: i32 = 21557;
pub const TIOCVHANGUP: i32 = 21559;
pub const TIOCSERCONFIG: i32 = 21587;
pub const TIOCSERGWILD: i32 = 21588;
pub const TIOCSERSWILD: i32 = 21589;
pub const TIOCGLCKTRMIOS: i32 = 21590;
pub const TIOCSLCKTRMIOS: i32 = 21591;
pub const TIOCSERGSTRUCT: i32 = 21592;
pub const TIOCSERGETLSR: i32 = 21593;
pub const TIOCSERGETMULTI: i32 = 21594;
pub const TIOCSERSETMULTI: i32 = 21595;
pub const TIOCMIWAIT: i32 = 21596;
pub const TIOCGICOUNT: i32 = 21597;
pub const TIOCPKT_DATA: i32 = 0;
pub const TIOCPKT_FLUSHREAD: i32 = 1;
pub const TIOCPKT_FLUSHWRITE: i32 = 2;
pub const TIOCPKT_STOP: i32 = 4;
pub const TIOCPKT_START: i32 = 8;
pub const TIOCPKT_NOSTOP: i32 = 16;
pub const TIOCPKT_DOSTOP: i32 = 32;
pub const TIOCPKT_IOCTL: i32 = 64;
pub const TIOCSER_TEMT: i32 = 1;
pub const TIOCM_LE: i32 = 1;
pub const TIOCM_DTR: i32 = 2;
pub const TIOCM_RTS: i32 = 4;
pub const TIOCM_ST: i32 = 8;
pub const TIOCM_SR: i32 = 16;
pub const TIOCM_CTS: i32 = 32;
pub const TIOCM_CAR: i32 = 64;
pub const TIOCM_RNG: i32 = 128;
pub const TIOCM_DSR: i32 = 256;
pub const TIOCM_CD: i32 = 64;
pub const TIOCM_RI: i32 = 128;
pub const TIOCM_OUT1: i32 = 8192;
pub const TIOCM_OUT2: i32 = 16384;
pub const TIOCM_LOOP: i32 = 32768;
pub type cc_t = ::core::ffi::c_uchar;
pub type speed_t = ::core::ffi::c_uint;
pub type tcflag_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct termios2 {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19usize],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct winsize {
    pub ws_row: ::core::ffi::c_ushort,
    pub ws_col: ::core::ffi::c_ushort,
    pub ws_xpixel: ::core::ffi::c_ushort,
    pub ws_ypixel: ::core::ffi::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct termio {
    pub c_iflag: ::core::ffi::c_ushort,
    pub c_oflag: ::core::ffi::c_ushort,
    pub c_cflag: ::core::ffi::c_ushort,
    pub c_lflag: ::core::ffi::c_ushort,
    pub c_line: ::core::ffi::c_uchar,
    pub c_cc: [::core::ffi::c_uchar; 8usize],
}
