// required for defining panic handler
#![feature(lang_items)]
// don't link Rust stdlib
#![no_std]
// disable Rust-level entry points
// because without stdlib we can't access C or Rust runtime
#![no_main]

#[lang = "panic_fmt"] // define a function to be called on system panic
#[no_mangle] 
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> !
{
    loop {}
}

// disable function name mangle/hashing
// because linker expects function name `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
