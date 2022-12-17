#[cfg(target_arch = "aarch64")]
pub mod time_arm64;
#[cfg(target_arch = "aarch64")]
pub use time_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod time_x86;
#[cfg(target_arch = "x86_64")]
pub use time_x86::*;
