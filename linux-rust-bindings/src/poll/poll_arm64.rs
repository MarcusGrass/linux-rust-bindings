/* automatically generated by rust-bindgen 0.65.1 */

pub const POLLIN: i32 = 1;
pub const POLLPRI: i32 = 2;
pub const POLLOUT: i32 = 4;
pub const POLLERR: i32 = 8;
pub const POLLHUP: i32 = 16;
pub const POLLNVAL: i32 = 32;
pub const POLLRDNORM: i32 = 64;
pub const POLLRDBAND: i32 = 128;
pub const POLLWRNORM: i32 = 256;
pub const POLLWRBAND: i32 = 512;
pub const POLLMSG: i32 = 1024;
pub const POLLREMOVE: i32 = 4096;
pub const POLLRDHUP: i32 = 8192;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
