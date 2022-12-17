#[cfg(target_arch = "aarch64")]
pub mod types_arm64;
#[cfg(target_arch = "aarch64")]
pub use types_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod types_x86;
#[cfg(target_arch = "x86_64")]
pub use types_x86::*;
