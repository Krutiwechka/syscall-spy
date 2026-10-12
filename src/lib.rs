#![allow(clippy::missing_safety_doc)]
use libc::{ RTLD_NEXT, mode_t, size_t, ssize_t};
use std::ffi::{c_char, c_int, c_void};

unsafe fn errno() -> *mut c_int{unsafe {libc::__errno_location()}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    let errno_saved = unsafe { *errno() };
    let addr = unsafe { libc::dlsym(RTLD_NEXT, c"close".as_ptr()) };
    let close_fn: unsafe extern "C" fn(c_int) -> c_int = unsafe {std::mem::transmute(addr)};
    unsafe { *errno() = errno_saved };
    unsafe { close_fn(fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t{
    let errno_saved = unsafe { *errno() };
    let addr = unsafe {libc::dlsym(RTLD_NEXT, c"write".as_ptr()) };
    let write_fn: unsafe extern "C" fn(c_int, *const c_void, size_t) -> ssize_t = unsafe {std::mem::transmute(addr)};
    unsafe { *errno() = errno_saved };
    unsafe { write_fn(fd, buf, count)}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t{
    let errno_saved = unsafe { *errno() };  
    let addr = unsafe {libc::dlsym(RTLD_NEXT, c"read".as_ptr())};
    let read_fn: unsafe extern "C" fn(c_int, *mut c_void, size_t) -> ssize_t = unsafe {std::mem::transmute(addr)};
    unsafe { *errno() = errno_saved };
    unsafe { read_fn(fd, buf, count)}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int{
    let errno_saved = unsafe { *errno() };  
    let addr = unsafe {libc::dlsym(RTLD_NEXT, c"open".as_ptr()) };
    let open_fn: unsafe extern "C" fn(*const c_char, c_int, c_int) -> c_int = unsafe {std::mem::transmute(addr)};
    let r_mode = if (libc::O_CREAT | libc::O_TMPFILE) & flags == 0 {mode} else {0};
    unsafe { *errno() = errno_saved };
    unsafe {open_fn(pathname, flags, r_mode)}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int{
    let errno_saved = unsafe { *errno() };  
    let addr = unsafe {libc::dlsym(RTLD_NEXT, c"openat".as_ptr()) };
    let openat_fn: unsafe extern "C" fn(c_int, *const c_char, c_int, mode_t) -> c_int = unsafe {std::mem::transmute(addr)};
    let r_mode = if (libc::O_CREAT | libc::O_TMPFILE) & flags == 0 {mode} else {0};
    unsafe { *errno() = errno_saved };
    unsafe {openat_fn(dirfd, pathname, flags, r_mode)}
} 

