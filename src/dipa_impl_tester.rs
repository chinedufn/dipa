use super::Diffable;
use std::fmt::Debug;

use crate::{MacroOptimizationHints, Patchable};
use bincode::Options;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde::__private::PhantomData;

/// Diff/patch from start -> end, asserting that the patching was successful and that the patch
/// has the expected bincode serialized size (with varint encoding enabled).
///
/// Useful for verifying that custom implementations of the [DiffPatch] trait work as expected.
pub struct DiffPatchTestCase<
    'p,
    T: Debug + Diffable<'p, T> + Patchable<<T as Diffable<'p, T>>::Patch> + Eq + PartialEq + Serialize,
> {
    pub label: Option<&'p str>,
    pub start: T,
    pub end: &'p T,
    pub expected_delta: T::Diff,
    /// The size of the patch in bytes
    pub expected_serialized_patch_size: usize,
    pub expected_macro_hints: MacroOptimizationHints,
}

impl<
        'p,
        T: 'p
            + Debug
            + Diffable<'p, T>
            + Patchable<<T as Diffable<'p, T>>::Patch>
            + Eq
            + PartialEq
            + Serialize,
    > DiffPatchTestCase<'p, T>
where
    <T as Diffable<'p, T>>::Diff: Serialize + Debug + PartialEq,
    <T as Diffable<'p, T>>::Patch: DeserializeOwned + Debug + PartialEq,
{
    /// Verify that we can diff/patch from our start to our end as well as
    /// from our end to our start
    pub fn test(self) {
        let expected_serialized_patch_size = self.expected_serialized_patch_size;

        let (patch, macro_hints) = self.start.create_patch_towards(self.end);

        assert_eq!(
            patch, self.expected_delta,
            r#"
Test Label {:?}
"#,
            self.label
        );

        let patch_bytes = bincode::options()
            .with_varint_encoding()
            .serialize(&patch)
            .unwrap();

        let patch: <T as Diffable<'p, T>>::Patch = bincode::options()
            .with_varint_encoding()
            .deserialize(&patch_bytes[..])
            .unwrap();
        let mut patched_start = self.start;
        patched_start.apply_patch(patch);

        assert_eq!(&patched_start, self.end, "{:?}", self.label);

        assert_eq!(
            patch_bytes.len(),
            expected_serialized_patch_size,
            r#"
Expected patch size to be: {}
Actually computed: {}
Test Label: {:?}
"#,
            expected_serialized_patch_size,
            patch_bytes.len(),
            self.label
        );

        assert_eq!(macro_hints, self.expected_macro_hints);
    }
}

/// Create PhantomData<P>
pub fn patch_ty<P>() -> PhantomData<P> {
    PhantomData::<P>::default()
}
