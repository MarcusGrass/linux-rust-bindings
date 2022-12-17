#[cfg(target_arch = "aarch64")]
pub mod signal_arm64;
#[cfg(target_arch = "aarch64")]
pub use signal_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod signal_x86;
#[cfg(target_arch = "x86_64")]
pub use signal_x86::*;
