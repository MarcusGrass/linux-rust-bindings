#[cfg(target_arch = "aarch64")]
pub mod usb_arm64;
#[cfg(target_arch = "aarch64")]
pub use usb_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod usb_x86;
#[cfg(target_arch = "x86_64")]
pub use usb_x86::*;
