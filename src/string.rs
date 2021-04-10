use crate::sequence::{SequenceModificationDelta, SequenceModificationDeltaOwned};
use crate::{Diffable, MacroOptimizationHints, Patchable};

impl<'s, 'e> Diffable<'s, 'e, String> for String {
    type Delta = Vec<SequenceModificationDelta<'e, u8>>;
    type DeltaOwned = Vec<SequenceModificationDeltaOwned<u8>>;

    fn create_delta_towards(&self, end_state: &'e String) -> (Self::Delta, MacroOptimizationHints) {
        self.as_bytes().create_delta_towards(&end_state.as_bytes())
    }
}

impl Patchable<Vec<SequenceModificationDeltaOwned<u8>>> for String {
    fn apply_patch(&mut self, patch: Vec<SequenceModificationDeltaOwned<u8>>) {
        // TODO: More efficient implementation without copying.. Just quickly getting things working.
        let mut bytes = self.as_bytes().to_vec();

        bytes.apply_patch(patch);

        *self = String::from_utf8(bytes).unwrap()
    }
}

impl<'s, 'e> Diffable<'s, 'e, str> for str {
    type Delta = Vec<SequenceModificationDelta<'e, u8>>;
    type DeltaOwned = Vec<SequenceModificationDeltaOwned<u8>>;

    fn create_delta_towards(&'s self, end_state: &'e str) -> (Self::Delta, MacroOptimizationHints) {
        self.as_bytes().create_delta_towards(&end_state.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::SequenceModificationDelta;
    use crate::DipaImplTester;

    /// Verify that we can diff and patch strings.
    #[test]
    fn string_dipa() {
        DipaImplTester {
            label: Some("String unchanged"),
            start: &mut "XYZ".to_string(),
            end: &"XYZ".to_string(),
            expected_delta: vec![],
            // 1 for vec length
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DipaImplTester {
            label: Some("String changed"),
            start: &mut "ABCDE".to_string(),
            end: &"ABDE".to_string(),
            expected_delta: vec![SequenceModificationDelta::DeleteOne { index: 2 }],
            // 1 for vec length, 1 for variant, 1 for index
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}
