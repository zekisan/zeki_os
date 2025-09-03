#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn pain(_info: &PanicInfo) -> ! {
	loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	loop {}
}
