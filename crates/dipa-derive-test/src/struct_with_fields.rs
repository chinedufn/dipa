//! We test every combination of changing/not-changing for the fields in structs with one, two
//! and three fields.
//!
//! That gives us confident that our logic applies to `n` fields, so we don't test every
//! possible change/no-change combination for structs with 4+ fields. Instead, for structs with
//! 4+ fields we simply verify that they compile when annotated with `#[derive(Dipa)]`.

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct OneField {
    field1: u8,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct OneFieldTuple(u8);

//

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct TwoFields {
    field1: u8,
    field2: u16,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct TwoFieldsTuple(u8, u16);

//

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct ThreeFields {
    field1: u8,
    field2: u16,
    field3: u32,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct ThreeFieldsTuple(u8, u16, u32);

//

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct FourFields {
    field1: u8,
    field2: u16,
    field3: u32,
    field4: u64,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct FourFieldsTuple(u8, u16, u32, u64);

#[cfg(test)]
mod tests {

    use super::*;
    use dipa::private::{Diff2, Diff3};
    use dipa::{DiffPatchTestCase, MacroOptimizationHints};

    /// Verify that we can generate a diff/patch for structs with one field.
    #[test]
    fn structs_with_one_field() {
        DiffPatchTestCase {
            label: None,
            start: OneField { field1: 1 },
            end: &OneField { field1: 30 },
            expected_delta: 30,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: OneField { field1: 1 },
            end: &OneField { field1: 1 },
            expected_delta: 1,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: OneFieldTuple(1),
            end: &OneFieldTuple(30),
            expected_delta: 30,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: OneFieldTuple(1),
            end: &OneFieldTuple(1),
            expected_delta: 1,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();
    }

    /// Verify that we can generate a diff/patch for structs with two fields.
    #[test]
    fn structs_with_two_fields() {
        DiffPatchTestCase {
            label: None,
            start: TwoFields {
                field1: 2,
                field2: 2,
            },
            end: &TwoFields {
                field1: 2,
                field2: 2,
            },
            expected_delta: Diff2::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: TwoFields {
                field1: 2,
                field2: 2,
            },
            end: &TwoFields {
                field1: 50,
                field2: 2,
            },
            expected_delta: Diff2::Change_0(50),
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: TwoFields {
                field1: 2,
                field2: 2,
            },
            end: &TwoFields {
                field1: 2,
                field2: 50,
            },
            expected_delta: Diff2::Change_1(Some(50)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: TwoFields {
                field1: 2,
                field2: 2,
            },
            end: &TwoFields {
                field1: 10,
                field2: 50,
            },
            expected_delta: Diff2::Change_0_1(10, Some(50)),
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        //

        DiffPatchTestCase {
            label: None,
            start: TwoFieldsTuple(2, 2),
            end: &TwoFieldsTuple(2, 2),
            expected_delta: Diff2::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: TwoFieldsTuple(2, 2),
            end: &TwoFieldsTuple(50, 2),
            expected_delta: Diff2::Change_0(50),
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: TwoFieldsTuple(2, 2),
            end: &TwoFieldsTuple(2, 50),
            expected_delta: Diff2::Change_1(Some(50)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: TwoFieldsTuple(2, 2),
            end: &TwoFieldsTuple(10, 50),
            expected_delta: Diff2::Change_0_1(10, Some(50)),
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }

    /// Verify that we can generate a diff/patch for structs with three fields.
    #[test]
    fn structs_with_three_fields() {
        DiffPatchTestCase {
            label: Some("No Change"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            expected_delta: Diff3::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: Some("0"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 5,
                field2: 2,
                field3: 2,
            },
            expected_delta: Diff3::Change_0(5),
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("1"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 2,
                field2: 5,
                field3: 2,
            },
            expected_delta: Diff3::Change_1(Some(5)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("2"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 2,
                field2: 2,
                field3: 5,
            },
            expected_delta: Diff3::Change_2(Some(5)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("0 1"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 5,
                field2: 6,
                field3: 2,
            },
            expected_delta: Diff3::Change_0_1(5, Some(6)),
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("0 2"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 5,
                field2: 2,
                field3: 6,
            },
            expected_delta: Diff3::Change_0_2(5, Some(6)),
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
        DiffPatchTestCase {
            label: Some("1 2"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 2,
                field2: 5,
                field3: 6,
            },
            expected_delta: Diff3::Change_1_2(Some(5), Some(6)),
            expected_serialized_patch_size: 5,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();

        DiffPatchTestCase {
            label: Some("0 1 2"),
            start: ThreeFields {
                field1: 2,
                field2: 2,
                field3: 2,
            },
            end: &ThreeFields {
                field1: 5,
                field2: 6,
                field3: 7,
            },
            expected_delta: Diff3::Change_0_1_2(5, Some(6), Some(7)),
            expected_serialized_patch_size: 6,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}
