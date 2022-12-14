#[cfg(target_arch = "aarch64")]
pub mod nolibc_arm64;
#[cfg(target_arch = "aarch64")]
pub use nolibc_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod nolibc_x86;
#[cfg(target_arch = "x86_64")]
pub use nolibc_x86::*;
