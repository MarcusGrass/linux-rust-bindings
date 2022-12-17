#[cfg(target_arch = "aarch64")]
pub mod stat_arm64;
#[cfg(target_arch = "aarch64")]
pub use stat_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod stat_x86;
#[cfg(target_arch = "x86_64")]
pub use stat_x86::*;
