use crate::sequence::longest_common_subsequence::get_longest_common_subsequence;
use crate::sequence::SequenceModificationDelta;
use crate::MacroOptimizationHints;

// Tests are in the parent module.
pub(super) fn delta_towards<'p, T: PartialEq>(
    before: &[T],
    target_state: &'p [T],
) -> (
    Vec<SequenceModificationDelta<'p, T>>,
    MacroOptimizationHints,
)
where
    &'p T: serde::Serialize,
{
    if target_state.len() == 0 && before.len() > 0 {
        return (
            vec![SequenceModificationDelta::DeleteAll],
            MacroOptimizationHints { did_change: true },
        );
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
                    Some(SequenceModificationDelta::InsertOne {
                        index: previous_start_idx.unwrap() + 1,
                        value: &target_state[previous_target_idx.unwrap() + 1],
                    })
                }
                (start_advance, target_advance) if start_advance == 1 && target_advance > 2 => {
                    Some(SequenceModificationDelta::InsertMany {
                        start_idx: previous_start_idx.unwrap() + 1,
                        items: &target_state[previous_target_idx.unwrap() + 1..target_common_idx],
                    })
                }
                (start_advance, target_advance) if start_advance == 2 && target_advance == 2 => {
                    Some(SequenceModificationDelta::ReplaceOne {
                        index: previous_start_idx.unwrap() + 1,
                        new: &target_state[previous_target_idx.unwrap() + 1],
                    })
                }
                (start_advance, target_advance)
                    if start_advance > 2 && target_advance == start_advance =>
                {
                    Some(
                        SequenceModificationDelta::ReplaceManySameAmountAddedAndRemoved {
                            index: previous_start_idx.unwrap() + 1,
                            new: &target_state[previous_target_idx.unwrap() + 1..target_advance],
                        },
                    )
                }
                (start_advance, target_advance) if start_advance > 2 && target_advance > 1 => {
                    Some(SequenceModificationDelta::ReplaceMany {
                        start_idx: previous_start_idx.unwrap() + 1,
                        items_to_replace: start_advance - 1,
                        new: &target_state[previous_target_idx.unwrap() + 1..target_advance],
                    })
                }
                (start_advance, target_advance) if start_advance == 2 && target_advance == 1 => {
                    Some(SequenceModificationDelta::DeleteOne {
                        index: previous_start_idx.unwrap() + 1,
                    })
                }
                (start_advance, target_advance) if start_advance > 2 && target_advance == 1 => {
                    Some(SequenceModificationDelta::DeleteMany {
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
                let modification = SequenceModificationDelta::ReplaceFirst {
                    item: &target_state[0],
                };
                modifications.push(modification);
            } else if start_common_idx > 1 && start_common_idx == target_common_idx {
                let modification = SequenceModificationDelta::ReplaceAllBeforeIncluding {
                    before: start_common_idx - 1,
                    new: &target_state[0..target_common_idx],
                };
                modifications.push(modification);
            } else if start_common_idx == 1 && target_common_idx == 0 {
                let modification = SequenceModificationDelta::DeleteFirst;
                modifications.push(modification);
            } else if start_common_idx > 1 && target_common_idx == 0 {
                let modification = SequenceModificationDelta::DeleteAllBeforeIncluding {
                    end_index: start_common_idx - 1,
                };
                modifications.push(modification);
            } else if start_common_idx == 0 && target_common_idx == 1 {
                let modification = SequenceModificationDelta::PrependOne {
                    item: &target_state[0],
                };
                modifications.push(modification);
            } else if start_common_idx == 0 && target_common_idx > 1 {
                let modification = SequenceModificationDelta::PrependMany {
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
            Some(SequenceModificationDelta::ReplaceLast {
                item: target_state.last().unwrap(),
            })
        } else if start_remaining > 1 && start_remaining == target_remaining {
            Some(SequenceModificationDelta::ReplaceAllAfterIncluding {
                after: previous_start_idx + 1,
                new: &target_state[previous_target_idx + 1..],
            })
        } else if start_remaining == 1 && target_remaining == 0 {
            Some(SequenceModificationDelta::DeleteLast)
        } else if start_remaining > 1 && target_remaining == 0 {
            Some(SequenceModificationDelta::DeleteAllAfterIncluding {
                start_index: previous_start_idx + 1,
            })
        } else if start_remaining == 0 && target_remaining == 1 {
            Some(SequenceModificationDelta::AppendOne {
                item: target_state.last().unwrap(),
            })
        } else if start_remaining == 0 && target_remaining > 1 {
            Some(SequenceModificationDelta::AppendMany {
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

    let did_change = modifications.len() > 0;
    (modifications, MacroOptimizationHints { did_change })
}

#[derive(Copy, Clone)]
struct AdvancedBy {
    start_sequence: usize,
    target_sequence: usize,
}
