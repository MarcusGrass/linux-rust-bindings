#[cfg(target_arch = "aarch64")]
pub mod errno_arm64;
#[cfg(target_arch = "aarch64")]
pub use errno_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod errno_x86;
#[cfg(target_arch = "x86_64")]
pub use errno_x86::*;
