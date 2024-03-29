#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![no_std]

#[cfg(feature = "auxvec")]
pub mod auxvec;
#[cfg(feature = "elf")]
pub mod elf;
#[cfg(feature = "epoll")]
pub mod epoll;
#[cfg(feature = "errno")]
pub mod errno;
#[cfg(feature = "fcntl")]
pub mod fcntl;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "futex")]
pub mod futex;
#[cfg(feature = "hidio")]
pub mod hidio;
#[cfg(feature = "io_uring")]
pub mod io_uring;
#[cfg(feature = "ioctl")]
pub mod ioctl;
#[cfg(feature = "mman")]
pub mod mman;
#[cfg(feature = "mount")]
pub mod mount;
#[cfg(feature = "poll")]
pub mod poll;
#[cfg(feature = "sched")]
pub mod sched;
#[cfg(feature = "signal")]
pub mod signal;
#[cfg(feature = "socket")]
pub mod socket;
#[cfg(feature = "stat")]
pub mod stat;
#[cfg(feature = "termios")]
pub mod termios;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "types")]
pub mod types;
#[cfg(feature = "uio")]
pub mod uio;
#[cfg(feature = "usb")]
pub mod usb;
#[cfg(feature = "utsname")]
pub mod utsname;
#[cfg(feature = "wait")]
pub mod wait;

#[cfg(test)]
mod tests {}
