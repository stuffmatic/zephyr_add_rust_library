#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))] // https://github.com/rust-lang/rust-analyzer/issues/4490
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub fn rust_lib_2_mul(a: i32, b: i32) -> i32 {
    a * b
}