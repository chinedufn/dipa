use crate::CreatedDelta;
use crate::{number_diff_impl_option_wrapped, number_patch_impl_option_wrapped};

number_diff_impl_option_wrapped!(f32, f32);
number_patch_impl_option_wrapped!(f32, Option<f32>);

number_diff_impl_option_wrapped!(f64, f64);
number_patch_impl_option_wrapped!(f64, Option<f64>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dipa_impl_tester::DipaImplTester;
    use crate::{Diffable, Patchable};

    /// We wrap f32 so that we can impl Eq and PartialEq
    #[derive(Debug, Deserialize, Serialize, Copy, Clone)]
    struct F32TestWrapper(f32);
    /// We wrap f64 so that we can impl Eq and PartialEq
    #[derive(Debug, Deserialize, Serialize, Copy, Clone)]
    struct F64TestWrapper(f64);

    impl<'s, 'e> Diffable<'s, 'e, F32TestWrapper> for F32TestWrapper {
        type Delta = Option<f32>;
        type DeltaOwned = Option<f32>;

        fn create_delta_towards(&self, end_state: &Self) -> CreatedDelta<Self::Delta> {
            self.0.create_delta_towards(&end_state.0)
        }
    }

    impl Patchable<Option<f32>> for F32TestWrapper {
        fn apply_patch(&mut self, patch: Option<f32>) {
            self.0.apply_patch(patch)
        }
    }

    impl<'s, 'e> Diffable<'s, 'e, F64TestWrapper> for F64TestWrapper {
        type Delta = Option<f64>;
        type DeltaOwned = Option<f64>;

        fn create_delta_towards(&self, end_state: &Self) -> CreatedDelta<Self::Delta> {
            self.0.create_delta_towards(&end_state.0)
        }
    }

    impl Patchable<Option<f64>> for F64TestWrapper {
        fn apply_patch(&mut self, patch: Option<f64>) {
            self.0.apply_patch(patch)
        }
    }

    impl Eq for F32TestWrapper {}
    impl Eq for F64TestWrapper {}

    impl PartialEq for F32TestWrapper {
        fn eq(&self, other: &Self) -> bool {
            (self.0 - other.0).abs() < f32::EPSILON
        }
    }

    impl PartialEq for F64TestWrapper {
        fn eq(&self, other: &Self) -> bool {
            (self.0 - other.0).abs() < f64::EPSILON
        }
    }

    #[test]
    fn f32_unchanged() {
        DipaImplTester {
            label: Some("Diff patch same f32"),
            start: &mut F32TestWrapper(0.),
            end: &F32TestWrapper(0.),
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn f32_changed() {
        DipaImplTester {
            label: Some("Diff patch different f32"),
            start: &mut F32TestWrapper(0.),
            end: &F32TestWrapper(5.),
            expected_delta: Some(5.),
            expected_serialized_patch_size: 5,
            expected_did_change: true,
        }
        .test();
    }

    #[test]
    fn f64_unchanged() {
        DipaImplTester {
            label: Some("Diff patch different f64"),
            start: &mut F64TestWrapper(0.),
            end: &F64TestWrapper(0.),
            expected_delta: None,
            expected_serialized_patch_size: 1,
            expected_did_change: false,
        }
        .test();
    }

    #[test]
    fn f64_changed() {
        DipaImplTester {
            label: Some("Diff patch different f64"),
            start: &mut F64TestWrapper(0.),
            end: &F64TestWrapper(5.),
            expected_delta: Some(5.),
            expected_serialized_patch_size: 9,
            expected_did_change: true,
        }
        .test();
    }
}
