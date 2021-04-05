//! Exposes various types to the dipa-derive macro.

#![allow(missing_docs)]

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "impl-tester", derive(Debug, PartialEq))]
#[allow(non_camel_case_types)]
pub enum Diff2<A, B> {
    NoChange,
    Change_0(A),
    Change_1(B),
    Change_0_1(A, B),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "impl-tester", derive(Debug, PartialEq))]
#[allow(non_camel_case_types)]
pub enum Diff3<A, B, C> {
    NoChange,
    Change_0(A),
    Change_1(B),
    Change_2(C),
    Change_0_1(A, B),
    Change_1_2(B, C),
    Change_0_2(A, C),
    Change_0_1_2(A, B, C),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "impl-tester", derive(Debug, PartialEq))]
#[allow(non_camel_case_types)]
pub enum Diff4<A, B, C, D> {
    NoChange,
    Change_0(A),
    Change_1(B),
    Change_2(C),
    Change_3(D),
    Change_0_1(A, B),
    Change_0_2(A, C),
    Change_0_3(A, D),
    Change_0_1_2(A, B, C),
    Change_0_1_3(A, B, D),
    Change_0_1_2_3(A, B, C, D),
    Change_0_2_3(A, C, D),
    Change_1_2(B, C),
    Change_1_3(B, D),
    Change_1_2_3(B, C, D),
    Change_2_3(C, D),
}
