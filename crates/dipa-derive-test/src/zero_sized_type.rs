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

#[cfg(test)]
mod tests {
    use super::*;
    use dipa::{DiffPatchTestCase, MacroOptimizationHints};
    use serde::__private::PhantomData;

    /// Verify that the diff for zero sized types has no size.
    #[test]
    fn zst() {
        let expected_diff = ();
        let expected_serialized_patch_size = 0;

        DiffPatchTestCase {
            label: None,
            start: UnitStruct,
            end: &UnitStruct,
            expected_delta: expected_diff,
            expected_serialized_patch_size,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: EmptyStruct {},
            end: &EmptyStruct {},
            expected_delta: expected_diff,
            expected_serialized_patch_size,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: EmptyTupleStruct {},
            end: &EmptyTupleStruct {},
            expected_delta: expected_diff,
            expected_serialized_patch_size,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: SingleFieldEnum::Foo,
            end: &SingleFieldEnum::Foo,
            expected_delta: expected_diff,
            expected_serialized_patch_size,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();
    }
}
