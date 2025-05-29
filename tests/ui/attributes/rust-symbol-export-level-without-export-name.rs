// This test verfies that `#[rust_symbol_export_level]` cannot be used without
// either `#[export_name = ...]` or `#[no_mangle]`.
#![feature(rust_symbol_export_level)]
#[rust_symbol_export_level]
//~^ ERROR: `#[rust_symbol_export_level]` will be ignored
pub static TESTED_STATIC: [u8; 6] = *b"foobar";

fn main() {}
