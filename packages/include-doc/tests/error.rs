#[test]
// Windows path errors have different wording
#[cfg(not(target_os = "windows"))]
fn syntax_error() {
    let t = trybuild::TestCases::new();
    // We can't test much more than this as `trybuild` creates a package in
    // `target`. When it compiles this, `CARGO_MANIFEST_DIR` is set, so
    // `include-doc` won't be able to find our files.
    t.compile_fail("tests/error/fn_body_in_dependencies.rs");
    t.compile_fail("tests/error/syntax_error.rs");
    t.compile_fail("tests/error/missing_file.rs");
}
