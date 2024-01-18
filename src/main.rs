#![no_std] // don't link the rust std
#![no_main] // disable all rust-level netry points
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function (keep it as _start)
pub extern "C" fn _start() -> ! {
    // this is the entry point that the linker looks for
    // named _start by default
    println!("Hello World{}", "!");

    rustos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rustos::hlt_loop();
}

// this function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
