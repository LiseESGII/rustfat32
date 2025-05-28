#![no_std]      // Désactive la stdlib
#![no_main]     // Désactive le point d’entrée Rust standard

mod boot_sector;


use core::panic::PanicInfo;

/// Handler appelé en cas de panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Point d’entrée appelé par l’assembleur
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Appel du test manuel du Boot Sector
    boot_sector::test_boot_sector_parsing();
    loop {}
}
