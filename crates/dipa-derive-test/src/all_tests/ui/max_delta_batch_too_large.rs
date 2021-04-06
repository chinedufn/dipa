use dipa_derive::DiffPatch;

#[derive(DiffPatch)]
#[dipa(max_delta_batch = 8)]
struct Foo;

fn main() {}
