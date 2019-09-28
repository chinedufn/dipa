use crate::DiffPatch;

impl<'p> DiffPatch<'p> for f32 {
    // TODO: Option<&f32>
    type Diff = Option<f32>;
    type OwnedDiff = Self::Diff;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
        if let Some(patch) = patch {
            *self = patch;
        }
    }
}

impl<'p> DiffPatch<'p> for f64 {
    // TODO: Option<&f64>
    type Diff = Option<f64>;
    type OwnedDiff = Option<f64>;

    fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
        match *self == *end_state {
            true => None,
            false => Some(*end_state),
        }
    }

    fn apply_patch(&mut self, patch: Self::Diff) {
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

    impl<'p> DiffPatch<'p> for F32TestWrapper {
        type Diff = Option<f32>;
        type OwnedDiff = Option<f32>;

        fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
            self.0.create_patch_towards(&end_state.0)
        }

        fn apply_patch(&mut self, patch: Self::Diff) {
            self.0.apply_patch(patch)
        }
    }

    impl<'p> DiffPatch<'p> for F64TestWrapper {
        type Diff = Option<f64>;
        type OwnedDiff = Option<f64>;

        fn create_patch_towards(&self, end_state: &Self) -> Self::Diff {
            self.0.create_patch_towards(&end_state.0)
        }

        fn apply_patch(&mut self, patch: Self::Diff) {
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
            label: Some("Diff patch same f32"),
            start: F32TestWrapper(0.),
            end: &F32TestWrapper(0.),
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn f32_changed() {
        DiffPatchTestCase {
            label: Some("Diff patch different f32"),
            start: F32TestWrapper(0.),
            end: &F32TestWrapper(5.),
            expected_diff: Some(5.),
            expected_serialized_patch_size: 5,
        }
        .test();
    }

    #[test]
    fn f64_unchanged() {
        DiffPatchTestCase {
            label: Some("Diff patch different f64"),
            start: F64TestWrapper(0.),
            end: &F64TestWrapper(0.),
            expected_diff: None,
            expected_serialized_patch_size: 1,
        }
        .test();
    }

    #[test]
    fn f64_changed() {
        DiffPatchTestCase {
            label: Some("Diff patch different f64"),
            start: F64TestWrapper(0.),
            end: &F64TestWrapper(5.),
            expected_diff: Some(5.),
            expected_serialized_patch_size: 9,
        }
        .test();
    }
}
