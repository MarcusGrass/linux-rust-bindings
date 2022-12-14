/* automatically generated by rust-bindgen 0.63.0 */

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_RDWR: i32 = 2;
pub const O_CREAT: i32 = 64;
pub const O_EXCL: i32 = 128;
pub const O_NOCTTY: i32 = 256;
pub const O_TRUNC: i32 = 512;
pub const O_APPEND: i32 = 1024;
pub const O_NONBLOCK: i32 = 2048;
pub const O_DIRECTORY: i32 = 65536;
pub const AT_SYSINFO_EHDR: i32 = 33;
pub const AT_VECTOR_SIZE_ARCH: i32 = 3;
pub const AT_NULL: i32 = 0;
pub const AT_IGNORE: i32 = 1;
pub const AT_EXECFD: i32 = 2;
pub const AT_PHDR: i32 = 3;
pub const AT_PHENT: i32 = 4;
pub const AT_PHNUM: i32 = 5;
pub const AT_PAGESZ: i32 = 6;
pub const AT_BASE: i32 = 7;
pub const AT_FLAGS: i32 = 8;
pub const AT_ENTRY: i32 = 9;
pub const AT_NOTELF: i32 = 10;
pub const AT_UID: i32 = 11;
pub const AT_EUID: i32 = 12;
pub const AT_GID: i32 = 13;
pub const AT_EGID: i32 = 14;
pub const AT_PLATFORM: i32 = 15;
pub const AT_HWCAP: i32 = 16;
pub const AT_CLKTCK: i32 = 17;
pub const AT_SECURE: i32 = 23;
pub const AT_BASE_PLATFORM: i32 = 24;
pub const AT_RANDOM: i32 = 25;
pub const AT_HWCAP2: i32 = 26;
pub const AT_EXECFN: i32 = 31;
pub const AT_MINSIGSTKSZ: i32 = 51;
pub const O_ACCMODE: i32 = 3;
pub const O_DSYNC: i32 = 4096;
pub const O_DIRECT: i32 = 16384;
pub const O_LARGEFILE: i32 = 32768;
pub const O_NOFOLLOW: i32 = 131072;
pub const O_NOATIME: i32 = 262144;
pub const O_CLOEXEC: i32 = 524288;
pub const O_SYNC: i32 = 1052672;
pub const O_PATH: i32 = 2097152;
pub const O_TMPFILE: i32 = 4259840;
pub const O_TMPFILE_MASK: i32 = 4259904;
pub const O_NDELAY: i32 = 2048;
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
pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const ENOTBLK: i32 = 15;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const EDEADLK: i32 = 35;
pub const ENAMETOOLONG: i32 = 36;
pub const ENOLCK: i32 = 37;
pub const ENOSYS: i32 = 38;
pub const ENOTEMPTY: i32 = 39;
pub const ELOOP: i32 = 40;
pub const EWOULDBLOCK: i32 = 11;
pub const ENOMSG: i32 = 42;
pub const EIDRM: i32 = 43;
pub const ECHRNG: i32 = 44;
pub const EL2NSYNC: i32 = 45;
pub const EL3HLT: i32 = 46;
pub const EL3RST: i32 = 47;
pub const ELNRNG: i32 = 48;
pub const EUNATCH: i32 = 49;
pub const ENOCSI: i32 = 50;
pub const EL2HLT: i32 = 51;
pub const EBADE: i32 = 52;
pub const EBADR: i32 = 53;
pub const EXFULL: i32 = 54;
pub const ENOANO: i32 = 55;
pub const EBADRQC: i32 = 56;
pub const EBADSLT: i32 = 57;
pub const EDEADLOCK: i32 = 35;
pub const EBFONT: i32 = 59;
pub const ENOSTR: i32 = 60;
pub const ENODATA: i32 = 61;
pub const ETIME: i32 = 62;
pub const ENOSR: i32 = 63;
pub const ENONET: i32 = 64;
pub const ENOPKG: i32 = 65;
pub const EREMOTE: i32 = 66;
pub const ENOLINK: i32 = 67;
pub const EADV: i32 = 68;
pub const ESRMNT: i32 = 69;
pub const ECOMM: i32 = 70;
pub const EPROTO: i32 = 71;
pub const EMULTIHOP: i32 = 72;
pub const EDOTDOT: i32 = 73;
pub const EBADMSG: i32 = 74;
pub const EOVERFLOW: i32 = 75;
pub const ENOTUNIQ: i32 = 76;
pub const EBADFD: i32 = 77;
pub const EREMCHG: i32 = 78;
pub const ELIBACC: i32 = 79;
pub const ELIBBAD: i32 = 80;
pub const ELIBSCN: i32 = 81;
pub const ELIBMAX: i32 = 82;
pub const ELIBEXEC: i32 = 83;
pub const EILSEQ: i32 = 84;
pub const ERESTART: i32 = 85;
pub const ESTRPIPE: i32 = 86;
pub const EUSERS: i32 = 87;
pub const ENOTSOCK: i32 = 88;
pub const EDESTADDRREQ: i32 = 89;
pub const EMSGSIZE: i32 = 90;
pub const EPROTOTYPE: i32 = 91;
pub const ENOPROTOOPT: i32 = 92;
pub const EPROTONOSUPPORT: i32 = 93;
pub const ESOCKTNOSUPPORT: i32 = 94;
pub const EOPNOTSUPP: i32 = 95;
pub const EPFNOSUPPORT: i32 = 96;
pub const EAFNOSUPPORT: i32 = 97;
pub const EADDRINUSE: i32 = 98;
pub const EADDRNOTAVAIL: i32 = 99;
pub const ENETDOWN: i32 = 100;
pub const ENETUNREACH: i32 = 101;
pub const ENETRESET: i32 = 102;
pub const ECONNABORTED: i32 = 103;
pub const ECONNRESET: i32 = 104;
pub const ENOBUFS: i32 = 105;
pub const EISCONN: i32 = 106;
pub const ENOTCONN: i32 = 107;
pub const ESHUTDOWN: i32 = 108;
pub const ETOOMANYREFS: i32 = 109;
pub const ETIMEDOUT: i32 = 110;
pub const ECONNREFUSED: i32 = 111;
pub const EHOSTDOWN: i32 = 112;
pub const EHOSTUNREACH: i32 = 113;
pub const EALREADY: i32 = 114;
pub const EINPROGRESS: i32 = 115;
pub const ESTALE: i32 = 116;
pub const EUCLEAN: i32 = 117;
pub const ENOTNAM: i32 = 118;
pub const ENAVAIL: i32 = 119;
pub const EISNAM: i32 = 120;
pub const EREMOTEIO: i32 = 121;
pub const EDQUOT: i32 = 122;
pub const ENOMEDIUM: i32 = 123;
pub const EMEDIUMTYPE: i32 = 124;
pub const ECANCELED: i32 = 125;
pub const ENOKEY: i32 = 126;
pub const EKEYEXPIRED: i32 = 127;
pub const EKEYREVOKED: i32 = 128;
pub const EKEYREJECTED: i32 = 129;
pub const EOWNERDEAD: i32 = 130;
pub const ENOTRECOVERABLE: i32 = 131;
pub const ERFKILL: i32 = 132;
pub const EHWPOISON: i32 = 133;
pub const CLOCK_REALTIME: i32 = 0;
pub const CLOCK_MONOTONIC: i32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: i32 = 3;
pub const CLOCK_MONOTONIC_RAW: i32 = 4;
pub const CLOCK_REALTIME_COARSE: i32 = 5;
pub const CLOCK_MONOTONIC_COARSE: i32 = 6;
pub const CLOCK_BOOTTIME: i32 = 7;
pub const CLOCK_REALTIME_ALARM: i32 = 8;
pub const CLOCK_BOOTTIME_ALARM: i32 = 9;
pub const CLOCK_SGI_CYCLE: i32 = 10;
pub const CLOCK_TAI: i32 = 11;
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGIOT: i32 = 6;
pub const SIGBUS: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGSTKFLT: i32 = 16;
pub const SIGCHLD: i32 = 17;
pub const SIGCONT: i32 = 18;
pub const SIGSTOP: i32 = 19;
pub const SIGTSTP: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGURG: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGIO: i32 = 29;
pub const SIGPOLL: i32 = 29;
pub const SIGPWR: i32 = 30;
pub const SIGSYS: i32 = 31;
pub const SIGUNUSED: i32 = 31;
pub const SIGRTMIN: i32 = 32;
pub const SA_RESTORER: i32 = 67108864;
pub const SIGSTKSZ: i32 = 8192;
pub const SA_NOCLDSTOP: i32 = 1;
pub const SA_NOCLDWAIT: i32 = 2;
pub const SA_SIGINFO: i32 = 4;
pub const SA_UNSUPPORTED: i32 = 1024;
pub const SA_EXPOSE_TAGBITS: i32 = 2048;
pub const SA_ONSTACK: i32 = 134217728;
pub const SA_RESTART: i32 = 268435456;
pub const SA_NODEFER: i32 = 1073741824;
pub const SA_RESETHAND: i64 = 2147483648;
pub const SA_NOMASK: i32 = 1073741824;
pub const SA_ONESHOT: i64 = 2147483648;
pub const SIG_BLOCK: i32 = 0;
pub const SIG_UNBLOCK: i32 = 1;
pub const SIG_SETMASK: i32 = 2;
pub const POLL_IN: i32 = 1;
pub const POLL_OUT: i32 = 2;
pub const POLL_MSG: i32 = 3;
pub const POLL_ERR: i32 = 4;
pub const POLL_PRI: i32 = 5;
pub const POLL_HUP: i32 = 6;
pub const EMT_TAGOVF: i32 = 1;
pub const SIGEV_SIGNAL: i32 = 0;
pub const SIGEV_NONE: i32 = 1;
pub const SIGEV_THREAD: i32 = 2;
pub const SIGEV_THREAD_ID: i32 = 4;
pub const SIGEV_MAX_SIZE: i32 = 64;
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
pub const MAP_32BIT: i32 = 64;
pub const PROT_READ: i32 = 1;
pub const PROT_WRITE: i32 = 2;
pub const PROT_EXEC: i32 = 4;
pub const PROT_SEM: i32 = 8;
pub const PROT_NONE: i32 = 0;
pub const PROT_GROWSDOWN: i32 = 16777216;
pub const PROT_GROWSUP: i32 = 33554432;
pub const MAP_TYPE: i32 = 15;
pub const MAP_FIXED: i32 = 16;
pub const MAP_ANONYMOUS: i32 = 32;
pub const MAP_POPULATE: i32 = 32768;
pub const MAP_NONBLOCK: i32 = 65536;
pub const MAP_STACK: i32 = 131072;
pub const MAP_HUGETLB: i32 = 262144;
pub const MAP_SYNC: i32 = 524288;
pub const MAP_FIXED_NOREPLACE: i32 = 1048576;
pub const MAP_UNINITIALIZED: i32 = 67108864;
pub const MAP_FILE: i32 = 0;
pub const MAP_GROWSDOWN: i32 = 256;
pub const MAP_DENYWRITE: i32 = 2048;
pub const MAP_EXECUTABLE: i32 = 4096;
pub const MAP_LOCKED: i32 = 8192;
pub const MAP_NORESERVE: i32 = 16384;
pub const MAP_SHARED: i32 = 1;
pub const MAP_PRIVATE: i32 = 2;
pub const MAP_SHARED_VALIDATE: i32 = 3;
pub const MAP_HUGE_SHIFT: i32 = 26;
pub const MAP_HUGE_MASK: i32 = 63;
pub const MAP_HUGE_16KB: i32 = 939524096;
pub const MAP_HUGE_64KB: i32 = 1073741824;
pub const MAP_HUGE_512KB: i32 = 1275068416;
pub const MAP_HUGE_1MB: i32 = 1342177280;
pub const MAP_HUGE_2MB: i32 = 1409286144;
pub const MAP_HUGE_8MB: i32 = 1543503872;
pub const MAP_HUGE_16MB: i32 = 1610612736;
pub const MAP_HUGE_32MB: i32 = 1677721600;
pub const MAP_HUGE_256MB: i32 = 1879048192;
pub const MAP_HUGE_512MB: i32 = 1946157056;
pub const MAP_HUGE_1GB: i32 = 2013265920;
pub const MAP_HUGE_2GB: i32 = 2080374784;
pub const MAP_HUGE_16GB: i64 = 2281701376;
pub const POLLIN: i32 = 1;
pub const POLLPRI: i32 = 2;
pub const POLLOUT: i32 = 4;
pub const POLLERR: i32 = 8;
pub const POLLHUP: i32 = 16;
pub const POLLNVAL: i32 = 32;
pub const POLLRDNORM: i32 = 64;
pub const POLLRDBAND: i32 = 128;
pub const POLLWRNORM: i32 = 256;
pub const POLLWRBAND: i32 = 512;
pub const POLLMSG: i32 = 1024;
pub const POLLREMOVE: i32 = 4096;
pub const POLLRDHUP: i32 = 8192;
pub const WNOHANG: i32 = 1;
pub const WUNTRACED: i32 = 2;
pub const WSTOPPED: i32 = 2;
pub const WEXITED: i32 = 4;
pub const WCONTINUED: i32 = 8;
pub const WNOWAIT: i32 = 16777216;
pub const CLONE_VM: i32 = 256;
pub const CLONE_FS: i32 = 512;
pub const CLONE_FILES: i32 = 1024;
pub const CLONE_SIGHAND: i32 = 2048;
pub const CLONE_PIDFD: i32 = 4096;
pub const CLONE_PTRACE: i32 = 8192;
pub const CLONE_VFORK: i32 = 16384;
pub const CLONE_PARENT: i32 = 32768;
pub const CLONE_THREAD: i32 = 65536;
pub const CLONE_NEWNS: i32 = 131072;
pub const CLONE_SYSVSEM: i32 = 262144;
pub const CLONE_SETTLS: i32 = 524288;
pub const CLONE_PARENT_SETTID: i32 = 1048576;
pub const CLONE_CHILD_CLEARTID: i32 = 2097152;
pub const CLONE_DETACHED: i32 = 4194304;
pub const CLONE_UNTRACED: i32 = 8388608;
pub const CLONE_CHILD_SETTID: i32 = 16777216;
pub const CLONE_NEWCGROUP: i32 = 33554432;
pub const CLONE_NEWUTS: i32 = 67108864;
pub const CLONE_NEWIPC: i32 = 134217728;
pub const CLONE_NEWUSER: i32 = 268435456;
pub const CLONE_NEWPID: i32 = 536870912;
pub const CLONE_NEWNET: i32 = 1073741824;
pub const CLONE_IO: i64 = 2147483648;
pub const CLONE_CLEAR_SIGHAND: i64 = 4294967296;
pub const CLONE_INTO_CGROUP: i64 = 8589934592;
pub const CLONE_NEWTIME: i32 = 128;
pub const CLONE_ARGS_SIZE_VER0: i32 = 64;
pub const CLONE_ARGS_SIZE_VER1: i32 = 80;
pub const CLONE_ARGS_SIZE_VER2: i32 = 88;
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = ::core::ffi::c_longlong;
pub type __kernel_timer_t = ::core::ffi::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: ::core::ffi::c_longlong,
}
pub type sigset_t = ::core::ffi::c_ulong;
pub type __signalfn_t = ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>;
pub type __sighandler_t = __signalfn_t;
pub type __restorefn_t = ::core::option::Option<unsafe extern "C" fn()>;
pub type __sigrestore_t = __restorefn_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: ::core::ffi::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type __kernel_sa_family_t = ::core::ffi::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_un {
    pub sun_family: __kernel_sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108usize],
}
#[doc = " struct clone_args - arguments for the clone3 syscall\n @flags:        Flags for the new process as listed above.\n                All flags are valid except for CSIGNAL and\n                CLONE_DETACHED.\n @pidfd:        If CLONE_PIDFD is set, a pidfd will be\n                returned in this argument.\n @child_tid:    If CLONE_CHILD_SETTID is set, the TID of the\n                child process will be returned in the child's\n                memory.\n @parent_tid:   If CLONE_PARENT_SETTID is set, the TID of\n                the child process will be returned in the\n                parent's memory.\n @exit_signal:  The exit_signal the parent process will be\n                sent when the child exits.\n @stack:        Specify the location of the stack for the\n                child process.\n                Note, @stack is expected to point to the\n                lowest address. The stack direction will be\n                determined by the kernel and set up\n                appropriately based on @stack_size.\n @stack_size:   The size of the stack for the child process.\n @tls:          If CLONE_SETTLS is set, the tls descriptor\n                is set to tls.\n @set_tid:      Pointer to an array of type *pid_t. The size\n                of the array is defined using @set_tid_size.\n                This array is used to select PIDs/TIDs for\n                newly created processes. The first element in\n                this defines the PID in the most nested PID\n                namespace. Each additional element in the array\n                defines the PID in the parent PID namespace of\n                the original PID namespace. If the array has\n                less entries than the number of currently\n                nested PID namespaces only the PIDs in the\n                corresponding namespaces are set.\n @set_tid_size: This defines the size of the array referenced\n                in @set_tid. This cannot be larger than the\n                kernel's limit of nested PID namespaces.\n @cgroup:       If CLONE_INTO_CGROUP is specified set this to\n                a file descriptor for the cgroup.\n\n The structure is versioned by size and thus extensible.\n New struct members must go at the end of the struct and\n must be properly 64bit aligned."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clone_args {
    pub flags: __u64,
    pub pidfd: __u64,
    pub child_tid: __u64,
    pub parent_tid: __u64,
    pub exit_signal: __u64,
    pub stack: __u64,
    pub stack_size: __u64,
    pub tls: __u64,
    pub set_tid: __u64,
    pub set_tid_size: __u64,
    pub cgroup: __u64,
}