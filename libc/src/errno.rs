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

/// Checks if a value is null, if it is then sets errno to EINVAL and then returns EOF
macro_rules! die {
    ($e:expr) => {
        if $e.is_null() {
            #[allow(unused_unsafe)]
            unsafe {
                crate::errno::ERRNO = xenia::Errno::INVAL.raw_os_error()
            };

            return EOF;
        }
    };
}

pub(crate) use die;
pub(crate) use try_io;
