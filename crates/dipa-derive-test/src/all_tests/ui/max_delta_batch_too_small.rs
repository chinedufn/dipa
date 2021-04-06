use dipa_derive::DiffPatch;

#[derive(DiffPatch)]
#[dipa(max_delta_batch = 1)]
struct Foo;

fn main() {}
