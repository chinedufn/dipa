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

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "dipa-impl-tester", derive(Debug, PartialEq))]
#[allow(non_camel_case_types)]
pub enum Diff2<A, B> {
    NoChange,
    Change_0(A),
    Change_1(B),
    Change_0_1(A, B),
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "dipa-impl-tester", derive(Debug, PartialEq))]
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
