#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct UnitStruct;

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct EmptyStruct {}

#[cfg(test)]
mod tests {
    use super::*;
    use dipa::{DiffPatchTestCase, MacroOptimizationHints};

    /// Verify that the diff for zero sized types has no size.
    #[test]
    fn zst() {
        let expected_diff = ();
        let expected_serialized_patch_size = 0;

        DiffPatchTestCase {
            label: None,
            start: UnitStruct,
            end: &UnitStruct,
            expected_diff,
            expected_serialized_patch_size,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: EmptyStruct {},
            end: &EmptyStruct {},
            expected_diff,
            expected_serialized_patch_size,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();
    }
}
