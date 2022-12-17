#[cfg(target_arch = "aarch64")]
pub mod aux_arm64;
#[cfg(target_arch = "aarch64")]
pub use aux_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod aux_x86;
#[cfg(target_arch = "x86_64")]
pub use aux_x86::*;
