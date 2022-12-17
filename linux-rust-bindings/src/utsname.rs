#[cfg(target_arch = "aarch64")]
pub mod utsname_arm64;
#[cfg(target_arch = "aarch64")]
pub use utsname_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod utsname_x86;
#[cfg(target_arch = "x86_64")]
pub use utsname_x86::*;
