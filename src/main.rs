#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	loop {}
}
