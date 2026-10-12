#![allow(clippy::missing_safety_doc)]
use libc::{ RTLD_NEXT, mode_t, size_t, ssize_t};
use std::ffi::{self, c_char, c_int, c_void};
use std::sync::atomic::{Ordering, AtomicPtr};

//convenient errno func
unsafe fn errno() -> *mut c_int{unsafe {libc::__errno_location()}}

//caching func addr
static CLOSE_ADDR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static WRITE_ADDR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static READ_ADDR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static OPEN_ADDR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static OPENAT_ADDR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

//func to get func address
unsafe fn func_addr(cached_addr: &AtomicPtr<c_void>, func_name: &ffi::CStr) -> *mut c_void{
    let mut addr = cached_addr.load(Ordering::Relaxed);
    if addr.is_null() {
        addr = unsafe {libc::dlsym(RTLD_NEXT, func_name.as_ptr())};
        cached_addr.store(addr, Ordering::Relaxed);
    }
    addr
}

//func for checking mode
fn needs_mode(flags: c_int) -> bool{
    flags & libc::O_CREAT != 0 || flags & libc::O_TMPFILE == libc::O_TMPFILE
}

//methods intercepting libc calls
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    let addr = unsafe {func_addr(&CLOSE_ADDR, c"close")};
    let close_fn: unsafe extern "C" fn(c_int) -> c_int = unsafe {std::mem::transmute(addr)};
    let errno_saved = unsafe { *errno() };
    unsafe { *errno() = errno_saved };
    unsafe { close_fn(fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t{
    let addr = unsafe { func_addr(&WRITE_ADDR, c"write")};
    let write_fn: unsafe extern "C" fn(c_int, *const c_void, size_t) -> ssize_t = unsafe {std::mem::transmute(addr)};
    let errno_saved = unsafe { *errno() };
    unsafe { *errno() = errno_saved };
    unsafe { write_fn(fd, buf, count)}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t{
    let addr = unsafe {func_addr(&READ_ADDR, c"read")};
    let read_fn: unsafe extern "C" fn(c_int, *mut c_void, size_t) -> ssize_t = unsafe {std::mem::transmute(addr)};
    let errno_saved = unsafe { *errno() };  
    unsafe { *errno() = errno_saved };
    unsafe { read_fn(fd, buf, count)}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int{
    let addr = unsafe { func_addr(&OPEN_ADDR, c"open")};
    let open_fn: unsafe extern "C" fn(*const c_char, c_int, mode_t) -> c_int = unsafe {std::mem::transmute(addr)};
    let r_mode = if needs_mode(flags) {mode} else {0};
    let errno_saved = unsafe { *errno() };  
    unsafe { *errno() = errno_saved };
    unsafe {open_fn(pathname, flags, r_mode)}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int{
    let addr = unsafe { func_addr(&OPENAT_ADDR, c"openat")};
    let openat_fn: unsafe extern "C" fn(c_int, *const c_char, c_int, mode_t) -> c_int = unsafe {std::mem::transmute(addr)}; 
    let r_mode = if needs_mode(flags) {mode} else {0};
    let errno_saved = unsafe { *errno() };  
    unsafe { *errno() = errno_saved };
    unsafe {openat_fn(dirfd, pathname, flags, r_mode)}
} 

