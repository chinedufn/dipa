//! Various tests for the #[derive(DiffPatch)] macro.

#[macro_use]
extern crate dipa_derive;
#[macro_use]
extern crate serde;

mod enum_with_fields;
mod struct_with_fields;
mod zero_sized_type;
