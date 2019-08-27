

#![cfg_attr(libc_deny_warnings, deny(warnings))]
#![allow(
    bad_style,
    non_camel_case_types,
    overflowing_literals,
    improper_ctypes,
    unknown_lints,
    redundant_semicolon
)]
// Attributes needed when building as part of the standard library
#![cfg_attr(
    feature = "rustc-dep-of-std",
    feature(cfg_target_vendor, link_cfg, no_core)
)]
#![cfg_attr(libc_thread_local, feature(thread_local))]
// Enable extra lints:
#![cfg_attr(feature = "extra_traits", deny(missing_debug_implementations))]
#![deny(missing_copy_implementations, safe_packed_borrows)]
#![no_std]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]
#![cfg_attr(target_os = "redox", feature(static_nobundle))]

// The above is copied from libc
#[macro_use]
extern crate syscall;

pub use core::ffi::c_void;

pub type c_char = i8;
// pub type c_schar = i8;
// pub type c_uchar = u8;
pub type c_short = i16;
// pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
// pub type c_float = f32;
// pub type c_double = f64;
// pub type c_longlong = i64;
// pub type c_ulonglong = u64;
// pub type intmax_t = i64;
// pub type uintmax_t = u64;
pub type c_long = i64;

pub type size_t = usize;
// pub type ptrdiff_t = isize;
// pub type intptr_t = isize;
// pub type uintptr_t = usize;
pub type ssize_t = isize;

// pub type pid_t = i32;
// pub type uid_t = u32;
// pub type gid_t = u32;
// pub type in_addr_t = u32;
// pub type in_port_t = u16;
// pub type sighandler_t = size_t;
// pub type cc_t = c_uchar;


pub type mode_t = u32;
pub type tcflag_t = c_uint;
pub type idtype_t = c_uint;

mod consts;
pub use consts::*;

// structs
#[repr(C)]
#[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
#[allow(deprecated)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[allow(deprecated)]
impl Copy for iovec {}
#[allow(deprecated)]
impl Clone for iovec {
    fn clone(&self) -> iovec { *self }
}

// unsafe to mikick "extern"
pub unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    syscall!(READ, fd, buf, count) as ssize_t
}

pub unsafe fn umount(target: *const c_char) -> c_int {
    syscall!(UMOUNT2, target) as c_int
}

pub unsafe fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t {
    syscall!(WRITEV, fd, iov, iovcnt) as ssize_t
}

pub unsafe fn close(fd: c_int) -> c_int {
    syscall!(CLOSE, fd) as c_int
}
