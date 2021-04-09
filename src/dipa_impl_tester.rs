use super::Diffable;
use std::fmt::Debug;

use crate::{MacroOptimizationHints, Patchable};
use bincode::Options;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Diff/patch from start -> end, asserting that the patching was successful and that the patch
/// has the expected bincode serialized size (with varint encoding enabled).
///
/// Useful for verifying that custom implementations of the [DiffPatch] trait work as expected.
pub struct DipaImplTester<
    's,
    'e,
    T: Debug + Diffable<'s, 'e, T> + Patchable<<T as Diffable<'s, 'e, T>>::DeltaOwned> + PartialEq,
> {
    pub label: Option<&'s str>,
    pub start: &'s mut T,
    pub end: &'e T,
    pub expected_delta: T::Delta,
    /// The size of the patch in bytes when bincoded with variable integer encoding.
    pub expected_serialized_patch_size: usize,
    pub expected_macro_hints: MacroOptimizationHints,
}

impl<'s, 'e, T> DipaImplTester<'s, 'e, T>
where
    <T as Diffable<'s, 'e, T>>::Delta: Serialize + Debug + PartialEq,
    <T as Diffable<'s, 'e, T>>::DeltaOwned: DeserializeOwned,
    T: Debug,
    T: Diffable<'s, 'e, T>,
    T: Patchable<<T as Diffable<'s, 'e, T>>::DeltaOwned>,
    T: PartialEq,
{
    /// Verify that we can diff/patch from our start to our end as well as
    /// from our end to our start
    pub fn test(self) {
        // SAFETY: Using this to get around lifetime requirements. There may be a better approach
        // that does not require unsafe code. We aren't returning any borrowed data so this should
        // be safe.
        let start = unsafe { &mut *(self.start as *mut T) };

        let (delta, macro_hints) = start.create_delta_towards(self.end);

        assert_eq!(
            delta, self.expected_delta,
            r#"
Test Label {:?}
"#,
            self.label
        );

        assert_eq!(macro_hints, self.expected_macro_hints);

        let delta_bytes = bincode::options()
            .with_varint_encoding()
            .serialize(&delta)
            .unwrap();

        let patch: <T as Diffable<'s, 'e, T>>::DeltaOwned = bincode::options()
            .with_varint_encoding()
            .deserialize(&delta_bytes[..])
            .unwrap();
        self.start.apply_patch(patch);

        assert_eq!(self.start, self.end, "{:?}", self.label);

        assert_eq!(
            delta_bytes.len(),
            self.expected_serialized_patch_size,
            r#"
Expected patch size to be: {}
Actually computed: {}
Test Label: {:?}
"#,
            self.expected_serialized_patch_size,
            delta_bytes.len(),
            self.label
        );
    }
}
