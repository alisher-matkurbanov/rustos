#![no_std] // we say compiler don't use standard library
#![no_main] // we say to disable all Rust-level entry points

use core::panic::PanicInfo;

// implement our own panic (remember - we don't use std lib)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// turn off compiler's name changing
// (compiler will name next fn as '_start' not '_ZN3blog_os4_start7hb173fedf945531caE')
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}