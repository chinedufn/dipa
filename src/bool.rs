use crate::{number_diff_impl_u8_or_i8, number_patch_impl_u8_or_i8};
use crate::{CreatePatchTowardsReturn, MacroOptimizationHints};

number_diff_impl_u8_or_i8!(bool, bool);
number_patch_impl_u8_or_i8!(bool, bool);

#[cfg(test)]
mod tests {
    use crate::dipa_impl_tester::DipaImplTester;
    use crate::test_utils::{
        macro_optimization_hint_did_change, macro_optimization_hint_unchanged,
    };

    #[test]
    fn bool_unchanged() {
        DipaImplTester {
            label: Some("Diff patch same bool"),
            start: &mut true,
            end: &true,
            expected_delta: true,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    #[test]
    fn bool_changed() {
        DipaImplTester {
            label: Some("Diff patch different bool"),
            start: &mut true,
            end: &false,
            expected_delta: false,
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }
}
