#![no_std] // don't link the rust std
#![no_main] // disable all rust-level netry points

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function (keep it as _start)
pub extern "C" fn _start() -> ! {
    // this is the entry point that the linker looks for
    // named _start by default
    println!("Hello World{}", "!");
    panic!("panic gaming");
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
