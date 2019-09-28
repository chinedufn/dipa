use super::*;
use std::fmt::Debug;

use bincode::Options;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Diff/patch from start -> end, then also from end -> state. Useful when both directions
/// should have the same expected_serialized_patch_size.
///
/// For more complex diff/patches where start -> end could be different from end -> start, use
/// the OneDirectionalDiffPatchTestCase
///
/// TODO: Remove 'p so we can test references
pub struct DiffPatchTestCase<'p, T: Debug + DiffPatch<'p> + Eq + PartialEq + Serialize> {
    pub label: Option<&'p str>,
    pub start: T,
    pub end: &'p T,
    pub expected_diff: T::Diff,
    /// The size of the patch in bytes
    pub expected_serialized_patch_size: usize,
}

impl<'p, T: 'p + Debug + DiffPatch<'p> + Eq + PartialEq + Serialize> DiffPatchTestCase<'p, T>
where
    <T as DiffPatch<'p>>::Diff: Serialize + Debug + PartialEq,
    <T as DiffPatch<'p>>::OwnedDiff: DeserializeOwned + Debug + PartialEq,
{
    /// Verify that we can diff/patch from our start to our end as well as
    /// from our end to our start
    pub fn test(self) {
        let expected_serialized_patch_size = self.expected_serialized_patch_size;

        let patch = self.start.create_patch_towards(self.end);

        assert_eq!(patch, self.expected_diff);

        let patch_bytes = bincode::options()
            .with_varint_encoding()
            .serialize(&patch)
            .unwrap();

        let patch = bincode::options()
            .with_varint_encoding()
            .deserialize(&patch_bytes[..])
            .unwrap();
        let mut patched_start = self.start;
        patched_start.apply_patch(patch);

        assert_eq!(&patched_start, self.end, "{:?}", self.label);

        assert_eq!(
            patch_bytes.len(),
            expected_serialized_patch_size,
            "Expected patch size to be: {}. Actually computed: {}",
            expected_serialized_patch_size,
            patch_bytes.len()
        )
    }
}
