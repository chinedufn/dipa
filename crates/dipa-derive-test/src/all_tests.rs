// Make sure that our code generation doesn't generate any warnings.
// When working on tests you can `#[allow(warnings)]` within the test in order to avoid getting
// slowed down by warning compilation errors.
#![deny(warnings)]

mod enum_with_fields;
mod field_batching_strategy;
mod max_fields_per_batch;
mod struct_with_fields;
mod zero_sized_type;

mod public_type;

mod ui;
