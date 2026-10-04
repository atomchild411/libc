//! IRIX 6.5, N32 ABI.
//!
//! The types, structures and constants are IRIX 6.5.22's, as its headers declare them for the
//! N32 ABI with no feature-test macros (the default `_SGIAPI` namespace), together with what the
//! IRIX toolchain this target links with (clang with IRIX header wrappers, compiler-rt's IRIX
//! builtins) adds where IRIX falls short: `socklen_t`, `sockaddr_storage`, `getaddrinfo`,
//! `CLOCK_MONOTONIC`, `O_CLOEXEC` and the like. Functions that toolchain replaces are linked by
//! the replacement's name (`__irix_*`), as its headers do for C programs.
//!
//! IRIX has no IPv6: the IPv6 types exist so code compiles, and fail at run time.

use crate::prelude::*;

pub type wchar_t = i32;

pub type dev_t = u32;
pub type ino_t = u64;
pub type ino64_t = u64;
pub type off_t = i64;
pub type off64_t = i64;
pub type mode_t = u32;
pub type nlink_t = u32;
pub type uid_t = i32;
pub type gid_t = i32;
pub type pid_t = i32;
pub type id_t = i32;
pub type key_t = c_int;
pub type time_t = c_long;
pub type clock_t = c_int;
pub type clockid_t = c_int;
pub type timer_t = c_int;
pub type suseconds_t = c_long;
pub type useconds_t = c_uint;
pub type blksize_t = c_long;
pub type blkcnt_t = i64;
pub type blkcnt64_t = i64;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type rlim_t = u64;
pub type socklen_t = c_int;
pub type sa_family_t = u16;
pub type in_port_t = u16;
pub type in_addr_t = u32;
pub type speed_t = u32;
pub type tcflag_t = u32;
pub type cc_t = c_uchar;
pub type nfds_t = c_ulong;
pub type nl_item = c_int;
pub type pthread_t = c_uint;
pub type pthread_key_t = c_int;
pub type pthread_once_t = c_int;
pub type sighandler_t = size_t;
pub type caddr_t = *mut c_char;

s! {
    pub struct stat {
        pub st_dev: dev_t,
        st_pad1: [c_long; 3],
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        st_pad2: [c_long; 2],
        pub st_size: off_t,
        st_pad3: c_long,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_fstype: [c_char; 16],
        pub st_projid: c_long,
        st_pad4: [c_long; 7],
    }

    pub struct dirent {
        pub d_ino: ino_t,
        pub d_off: off_t,
        pub d_reclen: c_ushort,
        pub d_name: [c_char; 1],
    }

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [c_uchar; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_pad1: [c_char; 6],
        __ss_align: i64,
        __ss_pad2: [c_char; 112],
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut crate::sockaddr,
        pub ai_canonname: *mut c_char,
        pub ai_next: *mut addrinfo,
    }

    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: c_int,
        pub msg_control: *mut c_void,
        pub msg_controllen: size_t,
        pub msg_flags: c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: size_t,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }

    pub struct ip_mreq {
        pub imr_multiaddr: crate::in_addr,
        pub imr_interface: crate::in_addr,
    }

    pub struct in_addr {
        pub s_addr: in_addr_t,
    }

    pub struct sigset_t {
        __sigbits: [u32; 4],
    }

    pub struct sigaction {
        pub sa_flags: c_int,
        pub sa_sigaction: sighandler_t,
        pub sa_mask: sigset_t,
        sa_resv: [c_int; 2],
    }

    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_code: c_int,
        pub si_errno: c_int,
        __data: [c_int; 29],
    }

    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_size: size_t,
        pub ss_flags: c_int,
    }

    pub struct sched_param {
        pub sched_priority: c_int,
    }

    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_uid: uid_t,
        pub pw_gid: gid_t,
        pub pw_age: *mut c_char,
        pub pw_comment: *mut c_char,
        pub pw_gecos: *mut c_char,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
    }

    pub struct utsname {
        pub sysname: [c_char; 257],
        pub nodename: [c_char; 257],
        pub release: [c_char; 257],
        pub version: [c_char; 257],
        pub machine: [c_char; 257],
        pub m_type: [c_char; 257],
        pub base_rel: [c_char; 257],
        __reserve: [[c_char; 257]; 6],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
    }

    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_ospeed: speed_t,
        pub c_ispeed: speed_t,
        pub c_cc: [cc_t; crate::NCCS],
    }

    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_basetype: [c_char; 16],
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        pub f_fstr: [c_char; 32],
        f_filler: [c_ulong; 16],
    }

    pub struct fd_set {
        fds_bits: [c_int; 32],
    }

    pub struct Dl_info {
        pub dli_fname: *const c_char,
        pub dli_fbase: *mut c_void,
        pub dli_sname: *const c_char,
        pub dli_saddr: *mut c_void,
        pub dli_version: c_int,
        dli_reserved1: c_int,
        dli_reserved: [c_long; 4],
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
    }

    pub struct timezone {
        pub tz_minuteswest: c_int,
        pub tz_dsttime: c_int,
    }

    pub struct posix_spawnattr_t {
        __flags: c_short,
        __pgroup: pid_t,
        __sigdefault: sigset_t,
        __sigmask: sigset_t,
        __policy: c_int,
        __param: sched_param,
    }

    pub struct posix_spawn_file_actions_t {
        __used: c_int,
        __allocated: c_int,
        __actions: *mut c_void,
    }

    pub struct pthread_attr_t {
        __D: [c_long; 5],
    }

    pub struct pthread_mutex_t {
        __D: [c_long; 8],
    }

    pub struct pthread_mutexattr_t {
        __D: [c_long; 2],
    }

    pub struct pthread_cond_t {
        __D: [c_long; 8],
    }

    pub struct pthread_condattr_t {
        __D: [c_long; 2],
    }

    pub struct pthread_rwlock_t {
        __D: [c_long; 16],
    }

    pub struct pthread_rwlockattr_t {
        __D: [c_long; 4],
    }

    #[repr(align(8))]
    pub struct sem_t {
        sem_reserved: [u64; 8],
    }
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut c_void {
        *(self.__data.as_ptr() as *const *mut c_void)
    }

    pub unsafe fn si_pid(&self) -> pid_t {
        self.__data[0]
    }

    pub unsafe fn si_uid(&self) -> uid_t {
        self.__data[1]
    }

    pub unsafe fn si_status(&self) -> c_int {
        self.__data[2]
    }

    pub unsafe fn si_value(&self) -> crate::sigval {
        *(self.__data.as_ptr() as *const crate::sigval)
    }
}

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { __D: [0; 8] };
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { __D: [0; 8] };
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t { __D: [0; 16] };


// Generated from IRIX 6.5.22's headers with the toolchain's wrappers (N32).
include!("consts.rs");

f! {
    pub unsafe fn FD_CLR(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        (*set).fds_bits[fd / 32] &= !(1 << (fd % 32));
    }

    pub unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
        let fd = fd as usize;
        (*set).fds_bits[fd / 32] & (1 << (fd % 32)) != 0
    }

    pub unsafe fn FD_SET(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        (*set).fds_bits[fd / 32] |= 1 << (fd % 32);
    }

    pub unsafe fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }

    pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if ((*mhdr).msg_controllen as usize) < size_of::<cmsghdr>() {
            core::ptr::null_mut()
        } else {
            (*mhdr).msg_control as *mut cmsghdr
        }
    }

    pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
        (cmsg as *mut c_uchar).add(CMSG_ALIGN(size_of::<cmsghdr>()))
    }

    pub const unsafe fn CMSG_ALIGN(len: usize) -> usize {
        (len + size_of::<c_long>() - 1) & !(size_of::<c_long>() - 1)
    }

    pub const unsafe fn CMSG_SPACE(length: c_uint) -> c_uint {
        (CMSG_ALIGN(size_of::<cmsghdr>()) + CMSG_ALIGN(length as usize)) as c_uint
    }

    pub const unsafe fn CMSG_LEN(length: c_uint) -> c_uint {
        (CMSG_ALIGN(size_of::<cmsghdr>()) + length as usize) as c_uint
    }

    pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
        let next = (cmsg as usize) + CMSG_ALIGN((*cmsg).cmsg_len as usize);
        let max = ((*mhdr).msg_control as usize) + (*mhdr).msg_controllen as usize;
        if next + CMSG_ALIGN(size_of::<cmsghdr>()) > max {
            core::ptr::null_mut()
        } else {
            next as *mut cmsghdr
        }
    }
}

safe_f! {
    pub const safe fn WIFEXITED(status: c_int) -> bool {
        (status & 0xff) == 0
    }

    pub const safe fn WEXITSTATUS(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WIFSIGNALED(status: c_int) -> bool {
        (status & 0xff) > 0 && (status & 0xff00) == 0
    }

    pub const safe fn WTERMSIG(status: c_int) -> c_int {
        status & 0x7f
    }

    pub const safe fn WIFSTOPPED(status: c_int) -> bool {
        (status & 0xff) == 0o177 && (status & 0xff00) != 0
    }

    pub const safe fn WSTOPSIG(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WIFCONTINUED(status: c_int) -> bool {
        (status & 0xffff) == 0xffff
    }

    pub const safe fn WCOREDUMP(status: c_int) -> bool {
        (status & 0x80) != 0
    }
}

extern "C" {
    /// The calling thread's `errno` (IRIX's libc keeps a per-thread copy here, and one global).
    pub fn __oserror() -> *mut c_int;

    #[link_name = "__irix_clock_gettime"]
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut crate::timespec) -> c_int;
    #[link_name = "__irix_clock_getres"]
    pub fn clock_getres(clk_id: clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clock_settime(clk_id: clockid_t, tp: *const crate::timespec) -> c_int;

    pub fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    pub fn freeaddrinfo(res: *mut addrinfo);
    pub fn gai_strerror(errcode: c_int) -> *const c_char;
    pub fn getnameinfo(
        sa: *const crate::sockaddr,
        salen: socklen_t,
        host: *mut c_char,
        hostlen: socklen_t,
        serv: *mut c_char,
        servlen: socklen_t,
        flags: c_int,
    ) -> c_int;

    #[link_name = "__irix_recvmsg"]
    pub fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    pub fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;

    pub fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int;

    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t) -> c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const crate::timespec) -> c_int;
    pub fn getentropy(buf: *mut c_void, buflen: size_t) -> c_int;
    pub fn arc4random_buf(buf: *mut c_void, nbytes: size_t);

    pub fn posix_spawn(
        pid: *mut pid_t,
        path: *const c_char,
        file_actions: *const posix_spawn_file_actions_t,
        attrp: *const posix_spawnattr_t,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    pub fn posix_spawnp(
        pid: *mut pid_t,
        file: *const c_char,
        file_actions: *const posix_spawn_file_actions_t,
        attrp: *const posix_spawnattr_t,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: c_short) -> c_int;
    pub fn posix_spawnattr_setsigdefault(attr: *mut posix_spawnattr_t, default: *const sigset_t) -> c_int;
    pub fn posix_spawnattr_setsigmask(attr: *mut posix_spawnattr_t, default: *const sigset_t) -> c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: pid_t) -> c_int;
    pub fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> c_int;
    pub fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> c_int;
    pub fn posix_spawn_file_actions_addopen(
        actions: *mut posix_spawn_file_actions_t,
        fd: c_int,
        path: *const c_char,
        oflag: c_int,
        mode: mode_t,
    ) -> c_int;
    pub fn posix_spawn_file_actions_addclose(actions: *mut posix_spawn_file_actions_t, fd: c_int) -> c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut posix_spawn_file_actions_t,
        fd: c_int,
        newfd: c_int,
    ) -> c_int;

    pub fn sysctl(name: *mut c_int, namelen: c_uint, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int;
    pub fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> c_int;
    pub fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn memalign(align: size_t, size: size_t) -> *mut c_void;
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
    pub fn settimeofday(tp: *const crate::timeval, tz: *const c_void) -> c_int;
    pub fn sched_yield() -> c_int;
}
