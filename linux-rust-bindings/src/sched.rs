#[cfg(target_arch = "aarch64")]
pub mod sched_arm64;
#[cfg(target_arch = "aarch64")]
pub use sched_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod sched_x86;
#[cfg(target_arch = "x86_64")]
pub use sched_x86::*;
