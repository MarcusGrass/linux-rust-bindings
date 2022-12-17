#[cfg(target_arch = "aarch64")]
pub mod mman_arm64;
#[cfg(target_arch = "aarch64")]
pub use mman_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod mman_x86;
#[cfg(target_arch = "x86_64")]
pub use mman_x86::*;
