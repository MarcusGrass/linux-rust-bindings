#[cfg(target_arch = "aarch64")]
pub mod fs_arm64;
#[cfg(target_arch = "aarch64")]
pub use fs_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod fs_x86;
#[cfg(target_arch = "x86_64")]
pub use fs_x86::*;
