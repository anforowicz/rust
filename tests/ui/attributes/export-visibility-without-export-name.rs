// This test verfies that `#[export_visibility = ...]` cannot be used without
// either `#[export_name = ...]` or `#[no_mangle]`.
#![feature(export_visibility)]
#[export_visibility = "hidden"]
pub static TESTED_STATIC: [u8; 6] = *b"foobar";

fn main() {}
