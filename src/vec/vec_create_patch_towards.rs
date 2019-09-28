use crate::vec::longest_common_subsequence::get_longest_common_subsequence;
use crate::vec::SequenceModificationDiff;

// Tests are in the parent module.
pub(super) fn patch_towards<'p, T: PartialEq>(
    before: &Vec<T>,
    target_state: &'p Vec<T>,
) -> Vec<SequenceModificationDiff<'p, T>>
where
    &'p T: serde::Serialize,
{
    if target_state.len() == 0 && before.len() > 0 {
        return vec![SequenceModificationDiff::DeleteAll];
    }

    let mut modifications = vec![];

    let lcs = get_longest_common_subsequence(before, target_state);

    let mut previous_start_idx = None;
    let mut previous_target_idx = None;

    for (start_common_idx, target_common_idx) in lcs.into_iter() {
        let advanced_by = match (previous_start_idx, previous_target_idx) {
            (Some(previous_start), Some(previous_target)) => Some(AdvancedBy {
                start_sequence: start_common_idx - previous_start,
                target_sequence: target_common_idx - previous_target,
            }),
            _ => None,
        };

        if let Some(advanced_by) = advanced_by {
            let modification = match (advanced_by.start_sequence, advanced_by.target_sequence) {
                (start_advance, target_advance) if start_advance == 1 && target_advance == 2 => {
                    Some(SequenceModificationDiff::InsertOne {
                        index: previous_start_idx.unwrap() + 1,
                        value: &target_state[previous_target_idx.unwrap() + 1],
                    })
                }
                (start_advance, target_advance) if start_advance == 1 && target_advance > 2 => {
                    Some(SequenceModificationDiff::InsertMany {
                        start_idx: previous_start_idx.unwrap() + 1,
                        items: &target_state[previous_target_idx.unwrap() + 1..target_common_idx],
                    })
                }
                (start_advance, target_advance) if start_advance == 2 && target_advance == 2 => {
                    Some(SequenceModificationDiff::ReplaceOne {
                        index: previous_start_idx.unwrap() + 1,
                        new: &target_state[previous_target_idx.unwrap() + 1],
                    })
                }
                (start_advance, target_advance)
                    if start_advance > 2 && target_advance == start_advance =>
                {
                    Some(
                        SequenceModificationDiff::ReplaceManySameAmountAddedAndRemoved {
                            index: previous_start_idx.unwrap() + 1,
                            new: &target_state[previous_target_idx.unwrap() + 1..target_advance],
                        },
                    )
                }
                (start_advance, target_advance) if start_advance > 2 && target_advance > 1 => {
                    Some(SequenceModificationDiff::ReplaceMany {
                        start_idx: previous_start_idx.unwrap() + 1,
                        items_to_replace: start_advance - 1,
                        new: &target_state[previous_target_idx.unwrap() + 1..target_advance],
                    })
                }
                (start_advance, target_advance) if start_advance == 2 && target_advance == 1 => {
                    Some(SequenceModificationDiff::DeleteOne {
                        index: previous_start_idx.unwrap() + 1,
                    })
                }
                (start_advance, target_advance) if start_advance > 2 && target_advance == 1 => {
                    Some(SequenceModificationDiff::DeleteMany {
                        start_index: previous_start_idx.unwrap() + 1,
                        items_to_delete: start_advance - 1,
                    })
                }
                _ => {
                    // TODO: Once we implement all possible combinations we
                    // can make this branch unreachable_unchecked!()
                    None
                }
            };

            if let Some(modification) = modification {
                modifications.push(modification);
            }
        } else {
            if start_common_idx == 1 && target_common_idx == 1 {
                let modification = SequenceModificationDiff::ReplaceFirst {
                    item: &target_state[0],
                };
                modifications.push(modification);
            } else if start_common_idx > 1 && start_common_idx == target_common_idx {
                let modification = SequenceModificationDiff::ReplaceAllBeforeIncluding {
                    before: start_common_idx - 1,
                    new: &target_state[0..target_common_idx],
                };
                modifications.push(modification);
            } else if start_common_idx == 1 && target_common_idx == 0 {
                let modification = SequenceModificationDiff::DeleteFirst;
                modifications.push(modification);
            } else if start_common_idx > 1 && target_common_idx == 0 {
                let modification = SequenceModificationDiff::DeleteAllBeforeIncluding {
                    end_index: start_common_idx - 1,
                };
                modifications.push(modification);
            } else if start_common_idx == 0 && target_common_idx == 1 {
                let modification = SequenceModificationDiff::PrependOne {
                    item: &target_state[0],
                };
                modifications.push(modification);
            } else if start_common_idx == 0 && target_common_idx > 1 {
                let modification = SequenceModificationDiff::PrependMany {
                    items: &target_state[0..target_common_idx],
                };
                modifications.push(modification);
            }
        }

        previous_start_idx = Some(start_common_idx);
        previous_target_idx = Some(target_common_idx);
    }

    if let (Some(previous_start_idx), Some(previous_target_idx)) =
        (previous_start_idx, previous_target_idx)
    {
        let start_remaining = before.len() - 1 - previous_start_idx;
        let target_remaining = target_state.len() - 1 - previous_target_idx;

        let modification = if start_remaining == 1 && target_remaining == 1 {
            Some(SequenceModificationDiff::ReplaceLast {
                item: target_state.last().unwrap(),
            })
        } else if start_remaining > 1 && start_remaining == target_remaining {
            Some(SequenceModificationDiff::ReplaceAllAfterIncluding {
                after: previous_start_idx + 1,
                new: &target_state[previous_target_idx + 1..],
            })
        } else if start_remaining == 1 && target_remaining == 0 {
            Some(SequenceModificationDiff::DeleteLast)
        } else if start_remaining > 1 && target_remaining == 0 {
            Some(SequenceModificationDiff::DeleteAllAfterIncluding {
                start_index: previous_start_idx + 1,
            })
        } else if start_remaining == 0 && target_remaining == 1 {
            Some(SequenceModificationDiff::AppendOne {
                item: target_state.last().unwrap(),
            })
        } else if start_remaining == 0 && target_remaining > 1 {
            Some(SequenceModificationDiff::AppendMany {
                items: &target_state[target_state.len() - target_remaining..],
            })
        } else {
            None
        };
        if let Some(modification) = modification {
            modifications.push(modification);
        }
    }

    modifications.reverse();
    modifications
}

#[derive(Copy, Clone)]
struct AdvancedBy {
    start_sequence: usize,
    target_sequence: usize,
}
