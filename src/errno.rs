use core::ffi::c_int;

#[thread_local]
pub(crate) static mut ERRNO: c_int = 0;

#[unsafe(no_mangle)]
extern "C" fn __errno_location() -> *mut c_int {
    &raw mut ERRNO
}

macro_rules! try_io {
    ($e:expr) => {
        match $e {
            Ok(res) => res,
            Err(err) => {
                #[allow(unused_unsafe)]
                unsafe {
                    crate::errno::ERRNO = err.raw_os_error()
                };
                return crate::stdio::EOF;
            }
        }
    };
}

pub(crate) use try_io;
