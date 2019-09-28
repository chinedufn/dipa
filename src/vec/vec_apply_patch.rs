use crate::vec::OwnedSequenceModificationDiff;

// Tested in parent module.
pub(super) fn apply_patch<'p, T>(
    receiver: &mut Vec<T>,
    patch: Vec<OwnedSequenceModificationDiff<T>>,
) {
    for modification in patch {
        match modification {
            OwnedSequenceModificationDiff::InsertOne { index, value } => {
                receiver.insert(index, value);
            }
            OwnedSequenceModificationDiff::DeleteOne { index } => {
                receiver.remove(index);
            }
            OwnedSequenceModificationDiff::DeleteMany {
                start_index,
                items_to_delete,
            } => {
                for _ in 0..items_to_delete {
                    receiver.remove(start_index);
                }
            }
            OwnedSequenceModificationDiff::DeleteAllAfterIncluding { start_index } => {
                receiver.truncate(start_index);
            }
            OwnedSequenceModificationDiff::AppendOne { item } => {
                receiver.push(item);
            }
            OwnedSequenceModificationDiff::PrependOne { item } => {
                receiver.insert(0, item);
            }
            OwnedSequenceModificationDiff::InsertMany { start_idx, items } => {
                let mut current = start_idx;

                for item in items {
                    receiver.insert(current, item);
                    current += 1;
                }
            }
            OwnedSequenceModificationDiff::DeleteAllBeforeIncluding { end_index } => {
                for _ in 0..=end_index {
                    receiver.remove(0);
                }
            }
            OwnedSequenceModificationDiff::AppendMany { items } => {
                for item in items {
                    receiver.push(item);
                }
            }
            OwnedSequenceModificationDiff::DeleteFirst => {
                receiver.remove(0);
            }
            OwnedSequenceModificationDiff::DeleteLast => {
                receiver.remove(receiver.len() - 1);
            }
            OwnedSequenceModificationDiff::PrependMany { items } => {
                let mut idx = 0;

                for item in items {
                    receiver.insert(idx, item);
                    idx += 1;
                }
            }
            OwnedSequenceModificationDiff::ReplaceOne { index, new } => {
                receiver[index] = new;
            }
            OwnedSequenceModificationDiff::ReplaceFirst { item } => {
                receiver[0] = item;
            }
            OwnedSequenceModificationDiff::ReplaceLast { item } => {
                *receiver.last_mut().unwrap() = item;
            }
            OwnedSequenceModificationDiff::ReplaceMany {
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
            OwnedSequenceModificationDiff::ReplaceManySameAmountAddedAndRemoved { index, new } => {
                for (offset, item) in new.into_iter().enumerate() {
                    receiver[index + offset] = item;
                }
            }
            OwnedSequenceModificationDiff::ReplaceAllBeforeIncluding { before, new } => {
                for _ in 0..=before {
                    receiver.remove(0);
                }

                for (idx, item) in new.into_iter().enumerate() {
                    receiver.insert(idx, item);
                }
            }
            OwnedSequenceModificationDiff::ReplaceAllAfterIncluding { after, new } => {
                receiver.truncate(after);

                for item in new {
                    receiver.push(item);
                }
            }
            OwnedSequenceModificationDiff::DeleteAll => {
                receiver.clear();
            }
        };
    }
}
