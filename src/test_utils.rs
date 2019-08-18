use super::*;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub struct DiffPatchTestCase<
    T: Debug + Clone + DiffPatch + Eq + PartialEq + Serialize + DeserializeOwned,
> {
    pub desc: &'static str,
    pub start: T,
    pub end: T,
    /// The size of the patch in bytes
    pub expected_serialized_patch_size: usize,
}

impl<T: Debug + Clone + DiffPatch + Eq + PartialEq + Serialize + DeserializeOwned>
    DiffPatchTestCase<T>
where
    <T as DiffPatch>::Patch: Serialize + Debug,
{
    /// Verify that we can diff/patch from our start to our end as well as
    /// from our end to our start
    pub fn test(self) {
        let mut start = self.start.clone();
        let start_clone = start.clone();

        let end = self.end.clone();

        // Start to end
        self.patch(&mut start, &self.end);

        // End to start
        self.patch(&mut end.clone(), &start_clone)
    }

    fn patch(&self, start: &mut T, end: &T) {
        let expected_serialized_patch_size = self.expected_serialized_patch_size;

        let patch = start.create_patch_towards(&end);

        let patch_bytes = bincode::serialize(&patch).unwrap();

        start.apply_patch(patch);

        assert_eq!(start, end, "{}", self.desc);

        assert_eq!(
            patch_bytes.len(),
            expected_serialized_patch_size,
            "Expected patch size to be: {}. Actually computed: {}",
            expected_serialized_patch_size,
            patch_bytes.len()
        )
    }
}
