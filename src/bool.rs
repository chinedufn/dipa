use crate::{CreatePatchTowardsReturn, Diffable, MacroOptimizationHints};

impl<'p> Diffable<'p> for bool {
    // TODO: &bool
    type Diff = bool;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> CreatePatchTowardsReturn<Self::Diff> {
        let did_change = *self != *end_state;
        let hint = MacroOptimizationHints { did_change };

        (*end_state, hint)
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        *self = patch;
    }
}

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
