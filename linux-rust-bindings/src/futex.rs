#[cfg(target_arch = "aarch64")]
pub mod futex_arm64;
#[cfg(target_arch = "aarch64")]
pub use futex_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod futex_x86;
#[cfg(target_arch = "x86_64")]
pub use futex_x86::*;
