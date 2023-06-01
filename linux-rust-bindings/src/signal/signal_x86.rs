/* automatically generated by rust-bindgen 0.65.1 */

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
pub const SIGEV_SIGNAL: i32 = 0;
pub const SIGEV_NONE: i32 = 1;
pub const SIGEV_THREAD: i32 = 2;
pub const SIGEV_THREAD_ID: i32 = 4;
pub const SIGEV_MAX_SIZE: i32 = 64;
pub type __signalfn_t = ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>;
pub type __sighandler_t = __signalfn_t;
pub type __restorefn_t = ::core::option::Option<unsafe extern "C" fn()>;
pub type __sigrestore_t = __restorefn_t;
