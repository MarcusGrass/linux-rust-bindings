/* automatically generated by rust-bindgen 0.64.0 */

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
pub type __u64 = ::core::ffi::c_ulonglong;
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
