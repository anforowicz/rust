// This test verfies that `#[export_visibility = ...]` will report an error
// when the argument cannot be parsed as a `SymbolVisibility`.
#![feature(export_visibility)]
#[export_visibility = "unrecognized spec"]
pub static TESTED_STATIC: [u8; 6] = *b"foobar";

fn main() {}
