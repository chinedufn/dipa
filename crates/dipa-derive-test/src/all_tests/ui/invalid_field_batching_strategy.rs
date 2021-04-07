use dipa_derive::DiffPatch;

#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "does-not-exist")]
struct Foo;

fn main() {}
