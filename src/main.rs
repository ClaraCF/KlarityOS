#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod io;
use io::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Welcome to KlarityOS
    println(b"Welcome to KlarityOS!", Colors::Purple).expect("Print welcome message");

    loop{}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
