/* automatically generated by rust-bindgen 0.63.0 */

pub const O_DIRECTORY: i32 = 16384;
pub const O_NOFOLLOW: i32 = 32768;
pub const O_DIRECT: i32 = 65536;
pub const O_LARGEFILE: i32 = 131072;
pub const O_ACCMODE: i32 = 3;
pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_RDWR: i32 = 2;
pub const O_CREAT: i32 = 64;
pub const O_EXCL: i32 = 128;
pub const O_NOCTTY: i32 = 256;
pub const O_TRUNC: i32 = 512;
pub const O_APPEND: i32 = 1024;
pub const O_NONBLOCK: i32 = 2048;
pub const O_DSYNC: i32 = 4096;
pub const O_NOATIME: i32 = 262144;
pub const O_CLOEXEC: i32 = 524288;
pub const O_SYNC: i32 = 1052672;
pub const O_PATH: i32 = 2097152;
pub const O_TMPFILE: i32 = 4210688;
pub const O_TMPFILE_MASK: i32 = 4210752;
pub const O_NDELAY: i32 = 2048;
pub const F_DUPFD: i32 = 0;
pub const F_GETFD: i32 = 1;
pub const F_SETFD: i32 = 2;
pub const F_GETFL: i32 = 3;
pub const F_SETFL: i32 = 4;
pub const F_GETLK: i32 = 5;
pub const F_SETLK: i32 = 6;
pub const F_SETLKW: i32 = 7;
pub const F_SETOWN: i32 = 8;
pub const F_GETOWN: i32 = 9;
pub const F_SETSIG: i32 = 10;
pub const F_GETSIG: i32 = 11;
pub const F_SETOWN_EX: i32 = 15;
pub const F_GETOWN_EX: i32 = 16;
pub const F_GETOWNER_UIDS: i32 = 17;
pub const F_OFD_GETLK: i32 = 36;
pub const F_OFD_SETLK: i32 = 37;
pub const F_OFD_SETLKW: i32 = 38;
pub const F_OWNER_TID: i32 = 0;
pub const F_OWNER_PID: i32 = 1;
pub const F_OWNER_PGRP: i32 = 2;
pub const F_RDLCK: i32 = 0;
pub const F_WRLCK: i32 = 1;
pub const F_UNLCK: i32 = 2;
pub const F_EXLCK: i32 = 4;
pub const F_SHLCK: i32 = 8;
pub const F_LINUX_SPECIFIC_BASE: i32 = 1024;
pub const F_SETLEASE: i32 = 1024;
pub const F_GETLEASE: i32 = 1025;
pub const F_CANCELLK: i32 = 1029;
pub const F_DUPFD_CLOEXEC: i32 = 1030;
pub const F_NOTIFY: i32 = 1026;
pub const F_SETPIPE_SZ: i32 = 1031;
pub const F_GETPIPE_SZ: i32 = 1032;
pub const F_ADD_SEALS: i32 = 1033;
pub const F_GET_SEALS: i32 = 1034;
pub const F_SEAL_SEAL: i32 = 1;
pub const F_SEAL_SHRINK: i32 = 2;
pub const F_SEAL_GROW: i32 = 4;
pub const F_SEAL_WRITE: i32 = 8;
pub const F_SEAL_FUTURE_WRITE: i32 = 16;
pub const F_GET_RW_HINT: i32 = 1035;
pub const F_SET_RW_HINT: i32 = 1036;
pub const F_GET_FILE_RW_HINT: i32 = 1037;
pub const F_SET_FILE_RW_HINT: i32 = 1038;
pub const AT_FDCWD: i32 = -100;
pub const AT_SYMLINK_NOFOLLOW: i32 = 256;
pub const AT_EACCESS: i32 = 512;
pub const AT_REMOVEDIR: i32 = 512;
pub const AT_SYMLINK_FOLLOW: i32 = 1024;
pub const AT_NO_AUTOMOUNT: i32 = 2048;
pub const AT_EMPTY_PATH: i32 = 4096;
pub const AT_STATX_SYNC_TYPE: i32 = 24576;
pub const AT_STATX_SYNC_AS_STAT: i32 = 0;
pub const AT_STATX_FORCE_SYNC: i32 = 8192;
pub const AT_STATX_DONT_SYNC: i32 = 16384;
pub const AT_RECURSIVE: i32 = 32768;
