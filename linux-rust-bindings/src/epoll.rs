#[cfg(target_arch = "aarch64")]
pub mod epoll_arm64;
#[cfg(target_arch = "aarch64")]
pub use epoll_arm64::*;
#[cfg(target_arch = "x86_64")]
pub mod epoll_x86;
#[cfg(target_arch = "x86_64")]
pub use epoll_x86::*;
