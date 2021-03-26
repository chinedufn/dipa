use crate::{
    number_diff_impl_option_wrapped, number_diff_impl_u8_or_i8, number_patch_impl_option_wrapped,
    number_patch_impl_u8_or_i8,
};
use crate::{CreatePatchTowardsReturn, MacroOptimizationHints};

number_diff_impl_u8_or_i8!(u8);
number_patch_impl_u8_or_i8!(u8);
number_diff_impl_u8_or_i8!(i8);
number_patch_impl_u8_or_i8!(i8);

number_diff_impl_option_wrapped!(u16);
number_patch_impl_option_wrapped!(u16);
number_diff_impl_option_wrapped!(i16);
number_patch_impl_option_wrapped!(i16);

number_diff_impl_option_wrapped!(u32);
number_patch_impl_option_wrapped!(u32);
number_diff_impl_option_wrapped!(i32);
number_patch_impl_option_wrapped!(i32);

number_diff_impl_option_wrapped!(u64);
number_patch_impl_option_wrapped!(u64);
number_diff_impl_option_wrapped!(i64);
number_patch_impl_option_wrapped!(i64);

number_diff_impl_option_wrapped!(u128);
number_patch_impl_option_wrapped!(u128);
number_diff_impl_option_wrapped!(i128);
number_patch_impl_option_wrapped!(i128);

#[cfg(test)]
mod tests_signed {

    use crate::dipa_impl_tester::DiffPatchTestCase;
    use crate::test_utils::{
        macro_optimization_hint_did_change, macro_optimization_hint_unchanged,
    };

    #[test]
    fn diff_patch_u8_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u8"),
            start: 0u8,
            end: &0u8,
            expected_diff: 0,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_u8_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u8"),
            start: 0u8,
            end: &2u8,
            expected_diff: 2,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u16"),
            start: 0u16,
            end: &0u16,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_u16_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u16"),
            start: 0u16,
            end: &2u16,
            expected_diff: Some(2),
            expected_serialized_patch_size: 3,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u32"),
            start: 0u32,
            end: &0u32,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u32s"),
            start: 0u32,
            end: &1u32,
            expected_diff: Some(1),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u64"),
            start: 0u64,
            end: &0u64,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u64s"),
            start: 0u64,
            end: &1u64,
            expected_diff: Some(1),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same u128"),
            start: 0u128,
            end: &0u128,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different u128s"),
            start: 0u128,
            end: &1u128,
            expected_diff: Some(1),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }
}

#[cfg(test)]
mod tests_unsigned {

    use crate::dipa_impl_tester::DiffPatchTestCase;
    use crate::test_utils::{
        macro_optimization_hint_did_change, macro_optimization_hint_unchanged,
    };

    #[test]
    fn diff_patch_i8_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i8"),
            start: 0i8,
            end: &0i8,
            expected_diff: 0,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_i8_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i8"),
            start: 0i8,
            end: &1i8,
            expected_diff: 1,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i16"),
            start: 0i16,
            end: &0i16,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_i16_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i16"),
            start: 0i16,
            end: &2i16,
            expected_diff: Some(2),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_32_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i32"),
            start: 0i32,
            end: &0i32,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_32_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i32s"),
            start: 0i32,
            end: &1i32,
            expected_diff: Some(1),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_64_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i64"),
            start: 0i64,
            end: &0i64,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_64_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i64s"),
            start: 0i64,
            end: &1i64,
            expected_diff: Some(1),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    #[test]
    fn diff_patch_128_same() {
        DiffPatchTestCase {
            label: Some("Diff patch same i128"),
            start: 0i128,
            end: &0i128,
            expected_diff: None,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn diff_patch_128_different() {
        DiffPatchTestCase {
            label: Some("Diff patch different i128s"),
            start: 0i128,
            end: &1i128,
            expected_diff: Some(1),
            expected_serialized_patch_size: 2,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }
}
