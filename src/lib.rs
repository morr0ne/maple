#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(c_variadic)]

use core::{
    arch::naked_asm,
    ffi::{CStr, c_char, c_int},
    ptr::null_mut,
};

use rustix::{
    io::{Errno, write},
    stdio::stdout,
};

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::intrinsics::abort()
}

unsafe extern "C" {
    fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(format: *const c_char, mut args: ...) -> c_int {
    todo!()
}

pub const EOF: c_int = -1;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    if s.is_null() {
        return EOF;
    }

    let stdout = unsafe { stdout() };

    let mut buf = unsafe { CStr::from_ptr(s) }.to_bytes();

    while !buf.is_empty() {
        match write(stdout, buf) {
            Ok(0) => {
                return EOF;
            }
            Ok(n) => buf = &buf[n..],
            Err(Errno::INTR) => {}
            Err(e) => return e.raw_os_error(),
        }
    }

    match write(stdout, b"\n") {
        Ok(0) => EOF,
        Ok(_) => 0,
        Err(e) => e.raw_os_error(),
    }
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    fn entry(_mem: *mut usize) -> ! {
        unsafe { rustix::runtime::exit_group(main(0, null_mut(), null_mut())) }
    }

    naked_asm!(
        "mov rdi, rsp",
        "push rbp",
        "jmp {entry}",
        entry = sym entry,
    );
}
