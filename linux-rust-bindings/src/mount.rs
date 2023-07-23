#[cfg(target_arch = "aarch64")]
pub mod mount_arm64;
#[cfg(target_arch = "aarch64")]
pub use mount_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod mount_x86;
#[cfg(target_arch = "x86_64")]
pub use mount_x86::*;
