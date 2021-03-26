use crate::vec::vec_apply_patch::apply_patch;
use crate::vec::vec_create_patch_towards::patch_towards;
use crate::{CreatePatchTowardsReturn, Diffable, Patchable};

mod longest_common_subsequence;
mod vec_apply_patch;
mod vec_create_patch_towards;

impl<'p, T: 'p + Diffable<'p>> Diffable<'p> for Vec<T>
where
    T: PartialEq,
    &'p T: serde::Serialize,
{
    /// Option so that it's only 1 byte if nothing has changed.
    type Diff = Vec<SequenceModificationDiff<'p, T>>;

    fn create_patch_towards(&self, end_state: &'p Self) -> CreatePatchTowardsReturn<Self::Diff> {
        patch_towards(self, end_state)
    }
}

impl<T> Patchable for Vec<T> {
    /// Option so that it's only 1 byte if nothing has changed.
    type Patch = Vec<OwnedSequenceModificationDiff<T>>;

    fn apply_patch(&mut self, patch: Self::Patch) {
        apply_patch(self, patch)
    }
}

/// Used to diff/patch sequences such as vectors and slices.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum SequenceModificationDiff<'a, T>
where
    &'a T: serde::Serialize,
{
    /// Insert into the Vec<T>, starting at some start index.
    InsertOne { index: usize, value: &'a T },
    /// Prepend an item to the beginning of the vector.
    PrependOne { item: &'a T },
    /// Append item to the end of the new vector
    AppendOne { item: &'a T },
    /// Delete one item from the sequence
    DeleteOne { index: usize },
    /// Replace one item from the sequence
    ReplaceOne { index: usize, new: &'a T },

    // TODO: Heavily optimize small sequences. So we want operations like
    //  DeleteFirst, DeleteSecond ... DeleteTenth ... ReplaceThird ... etc.
    /// Delete the first item in the sequence
    DeleteFirst,
    /// Delete the last item in the sequence
    DeleteLast,
    /// Replace the first item in the sequence
    ReplaceFirst { item: &'a T },
    /// Replace the last item in the sequence
    ReplaceLast { item: &'a T },

    /// Prepend many items to the beginning of the vector.
    PrependMany { items: &'a [T] },
    /// Insert multiple items into the Vec<T>, starting at some start index.
    InsertMany { start_idx: usize, items: &'a [T] },
    /// Delete from the Vec<T> starting from some start index
    DeleteMany {
        start_index: usize,
        items_to_delete: usize,
    },
    /// Append item to the end of the new vector
    AppendMany { items: &'a [T] },
    /// Replace many items in the sequence.
    ReplaceMany {
        start_idx: usize,
        items_to_replace: usize,
        new: &'a [T],
    },
    /// Replace many items in the sequence when we are adding and removing the same number of
    /// values.
    ReplaceManySameAmountAddedAndRemoved { index: usize, new: &'a [T] },

    /// Delete all items
    DeleteAll,
    /// Remove all items *at* AND *before* the specified index.
    DeleteAllBeforeIncluding { end_index: usize },
    /// Remove all items *at* AND *after* the specified index.
    DeleteAllAfterIncluding { start_index: usize },

    /// Replace all values before the provided index, inclusive.
    ReplaceAllBeforeIncluding { before: usize, new: &'a [T] },
    /// Replace all values after the provided index, inclusive.
    ReplaceAllAfterIncluding { after: usize, new: &'a [T] },
}

/// Used to patch sequences such as vectors and slices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum OwnedSequenceModificationDiff<T> {
    /// Insert into the Vec<T>, starting at some start index.
    InsertOne { index: usize, value: T },
    /// Prepend an item to the beginning of the vector.
    PrependOne { item: T },
    /// Append item to the end of the new vector
    AppendOne { item: T },
    /// Delete one item from the sequence
    DeleteOne { index: usize },
    /// Replace one item from the sequence
    ReplaceOne { index: usize, new: T },

    /// Delete the first item in the sequence
    DeleteFirst,
    /// Delete the last item in the sequence
    DeleteLast,
    /// Replace the first item in the sequence
    ReplaceFirst { item: T },
    /// Replace the last item in the sequence
    ReplaceLast { item: T },

    /// Prepend many items to the beginning of the vector.
    PrependMany { items: Vec<T> },
    /// Insert multiple items into the Vec<T>, starting at some start index.
    InsertMany { start_idx: usize, items: Vec<T> },
    /// Delete from the Vec<T> starting from some start index
    DeleteMany {
        start_index: usize,
        items_to_delete: usize,
    },
    /// Append item to the end of the new vector
    AppendMany { items: Vec<T> },
    /// Replace many items in the sequence.
    ReplaceMany {
        start_idx: usize,
        items_to_replace: usize,
        new: Vec<T>,
    },
    /// Replace many items in the sequence when we are adding and removing the same number of
    /// values.
    ReplaceManySameAmountAddedAndRemoved { index: usize, new: Vec<T> },

    /// Delete all items
    DeleteAll,
    /// Remove all items *at* AND *before* the specified index.
    DeleteAllBeforeIncluding { end_index: usize },
    /// Remove all items *at* AND *after* the specified index.
    DeleteAllAfterIncluding { start_index: usize },
    /// Replace all values before the provided index, inclusive.
    ReplaceAllBeforeIncluding { before: usize, new: Vec<T> },
    /// Replace all values after the provided index, inclusive.
    ReplaceAllAfterIncluding { after: usize, new: Vec<T> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dipa_impl_tester::DiffPatchTestCase;
    use crate::test_utils::{
        macro_optimization_hint_did_change, macro_optimization_hint_unchanged,
    };
    use bincode::Options;

    /// 1 byte for the u8 length of the Vec that holds all of the patch operations
    const BASE_PATCH_BYTES: usize = 1;

    /// Verify that there is no diff if the start and end vector are the same.
    #[test]
    fn vec_unchanged() {
        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3],
            end: &vec![1u8, 2, 3],
            expected_diff: vec![],
            // No change, none variant is one byte
            expected_serialized_patch_size: 1,
            expected_macro_hints: macro_optimization_hint_unchanged(),
        }
        .test();
    }

    /// Verify that we delete one extra item at the end.
    #[test]
    fn deletion_one_at_end() {
        // 1 for the variant
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![0u8, 1, 2, 3],
            end: &vec![0u8, 1, 2],
            expected_diff: vec![SequenceModificationDiff::DeleteLast],
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we delete many extra items at the end.
    #[test]
    fn deletion_many_at_end() {
        // 1 for the variant then
        // 1 bytes for the start index
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![0u8, 1, 2, 3, 4],
            end: &vec![0u8, 1, 2],
            expected_diff: vec![SequenceModificationDiff::DeleteAllAfterIncluding {
                start_index: 3,
            }],
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we properly delete the first item.
    #[test]
    fn delete_one_at_beginning() {
        // 1 for the variant then
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![0u8, 1, 2],
            end: &vec![1u8, 2],
            expected_diff: vec![SequenceModificationDiff::DeleteFirst],
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we properly delete the first `n` items.
    #[test]
    fn delete_many_at_beginning() {
        // 1 for the variant then
        // 1 the index
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![0u8, 1, 2, 3],
            end: &vec![2u8, 3],
            expected_diff: vec![SequenceModificationDiff::DeleteAllBeforeIncluding {
                end_index: 1,
            }],
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we delete many extra items at the end.
    #[test]
    fn delete_many_beginning_and_many_end() {
        // 2 for the two variants then
        // 1 bytes for the end index
        // 1 bytes for the start index
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 2 + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![0u8, 1, 2, 3, 4, 5],
            end: &vec![2],
            expected_diff: vec![
                SequenceModificationDiff::DeleteAllAfterIncluding { start_index: 3 },
                SequenceModificationDiff::DeleteAllBeforeIncluding { end_index: 1 },
            ],
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we delete one item in the middle.
    #[test]
    fn delete_one_in_middle() {
        let expected_patch = vec![SequenceModificationDiff::DeleteOne { index: 1 }];

        // 1 for the one variant in the vec, 1 index
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3],
            end: &vec![1u8, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we delete many items in the middle.
    #[test]
    fn delete_many_in_middle() {
        let expected_patch = vec![SequenceModificationDiff::DeleteMany {
            start_index: 1,
            items_to_delete: 2,
        }];

        // 1 for the one variant in the vec
        // 1 for start idx
        // 1 for item to delete count
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3, 4],
            end: &vec![1u8, 4],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we properly insert one item at the beginning of the start sequence.
    #[test]
    fn insert_one_at_beginning() {
        let expected_patch = vec![SequenceModificationDiff::PrependOne { item: &1 }];

        // 1 for the one variant in the vec, 1 for the appended u8
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![2u8, 3],
            end: &vec![1u8, 2, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we properly insert many items at the beginning of the start sequence.
    #[test]
    fn insert_many_at_beginning() {
        let expected_patch = vec![SequenceModificationDiff::PrependMany { items: &[1, 2] }];

        // 1 for the one variant in the vec
        // 1 for the length of the items vector
        // 2 for the prepended u8's
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![3u8, 4],
            end: &vec![1u8, 2u8, 3, 4],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we properly diff/patch inserting one item in the middle
    #[test]
    fn insert_one_in_middle() {
        let expected_patch = vec![SequenceModificationDiff::InsertOne {
            index: 1,
            value: &2,
        }];

        // 1 for the one variant in the vec, 1 for the index, 1 for the appended u8
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1;

        // If the start idx has advanced by 1 but the end index has advanced by 2 then
        //  insert one
        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 3],
            end: &vec![1u8, 2, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Insert multiple items into the middle of the array.
    #[test]
    fn insert_many_in_middle() {
        let expected_patch = vec![SequenceModificationDiff::InsertMany {
            start_idx: 1,
            items: &[2, 3],
        }];

        // 1 for the one variant in the vec
        // 1 for the index
        // 1 for length of items slice
        // 2 for the appended u8's
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 4],
            end: &vec![1u8, 2, 3, 4],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we append one item to the end.
    #[test]
    fn append_one_at_end() {
        let expected_patch = vec![SequenceModificationDiff::AppendOne { item: &3 }];

        // 1 for the one variant in the vec, 1 for the appended u8
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2],
            end: &vec![1u8, 2, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we create a patch to append many items to the end.
    #[test]
    fn append_many_at_end() {
        let expected_patch = vec![SequenceModificationDiff::AppendMany { items: &[3, 4] }];

        // 1 for the one variant in the modifications
        // 1 for the length of the items vec
        // 2 for the appended u8's
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2],
            end: &vec![1u8, 2, 3, 4],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can replace one item at the beginning of the array.
    #[test]
    fn replace_one_at_beginning() {
        let expected_patch = vec![SequenceModificationDiff::ReplaceFirst { item: &2 }];

        // 1 for the one variant in the modifications
        // 1 for the item
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 3],
            end: &vec![2u8, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can replace many items in the middle of the array.
    #[test]
    fn replace_many_at_beginning() {
        let expected_patch = vec![SequenceModificationDiff::ReplaceAllBeforeIncluding {
            before: 1,
            new: &[5, 6],
        }];

        // 1 for the one variant in the modifications
        // 1 for the index
        // 1 for the items length
        // 2 for the items
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3, 4],
            end: &vec![5u8, 6, 3, 4],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can replace one item at the end of the array.
    #[test]
    fn replace_one_at_end() {
        let expected_patch = vec![SequenceModificationDiff::ReplaceLast { item: &3 }];

        // 1 for the one variant in the modifications
        // 1 for the item
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2],
            end: &vec![1u8, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can replace many items at the end of the array.
    #[test]
    fn replace_many_at_end() {
        let expected_patch = vec![SequenceModificationDiff::ReplaceAllAfterIncluding {
            after: 2,
            new: &[5, 6],
        }];

        // 1 for the one variant in the modifications
        // 1 for index
        // 1 for the items length
        // 2 for the items
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3, 4],
            end: &vec![1u8, 2, 5, 6],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can replace one item in the middle of the array.
    #[test]
    fn replace_one_in_middle() {
        let expected_patch = vec![SequenceModificationDiff::ReplaceOne { index: 1, new: &4 }];

        // 1 for the one variant in the modifications
        // 1 for index
        // 1 for the item
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3],
            end: &vec![1u8, 4, 3],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can rename `n` items with `m` new items.
    #[test]
    fn replace_many_in_middle() {
        let expected_patch = vec![SequenceModificationDiff::ReplaceMany {
            start_idx: 1,
            items_to_replace: 3,
            new: &[6, 7],
        }];

        // 1 for the one variant in the modifications
        // 1 for index
        // 1 for items to replace count
        // 1 for length of items
        // 2 for the items
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3, 4, 5],
            end: &vec![1u8, 6, 7, 5],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we can replace many items in the middle of the array when we are removing just
    /// as many items as we are adding.
    #[test]
    fn replace_many_in_middle_same_amount_added_and_removed() {
        let expected_patch = vec![
            SequenceModificationDiff::ReplaceManySameAmountAddedAndRemoved {
                index: 1,
                new: &[5, 6],
            },
        ];

        // 1 for the one variant in the modifications
        // 1 for index
        // 1 for items length
        // 2 for the items
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1 + 1 + 1 + 2;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3, 4],
            end: &vec![1u8, 5, 6, 4],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that we create a patch to remove all items.
    #[test]
    fn delete_entire_vector() {
        let expected_patch = vec![SequenceModificationDiff::DeleteAll];

        // 1 for the one variant in the modifications
        let expected_serialized_patch_size = BASE_PATCH_BYTES + 1;

        DiffPatchTestCase {
            label: None,
            start: vec![1u8, 2, 3, 4],
            end: &vec![],
            expected_diff: expected_patch,
            expected_serialized_patch_size,
            expected_macro_hints: macro_optimization_hint_did_change(),
        }
        .test();
    }

    /// Verify that only one byte is used to serialize a sequence modification variant.
    ///
    /// This test guards against us accidentally adding more than 250 variants, at which point
    /// bincode would begin to use 2 bytes instead of one.
    #[test]
    fn sequence_variant_one_byte() {
        let diff: SequenceModificationDiff<()> = SequenceModificationDiff::DeleteFirst;

        assert_eq!(
            bincode::options()
                .with_varint_encoding()
                .serialize(&diff)
                .unwrap()
                .len(),
            1
        );
    }
}
