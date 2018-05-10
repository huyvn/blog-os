// silence warning in test
#![cfg_attr(test, allow(dead_code, unused_macros))]
// required for defining panic handler
#![feature(lang_items)]
// don't link Rust stdlib
#![no_std]
// disable Rust-level entry points
// because without stdlib we can't access C or Rust runtime
// use cfg_attr because we want main during test
#![cfg_attr(not(test), no_main)]
#![feature(const_fn)]

// include std lib for test
#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate array_init;

#[macro_use]
extern crate lazy_static;

extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[cfg(not(test))] // only compile when the test flag is not set
#[lang = "panic_fmt"] // define a function to be called on system panic
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

#[cfg(not(test))]
// disable function name mangle/hashing
// because linker expects function name `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
