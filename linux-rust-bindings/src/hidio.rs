#[cfg(target_arch = "aarch64")]
pub mod hidio_arm64;
#[cfg(target_arch = "aarch64")]
pub use hidio_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod hidio_x86;
#[cfg(target_arch = "x86_64")]
pub use hidio_x86::*;
