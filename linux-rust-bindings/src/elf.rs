#[cfg(target_arch = "aarch64")]
pub mod elf_arm64;
#[cfg(target_arch = "aarch64")]
pub use elf_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod elf_x86;
#[cfg(target_arch = "x86_64")]
pub use elf_x86::*;
