use std::path::PathBuf;

const LETTERS: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];

// Enums will have 2^N fields, so we default to a small number and expose feature flags to generate
// larger `DeltaN` types.
// TODO: Expose those feature flags. Such as `delta-5` `delta-6` etc. Experiment to see how large
//  we can go without impacting compile times too much.
const MAX_DELTA_N: u8 = 4;

fn main() {
    let delta_n_types = generate_delta_n_types();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(
        PathBuf::from(out_dir).join("delta_n_types.rs"),
        delta_n_types,
    )
    .unwrap();
}

/// Generate `DeltaN` types.
///
/// ```no_run
/// #[derive(serde::Serialize)]
/// #[cfg_attr(feature = "impl-tester", derive(Debug, PartialEq))]
/// #[allow(non_camel_case_types, missing_docs)]
/// pub enum Delta2<A, B> {
///     NoChange,
///     Change_0(A),
///     Change_1(B),
///     Change_0_1(A, B),
/// }
///
/// #[derive(serde::Deserialize)]
/// #[allow(non_camel_case_types, missing_docs)]
/// pub enum DiffOwned2<A, B> {
///     NoChange,
///     Change_0(A),
///     Change_1(B),
///     Change_0_1(A, B),
/// }
/// ```
fn generate_delta_n_types() -> String {
    let mut all_types = "".to_string();

    for field_count in 2..=MAX_DELTA_N {
        let bool_combinations = make_bool_combinations(field_count as _);

        let mut change_combinations = "".to_string();
        let diff_n_generics: String = LETTERS[0..field_count as usize]
            .iter()
            .map(|l| format!("{},", l))
            .collect();

        for bools in bool_combinations {
            let mut change_str = "Change".to_string();
            let mut changed_generics = "".to_string();

            bools
                .iter()
                .enumerate()
                .filter(|(_, changed)| **changed)
                .for_each(|(idx, _)| {
                    change_str += &format!("_{}", idx);
                    changed_generics += &format!("{}, ", LETTERS[idx]);
                });

            if &change_str == "Change" {
                continue;
            }

            change_combinations += &format!(
                r#"{change_str}({changed_generics}),
    "#,
                change_str = change_str,
                changed_generics = changed_generics
            );
        }

        let diff_n = format!(
            r#"
#[derive(serde::Serialize)]
#[cfg_attr(feature = "impl-tester", derive(Debug, PartialEq))]
#[allow(non_camel_case_types, missing_docs)]
pub(crate) enum Delta{field_count}<{diff_n_generics}> {{
    NoChange,
    {change_combinations}
}}"#,
            field_count = field_count,
            change_combinations = change_combinations,
            diff_n_generics = diff_n_generics,
        );

        let diff_n_owned = format!(
            r#"
#[derive(serde::Deserialize)]
#[allow(non_camel_case_types, missing_docs)]
pub(crate) enum DeltaOwned{field_count}<{diff_n_generics}> {{
    NoChange,
    {change_combinations}
}}"#,
            field_count = field_count,
            change_combinations = change_combinations,
            diff_n_generics = diff_n_generics,
        );

        all_types += &diff_n;
        all_types += &diff_n_owned;
    }

    all_types
}

/// Every possible combination of `n` booleans being true or false
/// There are `2 ^ field_count` combinations.
///
/// So for two fields the four combinations are:
///
/// [false, false], [true, false], [false, true], [true, true]
fn make_bool_combinations(field_count: usize) -> Vec<Vec<bool>> {
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
