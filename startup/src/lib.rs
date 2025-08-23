#![no_std]
#![feature(linkage)]

use core::{
    arch::naked_asm,
    ffi::{c_char, c_int},
};

use xenia::exit_group;

#[panic_handler]
#[linkage = "weak"] // Nothing can panic here but rust won't compile without defining the symbol
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

unsafe extern "C" {
    safe fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
}

fn entry(stack: *mut usize) -> ! {
    if stack.is_null() {
        exit_group(1) // Something has gone horrifically wrong :(
    }

    let (argc, argv, envp) = unsafe {
        let argc = *stack as c_int;
        let argv = stack.offset(1).cast::<*mut c_char>();
        let envp = argv.offset(argc as isize + 1);

        (argc, argv, envp)
    };

    xenia::exit_group(main(argc, argv, envp))
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        "xor rbp, rbp", // Clear frame pointer as required by the x86_64 abi
        "mov rdi, rsp", // Pass the stack pointer to the enty function
        "and rsp, -16", // Make sure the stack is 16 byte aligned
        "call {entry}", // Use call instead of jmp to preserve a stack trace
        entry = sym entry,
    );
}
