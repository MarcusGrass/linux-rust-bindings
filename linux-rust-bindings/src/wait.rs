#[cfg(target_arch = "aarch64")]
pub mod wait_arm64;
#[cfg(target_arch = "aarch64")]
pub use wait_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod wait_x86;
#[cfg(target_arch = "x86_64")]
pub use wait_x86::*;
