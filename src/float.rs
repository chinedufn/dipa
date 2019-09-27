use crate::DiffPatch;

impl DiffPatch for f32 {
    type Patch = Option<f32>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl DiffPatch for f64 {
    type Patch = Option<f64>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Patch) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::DiffPatchTestCase;

    /// We wrap f32 so that we can impl Eq and PartialEq
    #[derive(Debug, Deserialize, Serialize, Copy, Clone)]
    struct F32TestWrapper(f32);
    /// We wrap f64 so that we can impl Eq and PartialEq
    #[derive(Debug, Deserialize, Serialize, Copy, Clone)]
    struct F64TestWrapper(f64);

    impl DiffPatch for F32TestWrapper {
        type Patch = Option<f32>;

        fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
            self.0.create_patch_towards(&end_state.0)
        }

        fn apply_patch(&mut self, patch: Self::Patch) {
            self.0.apply_patch(patch)
        }
    }

    impl DiffPatch for F64TestWrapper {
        type Patch = Option<f64>;

        fn create_patch_towards(&self, end_state: &Self) -> Self::Patch {
            self.0.create_patch_towards(&end_state.0)
        }

        fn apply_patch(&mut self, patch: Self::Patch) {
            self.0.apply_patch(patch)
        }
    }

    impl Eq for F32TestWrapper {}
    impl Eq for F64TestWrapper {}

    impl PartialEq for F32TestWrapper {
        fn eq(&self, other: &Self) -> bool {
            (self.0 - other.0).abs() < std::f32::EPSILON
        }
    }

    impl PartialEq for F64TestWrapper {
        fn eq(&self, other: &Self) -> bool {
            (self.0 - other.0).abs() < std::f64::EPSILON
        }
    }

    #[test]
    fn f32_unchanged() {
        DiffPatchTestCase {
            desc: "Diff patch same f32",
            start: F32TestWrapper(0.),
            end: F32TestWrapper(0.),
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn f32_changed() {
        DiffPatchTestCase {
            desc: "Diff patch different f32",
            start: F32TestWrapper(0.),
            end: F32TestWrapper(5.),
            expected_serialized_patch_size: 5,
        }
        .test();
    }

    #[test]
    fn f64_unchanged() {
        DiffPatchTestCase {
            desc: "Diff patch different f64",
            start: F64TestWrapper(0.),
            end: F64TestWrapper(0.),
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn f64_changed() {
        DiffPatchTestCase {
            desc: "Diff patch different f64",
            start: F64TestWrapper(0.),
            end: F64TestWrapper(5.),
            expected_serialized_patch_size: 9,
        }
        .test();
    }
}
