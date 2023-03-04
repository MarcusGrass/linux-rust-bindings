/* automatically generated by rust-bindgen 0.64.0 */

pub const EPOLL_CLOEXEC: i32 = 524288;
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type __poll_t = ::core::ffi::c_uint;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct epoll_event {
    pub events: __poll_t,
    pub data: __u64,
}
