/// Get the indices of the longest common subsequence between two vectors.
///
/// Uses the dynamic programming approach to the LCS problem.
///
/// FIXME: Refactor this. Messy.
pub(super) fn get_longest_common_subsequence<'a, T: PartialEq>(
    left: &'a [T],
    right: &'a [T],
) -> Vec<(usize, usize)> {
    let mut solution = vec![];

    if left.len() == 0 || right.len() == 0 {
        return solution;
    }

    let mut lcs_table = LcsTable {
        right_len: right.len(),
        subproblem_solutions: vec![
            SubproblemSolution {
                lcs_length: 0,
                // Will be overwritten, just initializing a value.
                next_subproblem: NextSubproblem::ReduceLeft,
            };
            left.len() * right.len()
        ],
    };

    for cur_left in 0..left.len() {
        for cur_right in 0..right.len() {
            if left[cur_left] == right[cur_right] {
                let lcs_length = if cur_left > 0 && cur_right > 0 {
                    lcs_table.lcs_length_at_idx(cur_left - 1, cur_right - 1) + 1
                } else {
                    1
                };

                lcs_table.set(
                    cur_left,
                    cur_right,
                    SubproblemSolution {
                        lcs_length,
                        next_subproblem: NextSubproblem::ReduceLeftAndRight,
                    },
                )
            } else if cur_left > 0
                && cur_right > 0
                && lcs_table.lcs_length_at_idx(cur_left - 1, cur_right)
                    >= lcs_table.lcs_length_at_idx(cur_left, cur_right - 1)
            {
                lcs_table.set(
                    cur_left,
                    cur_right,
                    SubproblemSolution {
                        lcs_length: lcs_table.lcs_length_at_idx(cur_left - 1, cur_right),
                        next_subproblem: NextSubproblem::ReduceLeft,
                    },
                )
            } else if cur_right > 0 {
                lcs_table.set(
                    cur_left,
                    cur_right,
                    SubproblemSolution {
                        lcs_length: lcs_table.lcs_length_at_idx(cur_left, cur_right - 1),
                        next_subproblem: NextSubproblem::ReduceRight,
                    },
                )
            }
        }
    }

    let mut left_idx = left.len() - 1;
    let mut right_idx = right.len() - 1;

    loop {
        let subproblem = lcs_table.get(left_idx, right_idx);

        match subproblem.next_subproblem {
            NextSubproblem::ReduceLeftAndRight => {
                solution.push((left_idx, right_idx));

                if left_idx > 0 && right_idx > 0 {
                    left_idx -= 1;
                    right_idx -= 1;
                } else {
                    break;
                }
            }
            NextSubproblem::ReduceLeft => {
                if left_idx > 0 {
                    left_idx -= 1;
                } else {
                    break;
                }
            }
            NextSubproblem::ReduceRight => {
                if right_idx > 0 {
                    right_idx -= 1;
                } else {
                    break;
                }
            }
        };
    }

    solution.reverse();
    solution
}

struct LcsTable {
    right_len: usize,
    subproblem_solutions: Vec<SubproblemSolution>,
}

impl LcsTable {
    fn get(&self, left: usize, right: usize) -> SubproblemSolution {
        self.subproblem_solutions[self.idx_at(left, right)]
    }

    fn lcs_length_at_idx(&self, left: usize, right: usize) -> usize {
        self.subproblem_solutions
            .get(self.idx_at(left, right))
            .map(|s| s.lcs_length)
            .unwrap_or(0)
    }

    fn set(&mut self, left: usize, right: usize, solution: SubproblemSolution) {
        let idx = self.idx_at(left, right);
        self.subproblem_solutions[idx] = solution;
    }

    fn idx_at(&self, left: usize, right: usize) -> usize {
        (left * self.right_len) + right
    }
}

#[derive(Copy, Clone)]
struct SubproblemSolution {
    lcs_length: usize,
    next_subproblem: NextSubproblem,
}

#[derive(Copy, Clone)]
enum NextSubproblem {
    ReduceLeftAndRight,
    ReduceLeft,
    ReduceRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &'static str = "A";
    const B: &'static str = "B";
    const C: &'static str = "C";
    const D: &'static str = "D";
    const E: &'static str = "E";

    /// Verify that we properly determine longest common subsequences.
    #[test]
    fn longest_common_subsequences() {
        for (idx, (left, right, expected)) in vec![
            (vec![], vec![], vec![]),
            (vec![D, C], vec![A, B, C], vec![(1, 2)]),
            (vec![A, B, D, C], vec![A, A, A, D], vec![(0, 2), (2, 3)]),
            (
                vec![A, B, C, D],
                vec![A, B, C],
                vec![(0, 0), (1, 1), (2, 2)],
            ),
            (vec![A, B, C, D], vec![C, D], vec![(2, 0), (3, 1)]),
            (vec![C, D], vec![A, B, C, D], vec![(0, 2), (1, 3)]),
            (vec![A, B, C, D, E], vec![C], vec![(2, 0)]),
            (vec![C], vec![A, B, C, D, E], vec![(0, 2)]),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(
                get_longest_common_subsequence(&left, &right),
                expected,
                "Test at index {} failed.",
                idx
            );
        }
    }
}
