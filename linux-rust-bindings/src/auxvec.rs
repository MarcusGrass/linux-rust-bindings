#[cfg(target_arch = "aarch64")]
pub mod auxvec_arm64;
#[cfg(target_arch = "aarch64")]
pub use auxvec_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod auxvec_x86;
#[cfg(target_arch = "x86_64")]
pub use auxvec_x86::*;
