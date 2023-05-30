#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod console;
use console::{Colors};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the console
    let mut console = console::Console::new();

    // Welcome to KlarityOS
    console.println("Este es un texto increiblemente largo que deberia exceder el limite de caracteres de una sola linea y por lo tanto deberia automaticamente ocupar varias", Some(Colors::Purple));

    loop{}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
