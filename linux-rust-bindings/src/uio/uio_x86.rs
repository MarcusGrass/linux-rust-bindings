/* automatically generated by rust-bindgen 0.64.0 */

pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_size_t = __kernel_ulong_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: __kernel_size_t,
}
