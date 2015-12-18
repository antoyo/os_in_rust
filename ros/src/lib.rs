#![feature(const_fn, lang_items, unique)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

use vga_buffer::Writer;

#[no_mangle]
pub extern fn rust_main() {
    use core::fmt::Write;

    /*let x = ["Hello", " ", "World", "!"];
    let test = (0..3).flat_map(|x| 0..x).zip(0..);
    let mut a = 42;
    a += 1;*/

    let hello = b"Hello World!";
    let color_byte = 0x1F; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i * 2] = *char_byte;
    }

    // write "Hello World!" to the center of the VGA text buffer
    let buffer_ptr = (0xB8000 + 1988) as *mut _;
    unsafe {
        *buffer_ptr = hello_colored;
    }

    Writer::print_something();

    vga_buffer::WRITER.lock().write_str("Hello again");
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
