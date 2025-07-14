use core::ffi::{c_int, c_void};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    unsafe { builtins::mem::memset(dest.cast(), c, n).cast() }
}
