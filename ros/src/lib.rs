#![feature(const_fn, lang_items, unique)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate x86;

#[macro_use]
mod vga_buffer;
mod memory;

use memory::{AreaFrameAllocator, FrameAllocator};
use vga_buffer::Writer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
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
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("Memory areas:");

    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

    println!("Kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);

    //panic!("Test");

    let mut frame_allocator = AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start as usize, multiboot_end as usize, memory_map_tag.memory_areas());

    /*println!("{:?}", frame_allocator.allocate_frame());

    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }*/

    memory::test_paging(&mut frame_allocator);

    loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"] extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}
