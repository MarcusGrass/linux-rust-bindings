#[cfg(target_arch = "aarch64")]
pub mod io_uring_arm64;
#[cfg(target_arch = "aarch64")]
pub use io_uring_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod io_uring_x86;
#[cfg(target_arch = "x86_64")]
pub use io_uring_x86::*;
