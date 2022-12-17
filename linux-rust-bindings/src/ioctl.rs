#[cfg(target_arch = "aarch64")]
pub mod ioctl_arm64;
#[cfg(target_arch = "aarch64")]
pub use ioctl_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod ioctl_x86;
#[cfg(target_arch = "x86_64")]
pub use ioctl_x86::*;
