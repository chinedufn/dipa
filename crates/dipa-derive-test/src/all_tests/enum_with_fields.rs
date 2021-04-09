use dipa::{DipaImplTester, MacroOptimizationHints};

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum OneVariantStructOneField {
    One { foo: u8 },
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
enum OneVariantOneTuple {
    One(u8),
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
#[dipa(diff_derives = "Debug, PartialEq")]
enum OneVariantStructTwoFields {
    One { foo: u8, bar: u16 },
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
#[dipa(diff_derives = "Debug, PartialEq")]
enum OneVariantTwoTuple {
    One(u8, u16),
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize, Deserialize)]
enum TwoVariants {
    One,
    Two,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize, Deserialize)]
#[dipa(diff_derives = "Debug, PartialEq")]
enum TwoVariantsOneTuple {
    One(u8),
    Two,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize, Deserialize)]
#[dipa(diff_derives = "Debug, PartialEq")]
enum TwoVariantsOneStruct {
    One { foo: u8 },
    Two,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize, Deserialize)]
#[dipa(diff_derives = "Debug, PartialEq")]
enum TwoVariantsTupleTwoFields {
    One,
    Two(u8, u16),
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize, Deserialize)]
#[dipa(diff_derives = "Debug, PartialEq")]
enum TwoVariantsStructTwoFields {
    One,
    Two { buzz: u8, bazz: u16 },
}

/// Verify that we properly handle an enum with a single variant and one piece of data.
#[test]
fn single_variant_enum_single_data() {
    DipaImplTester {
        label: None,
        start: OneVariantStructOneField::One { foo: 2 },
        end: &OneVariantStructOneField::One { foo: 2 },
        expected_delta: 2,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: OneVariantStructOneField::One { foo: 1 },
        end: &OneVariantStructOneField::One { foo: 5 },
        expected_delta: 5,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();

    DipaImplTester {
        label: None,
        start: OneVariantOneTuple::One(2),
        end: &OneVariantOneTuple::One(2),
        expected_delta: 2,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: OneVariantOneTuple::One(1),
        end: &OneVariantOneTuple::One(5),
        expected_delta: 5,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();
}

/// Verify that we properly handle an enum with a single variant and two pieces of data.
#[test]
fn single_variant_enum_with_two_data() {
    DipaImplTester {
        label: Some("Struct no change"),
        start: OneVariantStructTwoFields::One { foo: 0, bar: 0 },
        end: &OneVariantStructTwoFields::One { foo: 0, bar: 0 },
        expected_delta: OneVariantStructTwoFieldsDelta::NoChange,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: Some("Struct Change_0"),
        start: OneVariantStructTwoFields::One { foo: 0, bar: 0 },
        end: &OneVariantStructTwoFields::One { foo: 5, bar: 0 },
        expected_delta: OneVariantStructTwoFieldsDelta::Change_0(5),
        expected_serialized_patch_size: 2,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();

    DipaImplTester {
        label: Some("Struct Change_1"),
        start: OneVariantTwoTuple::One(2, 2),
        end: &OneVariantTwoTuple::One(2, 5),
        expected_delta: OneVariantTwoTupleDelta::Change_1(Some(5)),
        expected_serialized_patch_size: 3,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();

    DipaImplTester {
        label: Some("Struct Change_0_1"),
        start: OneVariantTwoTuple::One(2, 2),
        end: &OneVariantTwoTuple::One(5, 6),
        expected_delta: OneVariantTwoTupleDelta::Change_0_1(5, Some(6)),
        expected_serialized_patch_size: 4,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();
}

/// Verify that enum diffs are one byte if none of the variants of an enum contain any data.
#[test]
fn empty_variants_single_byte_diffs() {
    DipaImplTester {
        label: Some("Enum no data same"),
        start: TwoVariants::One,
        end: &TwoVariants::One,
        expected_delta: TwoVariants::One,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: Some("Enum no data different"),
        start: TwoVariants::One,
        end: &TwoVariants::Two,
        expected_delta: TwoVariants::Two,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();
}

/// Verify that we can diff an enum with two variants, one of which contains data.
#[test]
fn two_variants_one_tuple() {
    DipaImplTester {
        label: None,
        start: TwoVariantsOneTuple::One(5),
        end: &TwoVariantsOneTuple::One(5),
        expected_delta: TwoVariantsOneTupleDelta::OneNoChange,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: TwoVariantsOneTuple::Two,
        end: &TwoVariantsOneTuple::Two,
        expected_delta: TwoVariantsOneTupleDelta::TwoNoChange,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: false },
    }
    .test();

    DipaImplTester {
        label: None,
        start: TwoVariantsOneTuple::One(5),
        end: &TwoVariantsOneTuple::Two,
        expected_delta: TwoVariantsOneTupleDelta::ChangedToVariantTwo,
        expected_serialized_patch_size: 1,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();

    DipaImplTester {
        label: None,
        start: TwoVariantsOneTuple::Two,
        end: &TwoVariantsOneTuple::One(5),
        expected_delta: TwoVariantsOneTupleDelta::ChangedToVariantOne(&5),
        expected_serialized_patch_size: 2,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();

    DipaImplTester {
        label: None,
        start: TwoVariantsOneTuple::One(5),
        end: &TwoVariantsOneTuple::One(10),
        expected_delta: TwoVariantsOneTupleDelta::OneChange_0(10),
        expected_serialized_patch_size: 2,
        expected_macro_hints: MacroOptimizationHints { did_change: true },
    }
    .test();
}
