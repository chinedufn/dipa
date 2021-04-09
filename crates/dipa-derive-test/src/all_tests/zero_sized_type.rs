use dipa::{DipaImplTester, MacroOptimizationHints};

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct UnitStruct;

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct EmptyStruct {}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct EmptyTupleStruct();

// Can't test this since we can't construct it. Just verifying that it compiles.
#[derive(DiffPatch)]
#[allow(unused)]
enum EmptyEnum {}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum SingleFieldEnum {
    Foo,
}

/// Verify that the diff for zero sized types has no size.
#[test]
fn zst() {
    let expected_diff = ();
    let expected_serialized_patch_size = 0;

    DipaImplTester {
        label: None,
        start: UnitStruct,
        end: &UnitStruct,
        expected_delta: expected_diff,
        expected_serialized_patch_size,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: EmptyStruct {},
        end: &EmptyStruct {},
        expected_delta: expected_diff,
        expected_serialized_patch_size,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: EmptyTupleStruct {},
        end: &EmptyTupleStruct {},
        expected_delta: expected_diff,
        expected_serialized_patch_size,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: SingleFieldEnum::Foo,
        end: &SingleFieldEnum::Foo,
        expected_delta: expected_diff,
        expected_serialized_patch_size,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();
}
