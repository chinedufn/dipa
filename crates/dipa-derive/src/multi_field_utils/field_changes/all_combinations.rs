use crate::dipa_attribute::DipaAttrs;
use crate::multi_field_utils::ChangedFieldIndices;
use std::cmp::Ordering;

impl ChangedFieldIndices {
    /// Generate all of the combinations of changed indices.
    ///
    /// So for 2 fields it would be
    ///     [0], [1], and [0, 1]
    pub fn all_changed_index_combinations(
        field_count: usize,
        dipa_attrs: &DipaAttrs,
    ) -> Vec<ChangedFieldIndices> {
        let bool_combinations =
            make_bool_combinations(field_count, dipa_attrs.max_fields_per_batch);

        let mut all_combinations = vec![];

        for bools in bool_combinations {
            let mut changed_indices = vec![];

            for (idx, _bool) in bools
                .iter()
                .enumerate()
                .filter(|(_, did_change)| **did_change)
            {
                changed_indices.push(idx as u8);
            }

            if changed_indices.len() == 0 {
                continue;
            }

            all_combinations.push(ChangedFieldIndices::new(changed_indices));
        }

        all_combinations.sort_by(|a, b| {
            let ordering = a.len().cmp(&b.len());

            if ordering == Ordering::Equal {
                for (a, b) in a.iter().zip(b.iter()) {
                    let cmp = a.cmp(b);

                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
            }

            ordering
        });

        all_combinations
    }
}

/// Create every combination of true and false.
/// There are `2 ^ field_count` combinations.
///
/// So for two fields the four combinations are:
///
///   [false, false], [true, false], [false, true], [true, true]
pub(in crate) fn make_bool_combinations(
    field_count: usize,
    max_fields_per_batch: Option<u8>,
) -> Vec<Vec<bool>> {
    if field_count > max_fields_per_batch.unwrap_or(5) as usize {
        todo!(
            r#"
Compile time error either telling you to use a different strategy or increase the
max_fields_per_batch
"#
        )
    }

    let mut all = vec![];
    let start = vec![false; field_count];

    bool_combinations_recursive(&mut all, 0, field_count, start);

    all
}

fn bool_combinations_recursive(
    all: &mut Vec<Vec<bool>>,
    start_idx: usize,
    field_count: usize,
    current: Vec<bool>,
) {
    if start_idx > field_count {
        return;
    }

    all.push(current.clone());

    for idx in start_idx..field_count {
        let mut flipped = current.clone();

        flipped[idx] = !flipped[idx];

        bool_combinations_recursive(all, idx + 1, field_count, flipped);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that we properly generate all of the change variants for a 2 field struct/tuple
    #[test]
    fn change_names_2_fields() {
        assert_eq!(
            ChangedFieldIndices::all_changed_index_combinations(2, &DipaAttrs::default()),
            vec![
                ChangedFieldIndices::new(vec![]),
                ChangedFieldIndices::new(vec![0]),
                ChangedFieldIndices::new(vec![1]),
                ChangedFieldIndices::new(vec![0, 1]),
            ]
        )
    }
}
