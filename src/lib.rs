use libc::{c_int, c_char, RTLD_NEXT};

#[unsafe(no_mangle)]
unsafe extern "C" fn close(fd: c_int) -> c_int {
    let ptr = unsafe { libc::dlsym(RTLD_NEXT, c"close".as_ptr() as *const c_char) };
    let close_ptr: *const unsafe extern "C" fn(c_int) -> c_int = ptr.cast();
    let close_fn: unsafe extern "C" fn(c_int) -> c_int = unsafe { close_ptr.read() };
    unsafe { close_fn(fd) }
}
