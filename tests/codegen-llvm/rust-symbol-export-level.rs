// Verifies that `#[rust_symbol_export_level = ...]` can be used to override the visibility that is
// normally implied by `#[export_name]` or `#[no_mangle]`.
//
// High-level test expectations for items with `#[export_name = ...]`
// (or with `#[no_mangle]`) and:
//
// * Without `#[rust_symbol_export_level]` => public
// * With `#[rust_symbol_export_level]` => value inherited from the target
//   platform or from the `-Zdefault-visibility=...` command-line flag
//   (this expectation depends on whether the `HIDDEN` vs `PROTECTED`
//   test revision is used).
//
// Note that what we call "public" in the expectations above is also referred
// to as "default" in LLVM docs - see
// https://llvm.org/docs/LangRef.html#visibility-styles

//@ revisions:HIDDEN PROTECTED
//@[HIDDEN] compile-flags: -Zdefault-visibility=hidden
//@[PROTECTED] compile-flags: -Zdefault-visibility=protected

// This test focuses on rlib to exercise the scenario described in
// https://github.com/rust-lang/rust/issues/73958#issuecomment-2891711649
#![crate_type = "rlib"]
#![feature(rust_symbol_export_level)]

// Exact LLVM IR differs depending on the target triple (e.g. `hidden constant`
// vs `internal constant` vs `constant`).  Because of this, we only apply the
// specific test expectations below to one specific target triple.  If needed,
// additional targets can be covered by adding copies of this test file with
// a different `only-X` directive.
//
//@     only-x86_64-unknown-linux-gnu

///////////////////////////////////////////////////////////////////////
// The tests below focus on how `#[rust_symbol_export_level]` works for
// a `static`.  The tests are based on similar tests in
// `tests/codegen/default-visibility.rs`

#[unsafe(export_name = "test_static_no_attr")]
pub static TEST_STATIC_NO_ATTR: [u8; 7] = *b"static1";

#[unsafe(export_name = "test_static_rust_level")]
#[rust_symbol_export_level]
pub static TESTED_STATIC_ATTR_RUST_LEVEL: [u8; 7] = *b"static2";

#[unsafe(no_mangle)]
pub static test_static_no_mangle_no_attr: [u8; 10] = *b"no_mangle1";

#[unsafe(no_mangle)]
#[rust_symbol_export_level]
pub static test_static_no_mangle_rust_level: [u8; 10] = *b"no_mangle2";

// HIDDEN: @test_static_no_attr = local_unnamed_addr constant
// HIDDEN: @test_static_rust_level = hidden local_unnamed_addr constant
// HIDDEN: @test_static_no_mangle_no_attr = local_unnamed_addr constant
// HIDDEN: @test_static_no_mangle_rust_level = hidden local_unnamed_addr constant

// PROTECTED: @test_static_no_attr = local_unnamed_addr constant
// PROTECTED: @test_static_rust_level = protected local_unnamed_addr constant

///////////////////////////////////////////////////////////////////////
// The tests below focus on how `#[rust_symbol_export_level]` works for
// a `fn`.
//
// The tests below try to mimics how `cxx` exports known/hardcoded helpers (e.g.
// `cxxbridge1$string$drop` [1]) as well as build-time-generated thunks (e.g.
// `serde_json_lenient$cxxbridge1$decode_json` from https://crbug.com/418073233#comment7).
//
// We use `line!()` to ensure that each function has a unique body
// (and therefore that the functions won't get folded together).
//
// [1]
// https://github.com/dtolnay/cxx/blob/ebdd6a0c63ae10dc5224ed21970b7a0504657434/src/symbols/rust_string.rs#L83-L86

#[unsafe(export_name = "test_fn_no_attr")]
unsafe extern "C" fn test_fn_no_attr() -> u32 {
    line!()
}

#[unsafe(export_name = "test_fn_rust_level")]
#[rust_symbol_export_level]
unsafe extern "C" fn test_fn_rust_level() -> u32 {
    line!()
}

// HIDDEN: define noundef i32 @test_fn_no_attr
// HIDDEN: define hidden noundef i32 @test_fn_rust_level

// PROTECTED: define noundef i32 @test_fn_no_attr
// PROTECTED: define protected noundef i32 @test_fn_rust_level
