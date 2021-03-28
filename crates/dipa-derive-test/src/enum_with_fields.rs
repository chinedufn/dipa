#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum OneVariantStructOneField {
    One { foo: u8 },
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum OneVariantOneTuple {
    One(u8),
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum OneVariantStructTwoFields {
    One { foo: u8, bar: u16 },
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum OneVariantTwoTuple {
    One(u8, u16),
}

enum TwoVariants {
    One,
    Two,
}

enum TwoVariantsOneTuple {
    One(u8),
    Two,
}

enum TwoVariantsOneStruct {
    One { foo: u8 },
    Two,
}

enum TupleVariantTwoFields {
    One,
    Two(u8, u16),
}

enum StructVariantTwoFields {
    One,
    Two { buzz: u8, bazz: u16 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use dipa::private::Diff2;
    use dipa::{patch_ty, DiffPatchTestCase, MacroOptimizationHints};

    /// Verify that we properly handle an enum with a single variant and one piece of data.
    #[test]
    fn single_variant_enum_single_data() {
        DiffPatchTestCase {
            label: None,
            start: OneVariantStructOneField::One { foo: 2 },
            end: &OneVariantStructOneField::One { foo: 2 },
            expected_diff: 2,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
            patch_type: patch_ty::<u8>(),
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: OneVariantStructOneField::One { foo: 1 },
            end: &OneVariantStructOneField::One { foo: 5 },
            expected_diff: 5,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
            patch_type: patch_ty::<u8>(),
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: OneVariantOneTuple::One(2),
            end: &OneVariantOneTuple::One(2),
            expected_diff: 2,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
            patch_type: patch_ty::<u8>(),
        }
        .test();

        DiffPatchTestCase {
            label: None,
            start: OneVariantOneTuple::One(1),
            end: &OneVariantOneTuple::One(5),
            expected_diff: 5,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
            patch_type: patch_ty::<u8>(),
        }
        .test();
    }

    /// Verify that we properly handle an enum with a single variant and two pieces of data.
    #[test]
    fn single_variant_enum_with_two_data() {
        DiffPatchTestCase {
            label: Some("Struct no change"),
            start: OneVariantStructTwoFields::One { foo: 0, bar: 0 },
            end: &OneVariantStructTwoFields::One { foo: 0, bar: 0 },
            expected_diff: Diff2::NoChange,
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
            patch_type: patch_ty::<Diff2<u8, Option<u16>>>(),
        }
        .test();

        DiffPatchTestCase {
            label: Some("Struct Change_0"),
            start: OneVariantStructTwoFields::One { foo: 0, bar: 0 },
            end: &OneVariantStructTwoFields::One { foo: 5, bar: 0 },
            expected_diff: Diff2::Change_0(5),
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
            patch_type: patch_ty::<Diff2<u8, Option<u16>>>(),
        }
        .test();

        DiffPatchTestCase {
            label: Some("Struct Change_1"),
            start: OneVariantTwoTuple::One(2, 2),
            end: &OneVariantTwoTuple::One(2, 5),
            expected_diff: Diff2::Change_1(Some(5)),
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
            patch_type: patch_ty::<Diff2<u8, Option<u16>>>(),
        }
        .test();

        DiffPatchTestCase {
            label: Some("Struct Change_0_1"),
            start: OneVariantTwoTuple::One(2, 2),
            end: &OneVariantTwoTuple::One(5, 6),
            expected_diff: Diff2::Change_0_1(5, Some(6)),
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
            patch_type: patch_ty::<Diff2<u8, Option<u16>>>(),
        }
        .test();
    }
}
