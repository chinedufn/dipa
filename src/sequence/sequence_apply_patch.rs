use crate::sequence::SequenceModificationDeltaOwned;

// Tested in parent module.
pub(super) fn apply_patch<'p, T>(
    receiver: &mut Vec<T>,
    patch: Vec<SequenceModificationDeltaOwned<T>>,
) {
    for modification in patch {
        match modification {
            SequenceModificationDeltaOwned::InsertOne { index, value } => {
                receiver.insert(index, value);
            }
            SequenceModificationDeltaOwned::DeleteOne { index } => {
                receiver.remove(index);
            }
            SequenceModificationDeltaOwned::DeleteMany {
                start_index,
                items_to_delete,
            } => {
                for _ in 0..items_to_delete {
                    receiver.remove(start_index);
                }
            }
            SequenceModificationDeltaOwned::DeleteAllAfterIncluding { start_index } => {
                receiver.truncate(start_index);
            }
            SequenceModificationDeltaOwned::AppendOne { item } => {
                receiver.push(item);
            }
            SequenceModificationDeltaOwned::PrependOne { item } => {
                receiver.insert(0, item);
            }
            SequenceModificationDeltaOwned::InsertMany { start_idx, items } => {
                let mut current = start_idx;

                for item in items {
                    receiver.insert(current, item);
                    current += 1;
                }
            }
            SequenceModificationDeltaOwned::DeleteAllBeforeIncluding { end_index } => {
                for _ in 0..=end_index {
                    receiver.remove(0);
                }
            }
            SequenceModificationDeltaOwned::AppendMany { items } => {
                for item in items {
                    receiver.push(item);
                }
            }
            SequenceModificationDeltaOwned::DeleteFirst => {
                receiver.remove(0);
            }
            SequenceModificationDeltaOwned::DeleteLast => {
                receiver.remove(receiver.len() - 1);
            }
            SequenceModificationDeltaOwned::PrependMany { items } => {
                let mut idx = 0;

                for item in items {
                    receiver.insert(idx, item);
                    idx += 1;
                }
            }
            SequenceModificationDeltaOwned::ReplaceOne { index, new } => {
                receiver[index] = new;
            }
            SequenceModificationDeltaOwned::ReplaceFirst { item } => {
                receiver[0] = item;
            }
            SequenceModificationDeltaOwned::ReplaceLast { item } => {
                *receiver.last_mut().unwrap() = item;
            }
            SequenceModificationDeltaOwned::ReplaceMany {
                start_idx,
                items_to_replace,
                new,
            } => {
                for _ in 0..items_to_replace {
                    receiver.remove(start_idx);
                }

                let mut offset = 0;
                for item in new {
                    receiver.insert(start_idx + offset, item);
                    offset += 1;
                }
            }
            SequenceModificationDeltaOwned::ReplaceManySameAmountAddedAndRemoved { index, new } => {
                for (offset, item) in new.into_iter().enumerate() {
                    receiver[index + offset] = item;
                }
            }
            SequenceModificationDeltaOwned::ReplaceAllBeforeIncluding { before, new } => {
                for _ in 0..=before {
                    receiver.remove(0);
                }

                for (idx, item) in new.into_iter().enumerate() {
                    receiver.insert(idx, item);
                }
            }
            SequenceModificationDeltaOwned::ReplaceAllAfterIncluding { after, new } => {
                receiver.truncate(after);

                for item in new {
                    receiver.push(item);
                }
            }
            SequenceModificationDeltaOwned::DeleteAll => {
                receiver.clear();
            }
            SequenceModificationDeltaOwned::ReplaceAll { new } => {
                *receiver = new;
            }
        };
    }
}
