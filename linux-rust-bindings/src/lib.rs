#![allow(non_camel_case_types)]
#![no_std]
#[cfg(feature = "aux")]
pub mod aux;
#[cfg(feature = "errno")]
pub mod errno;
#[cfg(feature = "elf")]
pub mod elf;
#[cfg(feature = "fcntl")]
pub mod fcntl;
#[cfg(feature = "ioctl")]
pub mod ioctl;
#[cfg(feature = "mman")]
pub mod mman;
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
#[cfg(feature = "wait")]
pub mod wait;
#[cfg(feature = "types")]
pub mod types;
#[cfg(feature = "utsname")]
pub mod utsname;

#[cfg(test)]
mod tests {}
