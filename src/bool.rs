use crate::DiffPatch;

impl DiffPatch for bool {
    type Patch = bool;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        *end_state
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        *self = patch;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::DiffPatchTestCase;

    #[test]
    fn bool_unchanged() {
        DiffPatchTestCase {
            desc: "Diff patch same bool",
            start: true,
            end: true,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn bool_changed() {
        DiffPatchTestCase {
            desc: "Diff patch different bool",
            start: true,
            end: false,
            expected_serialized_patch_size: 1,
        }
        .test();
    }
}
