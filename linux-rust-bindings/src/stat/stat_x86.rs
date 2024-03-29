/* automatically generated by rust-bindgen 0.69.1 */

pub const S_IFMT: i32 = 61440;
pub const S_IFSOCK: i32 = 49152;
pub const S_IFLNK: i32 = 40960;
pub const S_IFREG: i32 = 32768;
pub const S_IFBLK: i32 = 24576;
pub const S_IFDIR: i32 = 16384;
pub const S_IFCHR: i32 = 8192;
pub const S_IFIFO: i32 = 4096;
pub const S_ISUID: i32 = 2048;
pub const S_ISGID: i32 = 1024;
pub const S_ISVTX: i32 = 512;
pub const S_IRWXU: i32 = 448;
pub const S_IRUSR: i32 = 256;
pub const S_IWUSR: i32 = 128;
pub const S_IXUSR: i32 = 64;
pub const S_IRWXG: i32 = 56;
pub const S_IRGRP: i32 = 32;
pub const S_IWGRP: i32 = 16;
pub const S_IXGRP: i32 = 8;
pub const S_IRWXO: i32 = 7;
pub const S_IROTH: i32 = 4;
pub const S_IWOTH: i32 = 2;
pub const S_IXOTH: i32 = 1;
pub const STATX_TYPE: i32 = 1;
pub const STATX_MODE: i32 = 2;
pub const STATX_NLINK: i32 = 4;
pub const STATX_UID: i32 = 8;
pub const STATX_GID: i32 = 16;
pub const STATX_ATIME: i32 = 32;
pub const STATX_MTIME: i32 = 64;
pub const STATX_CTIME: i32 = 128;
pub const STATX_INO: i32 = 256;
pub const STATX_SIZE: i32 = 512;
pub const STATX_BLOCKS: i32 = 1024;
pub const STATX_BASIC_STATS: i32 = 2047;
pub const STATX_BTIME: i32 = 2048;
pub const STATX_MNT_ID: i32 = 4096;
pub const STATX_DIOALIGN: i32 = 8192;
pub const STATX__RESERVED: i64 = 2147483648;
pub const STATX_ALL: i32 = 4095;
pub const STATX_ATTR_COMPRESSED: i32 = 4;
pub const STATX_ATTR_IMMUTABLE: i32 = 16;
pub const STATX_ATTR_APPEND: i32 = 32;
pub const STATX_ATTR_NODUMP: i32 = 64;
pub const STATX_ATTR_ENCRYPTED: i32 = 2048;
pub const STATX_ATTR_AUTOMOUNT: i32 = 4096;
pub const STATX_ATTR_MOUNT_ROOT: i32 = 8192;
pub const STATX_ATTR_VERITY: i32 = 1048576;
pub const STATX_ATTR_DAX: i32 = 2097152;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: __kernel_ulong_t,
    pub st_ino: __kernel_ulong_t,
    pub st_nlink: __kernel_ulong_t,
    pub st_mode: ::core::ffi::c_uint,
    pub st_uid: ::core::ffi::c_uint,
    pub st_gid: ::core::ffi::c_uint,
    pub __pad0: ::core::ffi::c_uint,
    pub st_rdev: __kernel_ulong_t,
    pub st_size: __kernel_long_t,
    pub st_blksize: __kernel_long_t,
    pub st_blocks: __kernel_long_t,
    pub st_atime: __kernel_ulong_t,
    pub st_atime_nsec: __kernel_ulong_t,
    pub st_mtime: __kernel_ulong_t,
    pub st_mtime_nsec: __kernel_ulong_t,
    pub st_ctime: __kernel_ulong_t,
    pub st_ctime_nsec: __kernel_ulong_t,
    pub __unused: [__kernel_long_t; 3usize],
}
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1usize],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub __spare3: [__u64; 12usize],
}
