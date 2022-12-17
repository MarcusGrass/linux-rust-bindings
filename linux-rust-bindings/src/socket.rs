#[cfg(target_arch = "aarch64")]
pub mod socket_arm64;
#[cfg(target_arch = "aarch64")]
pub use socket_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod socket_x86;
#[cfg(target_arch = "x86_64")]
pub use socket_x86::*;
