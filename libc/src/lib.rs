#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(c_variadic)]
#![feature(thread_local)]

pub mod errno;
pub mod mem;
pub mod stdio;

use core::{
    arch::naked_asm,
    ffi::{c_char, c_int},
    ptr::null_mut,
};

use rustix;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::intrinsics::abort()
}

unsafe extern "C" {
    fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
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
