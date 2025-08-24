#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]

extern crate startup;

use core::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char, _envp: *mut *mut c_char) -> c_int {
    0
}
