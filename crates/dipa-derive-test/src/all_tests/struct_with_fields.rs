//! We test every combination of changing/not-changing for the fields in structs with one, two
//! and three fields.
//!
//! That gives us confident that our logic applies to `n` fields, so we don't test every
//! possible change/no-change combination for structs with 4+ fields. Instead, for structs with
//! 4+ fields we simply verify that they compile when annotated with `#[derive(Dipa)]`.

use dipa::private::{Diff2, Diff3};
use dipa::{DiffPatchTestCase, MacroOptimizationHints};

// We can use `DiffN` types to support up to some max limit of fields. Lets call that 5 for now.
// If a struct had 51 fields, we would need our diff type to have 10 Diff5 and then 1 Diff1.
//
// Ideally if the struct has not changed it takes up 1 byte for the diff. This would require a tree
// of DiffN's. So a Diff5 which contains 5 Diff5 which then contain 5 Diff5s each and then we
// spread our 51 fields across.
// This would mean that if a field changed it would be a few levels deep in the enum so we would
// need to have 1 byte per level deep to encode the variant. This sounds very complex, which is not
// ideal.
//
// Another option is to have an `[Diff5; 10]`, but that would not work since each Diff5 is a
// different type.
//
// Another option is to just have our diff struct have a field for each field in the original
// struct. This would be 1 byte overhead for each field that hasn't changed as opposed to combining
// them into Diff5's
//
// Another option is a diff struct with one field for every 5 fields in the original struct.
//
// Best would be to implement all of these and then expose attributes to decide on which strategy
// is used for the container. Fail at compile time if the struct has more than 5 fields and tell
// the user to pick a different strategy. And link to documentation about the trade-offs between
// the options.
//
// So, if a variant has more than 5 fields and no attribute we have a UI error that we can test
// using try build.
// Same for a struct with more than 5 fields.
//
// We can add attributes for all of the strategies but have the harder ones `unimplemented!()` for
// now.
//
// `#[dipa(max_fields_per_batch = 5, field_batching_strategy = "one_batch")]` <--- Default
// `#[dipa(max_fields_per_batch = 5, field_batching_strategy = "many_batches")]` <--- Struct with multiple Diff5 and
// one DiffN
// `#[dipa(max_delta_batch = 5, field_batching_strategy = "no_batching")]` <--- Struct with one delta per
// original field.
//
// 0. (DONE) Add `DeltaN` and `DeltaNOwned` to the build script. First has Serialize, last has Deserialize.
//    Then try different max limits in build.rs to see where compile times get noticably slower.
//
// 1. (DONE) Add `max_delta_batch = N` trybuild test that fails if `N > 7`. Explaining that the impact
//    on compile times increases exponentially so this is the limit. Link to a GitHub issue for
//    discussion.
//
// 2. (DONE) Add trybuild test that fails if number if fields is greater than `max_delta_batch` with
//    error message linking to docs on delta_strategy and also printing options in case they are
//    offline.
//
// 3. (DONE) Add UI test for an invalid delta_strategy and have it link to the documentation.
//    https://chinedufn/github.io/dipa/using-derive/delta-encoding-optimizations/index.html
//
// 4. (DONE) Add a UI test for a delta_strategy on a struct with less than 2 fields.
//
// 5. Make structs generate their own DeltaN types instead of using the generated DeltaN to avoid
//     generics to help compile times.
//
// 6. Add dipa derive test in `mod field_batching_strategy` where we add the
//     no_batching on a struct and verify that the diff type is a struct with the correctly named
//     fields (use a struct with 10 fields and verify that each one shows up in the delta type
//      and the DeltaOwned type by creating a Delta and DeltaOwned instance).
//
// 7. Reduce DiffN to 4 and use it for up to 4 Tuples. Leave TODO to allow larger implementations
//     via feature flag.
//
// 8. In that same file leave a commented out TODO that we should add a dipa derive test where we
//     use no_batching strategy on an enum variant.
//
// 9. Add `many_batches` delta strategy but make it unimplemented!
//
// 10. Add book documentation to the attributes chapter for `max_fields_per_batch`
//
// 11. Add book documentation to the attributes chapter for `field_batching_strategy` where we discuss each
//    strategy.
//
// 12. Leave TODO in dipa's Cargo.toml to add feature flags that lets you unlock delta_batch_6 and
//    delta_batch_7 delta_batch_8
//    and delta_batch_9 which makes the build script generate Delta6 and Delta7 Delta8 and Delta9
//    types.
//
// 13. Make `max_delta_batch = N` respected in `all_combinations.rs`. Add dipa-derive-test to verify
//    that this works. Where we create a struct with 7 fields and set the max to 7. Perhaps in a
//    `mod max_delta_batch`
//
// 14. Get dipa working in Akigi in a separate branch and do a before and after on debug build
//      times.
//
// 15. Error if field batching strategy is used on an enum container indicating that it can only
//     be used on enum variants.
//
// 16. unimplemented!() for using field batching strategy on enum variants with message linking to
//     an issue.

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

//

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct FiveFields {
    field1: u8,
    field2: u16,
    field3: u32,
    field4: u64,
    field5: u128,
}

#[derive(Debug, DiffPatch, Eq, PartialEq, Serialize)]
struct FiveFieldsTuple(u8, u16, u32, u64, u128);

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
