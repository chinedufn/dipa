use crate::{number_diff_impl_u8_or_i8, number_patch_impl_u8_or_i8};
use crate::{CreatePatchTowardsReturn, MacroOptimizationHints, Patchable};

number_diff_impl_u8_or_i8!(bool);
number_patch_impl_u8_or_i8!(bool);

#[cfg(test)]
mod tests {
    use crate::dipa_impl_tester::DiffPatchTestCase;
    use crate::test_utils::{
        macro_optimization_hint_did_change, macro_optimization_hint_unchanged,
    };

    #[test]
    fn bool_unchanged() {
        DiffPatchTestCase {
            label: Some("Diff patch same bool"),
            start: true,
            end: &true,
            expected_diff: true,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn bool_changed() {
        DiffPatchTestCase {
            label: Some("Diff patch different bool"),
            start: true,
            end: &false,
            expected_diff: false,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }
}
