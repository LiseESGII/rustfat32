#![no_std]      // Désactive la stdlib
#![no_main]     // Désactive le point d’entrée Rust standard

use core::panic::PanicInfo;

/// Handler appelé en cas de panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Point d’entrée appelé par l’assembleur
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
