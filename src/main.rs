// required for defining panic handler
#![feature(lang_items)]
// don't link Rust stdlib
#![no_std]
// disable Rust-level entry points
// because without stdlib we can't access C or Rust runtime
#![no_main]

#[lang = "panic_fmt"] // define a function to be called on system panic
#[no_mangle] 
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32
) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// disable function name mangle/hashing
// because linker expects function name `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *const u8 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
