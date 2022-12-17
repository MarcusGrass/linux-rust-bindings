#[cfg(target_arch = "aarch64")]
pub mod termios_arm64;
#[cfg(target_arch = "aarch64")]
pub use termios_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod termios_x86;
#[cfg(target_arch = "x86_64")]
pub use termios_x86::*;
