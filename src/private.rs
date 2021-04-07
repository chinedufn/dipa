//! Exposes various types to the dipa-derive macro.

#![allow(missing_docs)]

// Types such as:
//
// #[derive(serde::Serialize, serde::Deserialize)]
// #[cfg_attr(feature = "impl-tester", derive(Debug, PartialEq))]
// #[allow(non_camel_case_types, missing_docs)]
// pub(crate) enum Diff3<A, B, C> {
//     NoChange,
//     Change_0(A),
//     Change_1(B),
//     Change_2(C),
//     Change_0_1(A, B),
//     Change_1_2(B, C),
//     Change_0_2(A, C),
//     Change_0_1_2(A, B, C),
// }
include!(concat!(env!("OUT_DIR"), "/delta_n_types.rs"));
