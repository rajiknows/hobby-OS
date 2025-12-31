#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

mod macros;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// static HELLO: &[u8] = b"Hello world";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // clear_screen();
    // println!("Hello World{}", "!");
    // println!("sussy baka");
    vga_buffer::WRITER.lock().write_str("hello raj!").unwrap();
    panic!("Some panic message");
    loop {}
}
