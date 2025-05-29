// This test verfies that `#[rust_symbol_export_level]` cannot be used without
// opting into the corresponding unstable feature via
// `#![feature(rust_symbol_export_level)]`.
#[rust_symbol_export_level] //~ ERROR: `#[rust_symbol_export_level]` is currently unstable
// `#[export_name = ...]` is present to avoid hitting the following error:
// ... will be ignored without `export_name`, `no_mangle`, or similar attribute
#[unsafe(export_name = "exported_static")]
pub static TESTED_STATIC: [u8; 6] = *b"foobar";

fn main() {}
