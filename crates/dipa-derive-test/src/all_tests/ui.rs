/// Test compile time errors.
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/all_tests/ui/*.rs");
}
