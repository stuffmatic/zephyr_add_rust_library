#![cfg_attr(not(feature = "std"), no_std)]

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(feature = "std"))] // https://github.com/rust-lang/rust-analyzer/issues/4490
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(feature = "c_api")]
#[no_mangle]
pub fn rust_lib_1_add(a: i32, b: i32) -> i32 {
    a + b
}