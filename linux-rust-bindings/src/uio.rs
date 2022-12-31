#[cfg(target_arch = "aarch64")]
pub mod uio_arm64;
#[cfg(target_arch = "aarch64")]
pub use uio_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod uio_x86;
#[cfg(target_arch = "x86_64")]
pub use uio_x86::*;
