#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::Writer::print_something();

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}
