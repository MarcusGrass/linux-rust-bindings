#[cfg(target_arch = "aarch64")]
pub mod bindings_arm64;
#[cfg(target_arch = "aarch64")]
pub use bindings_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod bindings_x86;
#[cfg(target_arch = "x86_64")]
pub use bindings_x86::*;
