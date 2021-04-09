#[derive(DiffPatch)]
#[dipa(
    field_batching_strategy = "no_batching",
    diff_derives = "Debug, PartialEq"
)]
#[derive(Debug, PartialEq)]
struct NoBatching {
    field_a: u16,
    field_b: u32,
}

// Verifies that with the no batching strategy we aren't limited on field count.
// This differs from the OneBatch strategy which has a default limit of around 5 fields.
#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "no_batching")]
#[derive(Debug, PartialEq)]
#[rustfmt::skip]
struct NoBatchingManyFields {
    f1: (), f2: (), f3: (), f4: (), f5: (),
    f6: (), f7: (), f8: (), f9: (), f10: (),
    f11: (), f12: (), f13: (), f14: (), f15: (),
}

// TODO: Add test where we use no batching strategy on an enum variant.

#[cfg(test)]
mod tests {
    use super::*;
    use dipa::{DipaImplTester, MacroOptimizationHints};

    /// Verify that the delta types for a struct with no field batching are properly created.
    /// The no_batching strategy creates a delta struct with one field per original field, so
    /// here we very that that is the case.
    #[test]
    fn no_batching_delta() {
        let _ = NoBatchingDelta {
            field_a: Some(0u16),
            field_b: Some(0u32),
        };

        let _ = NoBatchingDeltaOwned {
            field_a: Some(0u16),
            field_b: Some(0u32),
        };
    }

    /// Verify that we can properly diff and patch a type that uses the no batching strategy.
    #[test]
    fn diff_patch_no_batching_strategy() {
        DipaImplTester {
            label: None,
            start: &mut NoBatching {
                field_a: 1,
                field_b: 2,
            },
            end: &NoBatching {
                field_a: 1,
                field_b: 2,
            },
            expected_delta: NoBatchingDelta {
                field_a: None,
                field_b: None,
            },
            expected_serialized_patch_size: 2,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DipaImplTester {
            label: None,
            start: &mut NoBatching {
                field_a: 1,
                field_b: 2,
            },
            end: &NoBatching {
                field_a: 15,
                field_b: 16,
            },
            expected_delta: NoBatchingDelta {
                field_a: Some(15),
                field_b: Some(16),
            },
            expected_serialized_patch_size: 4,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}
