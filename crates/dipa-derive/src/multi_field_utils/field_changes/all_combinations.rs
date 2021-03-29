use crate::multi_field_utils::ChangedFieldIndices;
use std::cmp::Ordering;

const FALSE_TRUE: [bool; 2] = [false, true];

impl ChangedFieldIndices {
    /// Generate all of the combinations of changed indices.
    ///
    /// So for 2 fields it would be
    ///     [0], [1], and [0, 1]
    pub fn all_changed_index_combinations(field_count: usize) -> Vec<ChangedFieldIndices> {
        let bool_combinations = make_bool_combinations(field_count);

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

/// TODO: Refactor to a recursive building of the combinations when we add support for more
///  fields.
pub(in super::super) fn make_bool_combinations(bool_count: usize) -> Vec<Vec<bool>> {
    match bool_count {
        1 => make_bool_combinations_1(),
        2 => make_bool_combinations_2(),
        3 => make_bool_combinations_3(),
        4 => make_bool_combinations_4(),
        _ => panic!(
            r#"
TODO: Support larger structs.
"#
        ),
    }
}

fn make_bool_combinations_1() -> Vec<Vec<bool>> {
    vec![vec![false], vec![true]]
}

fn make_bool_combinations_2() -> Vec<Vec<bool>> {
    let mut bool_combinations = Vec::with_capacity(4);

    for field0 in FALSE_TRUE.iter() {
        for field1 in FALSE_TRUE.iter() {
            let bools = vec![*field0, *field1];
            bool_combinations.push(bools);
        }
    }

    bool_combinations
}

fn make_bool_combinations_3() -> Vec<Vec<bool>> {
    let mut bool_combinations = Vec::with_capacity(8);

    for field0 in FALSE_TRUE.iter() {
        for field1 in FALSE_TRUE.iter() {
            for field2 in FALSE_TRUE.iter() {
                let bools = vec![*field0, *field1, *field2];
                bool_combinations.push(bools);
            }
        }
    }

    bool_combinations
}

fn make_bool_combinations_4() -> Vec<Vec<bool>> {
    let mut bool_combinations = Vec::with_capacity(8);

    for field0 in FALSE_TRUE.iter() {
        for field1 in FALSE_TRUE.iter() {
            for field2 in FALSE_TRUE.iter() {
                for field3 in FALSE_TRUE.iter() {
                    let bools = vec![*field0, *field1, *field2, *field3];
                    bool_combinations.push(bools);
                }
            }
        }
    }

    bool_combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that we properly generate all of the change variants for a 2 field struct/tuple
    #[test]
    fn change_names_2_fields() {
        assert_eq!(
            ChangedFieldIndices::all_changed_index_combinations(2),
            vec![
                ChangedFieldIndices::new(vec![]),
                ChangedFieldIndices::new(vec![0]),
                ChangedFieldIndices::new(vec![1]),
                ChangedFieldIndices::new(vec![0, 1]),
            ]
        )
    }
}
