use crate::DiffPatch;

impl<'p> DiffPatch<'p> for bool {
    // TODO: &bool
    type Diff = bool;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        *end_state
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        *self = patch;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::DiffPatchTestCase;

    #[test]
    fn bool_unchanged() {
        DiffPatchTestCase {
            label: Some("Diff patch same bool"),
            start: true,
            end: &true,
            expected_diff: true,
            expected_serialized_patch_size: 1,
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
        }
        .test();
    }
}
