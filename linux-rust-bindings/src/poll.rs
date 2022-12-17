#[cfg(target_arch = "aarch64")]
pub mod poll_arm64;
#[cfg(target_arch = "aarch64")]
pub use poll_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod poll_x86;
#[cfg(target_arch = "x86_64")]
pub use poll_x86::*;
