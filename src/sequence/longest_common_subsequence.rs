use std::ops::Deref;

/// Get the indices of the longest common subsequence between two vectors.
///
/// FIXME: Dynamic programming instead of recursive.
pub(super) fn get_longest_common_subsequence<'a, T: PartialEq>(
    left: &'a [T],
    right: &'a [T],
) -> Vec<(usize, usize)> {
    lcs_recursive(
        LcsWrapper { list: left, idx: 0 },
        LcsWrapper {
            list: right,
            idx: 0,
        },
        vec![],
    )
}

fn lcs_recursive<'a, T: PartialEq>(
    left: LcsWrapper<'a, T>,
    right: LcsWrapper<'a, T>,
    mut holder: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    match (left.get(left.idx), right.get(right.idx)) {
        (Some(l), Some(r)) => {
            if l == r {
                holder.push((left.idx, right.idx));

                let mut next_left = left;
                next_left.idx += 1;

                let mut next_right = right;
                next_right.idx += 1;

                lcs_recursive(next_left, next_right, holder)
            } else {
                let mut less_left = left;
                less_left.idx += 1;

                let mut less_right = right;
                less_right.idx += 1;

                let left_holder = holder.clone();
                let left_attempt = lcs_recursive(less_left, right, left_holder);

                let right_holder = holder.clone();
                let right_attempt = lcs_recursive(left, less_right, right_holder);

                return if left_attempt.len() > right_attempt.len() {
                    left_attempt
                } else {
                    right_attempt
                };
            }
        }
        _ => holder,
    }
}

struct LcsWrapper<'a, T> {
    list: &'a [T],
    idx: usize,
}

impl<'a, T> Copy for LcsWrapper<'a, T> {}
impl<'a, T> Clone for LcsWrapper<'a, T> {
    fn clone(&self) -> Self {
        Self {
            list: self.list,
            idx: self.idx,
        }
    }
}

impl<'a, T: PartialEq> Deref for LcsWrapper<'a, T> {
    type Target = &'a [T];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &'static str = "A";
    const B: &'static str = "B";
    const C: &'static str = "C";
    const D: &'static str = "D";

    /// Verify that we properly determine longest common subsequences.
    #[test]
    fn longest_common_subsequences() {
        for (left, right, expected) in vec![
            (vec![], vec![], vec![]),
            (vec![D, C], vec![A, B, C], vec![(1, 2)]),
            (vec![A, B, D, C], vec![A, A, A, D], vec![(0, 0), (2, 3)]),
            (
                vec![A, B, C, D],
                vec![A, B, C],
                vec![(0, 0), (1, 1), (2, 2)],
            ),
        ] {
            assert_eq!(get_longest_common_subsequence(&left, &right), expected);
        }
    }
}
