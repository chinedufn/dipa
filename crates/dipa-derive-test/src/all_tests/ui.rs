/// Test compile time errors.
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/all_tests/ui/*.rs");
}

#[test]
fn todo_delete_me() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/all_tests/ui/field_batching_strategy_not_enough_fields.rs");
}
