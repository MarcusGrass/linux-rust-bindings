#[cfg(target_arch = "aarch64")]
pub mod fcntl_arm64;
#[cfg(target_arch = "aarch64")]
pub use fcntl_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod fcntl_x86;
#[cfg(target_arch = "x86_64")]
pub use fcntl_x86::*;
