//! Exposes various types to the dipa-derive macro.

#![allow(missing_docs)]

// TODO: Must structs will have a handful of fields. Let's say <10 or so.
//  We should have different Diff types that can represent different field changes.
//  For example, if a struct has 5 fields there might be some sort of type to represent that
//  fields 1-4 have not changed but field 5 has.
//  We can support every combination of possibly enabled/disabled fields up to structs with 15
//  variants (225 possible combinations)
//  Create a build script to auto generate these types
//
//  TODO: For field counts larger than what we generate enums for we can have our proc macro
//   generate the diff type on the fly.

// TODO: Build script to auto generate up until maybe Diff8 or so

// TODO: Actually .. just have the derive macro generate enum MyStructDiff { ... }

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
