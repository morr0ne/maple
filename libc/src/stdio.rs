use xenia::{
    Errno,
    fd::{AsFd, BorrowedFd, RawFd},
    stdio::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO},
    write,
};

use crate::errno::{die, try_io};
use core::ffi::{CStr, VaList as va_list, c_char, c_double, c_int, c_uint};

pub const EOF: c_int = -1;

pub struct FILE {
    fd: RawFd,
}

impl AsFd for FILE {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.fd) }
    }
}

// Static instances for standard streams
static mut STDIN_IMPL: FILE = FILE { fd: STDIN_FILENO };
static mut STDOUT_IMPL: FILE = FILE { fd: STDOUT_FILENO };
static mut STDERR_IMPL: FILE = FILE { fd: STDERR_FILENO };

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut stdin: *mut FILE = &raw mut STDIN_IMPL;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut stdout: *mut FILE = &raw mut STDOUT_IMPL;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut stderr: *mut FILE = &raw mut STDERR_IMPL;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(format: *const c_char, mut args: ...) -> c_int {
    unsafe { printf_internal(&*stdout, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vprintf(format: *const c_char, args: va_list) -> c_int {
    unsafe { printf_internal(&*stdout, format, args) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf(stream: *mut FILE, format: *const c_char, mut args: ...) -> c_int {
    unsafe { printf_internal(&*stream, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vfprintf(
    stream: *mut FILE,
    format: *const c_char,
    args: va_list,
) -> c_int {
    unsafe { printf_internal(&*stream, format, args) }
}

/*
TODO

While the implementation is meant to be correct it's very inneficient
since it's doing a lot of system calls

Ideally we want to buffer the input but that involves having an allocator,
which we dont yet have, and also some proper memory management to mitigate
at least some very basic buffer overflows vulnerabilities

*/
unsafe fn printf_internal<Fd: AsFd>(fd: Fd, format: *const c_char, mut args: va_list) -> c_int {
    die!(format);

    let str = unsafe { CStr::from_ptr(format) }.to_bytes();

    let mut bytes_iter = str.into_iter().peekable();

    let mut number_buffer = itoa::Buffer::new();
    let mut float_buffer = ryu::Buffer::new();

    let mut written = 0usize;

    'spec_loop: while let Some(byte) = bytes_iter.next() {
        'spec_blk: {
            if *byte == b'%'
                && let Some(spec) = bytes_iter.peek()
            {
                let buf = match *spec {
                    b'd' | b'i' => {
                        let arg: c_int = unsafe { args.arg() };
                        number_buffer.format(arg).as_bytes()
                    }
                    b'u' => {
                        let arg: c_uint = unsafe { args.arg() };
                        number_buffer.format(arg).as_bytes()
                    }
                    b's' => {
                        let arg: *const c_char = unsafe { args.arg() };
                        unsafe { CStr::from_ptr(arg) }.to_bytes()
                    }
                    b'f' => {
                        let arg: c_double = unsafe { args.arg() };
                        float_buffer.format(arg).as_bytes()
                    }
                    _ => {
                        break 'spec_blk;
                    }
                };

                written += try_io!(write_all(&fd, buf));

                bytes_iter.next();
                continue 'spec_loop;
            }
        }

        written += try_io!(write_all(&fd, &[*byte]));
    }

    written as _
}

// FIXME: return a result and then have a shortcircut function
fn write_all<Fd: AsFd>(fd: Fd, mut buf: &[u8]) -> xenia::Result<usize> {
    let mut written = 0usize;

    while !buf.is_empty() {
        match write(&fd, buf) {
            Ok(0) => return Err(Errno::IO),
            Ok(n) => {
                buf = &buf[n..];
                written += n;
            }
            Err(Errno::INTR) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(written)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    die!(s);

    let buf = unsafe { CStr::from_ptr(s) }.to_bytes();

    unsafe {
        try_io!(write_all(&*stdout, buf));
        try_io!(write_all(&*stdout, b"\n"));
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fileno(stream: *mut FILE) -> c_int {
    die!(stream);

    unsafe { (*stream).fd }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn putchar(c: c_int) -> c_int {
    unsafe {
        try_io!(write_all(&*stdout, &[c as u8]));
    }

    0
}
