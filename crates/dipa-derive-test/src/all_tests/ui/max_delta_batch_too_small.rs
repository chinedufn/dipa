use dipa_derive::DiffPatch;

#[derive(DiffPatch)]
#[dipa(max_fields_per_batch = 1)]
struct Foo;

fn main() {}
