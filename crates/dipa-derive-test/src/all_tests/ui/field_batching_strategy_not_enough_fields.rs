use dipa_derive::DiffPatch;

#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "no_batching")]
struct Foo;

#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "no_batching")]
struct Bar(u32);

#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "no_batching")]
struct Bazz {
    field: u64,
}

// TODO: Support field batching strategy on enum variants
//
// #[derive(DiffPatch)]
// enum Buzz {
//     #[dipa(field_batching_strategy = "no_batching")]
//     Variant(i32)
// }
//
// #[derive(DiffPatch)]
// enum Bizz {
//     #[dipa(field_batching_strategy = "no_batching")]
//     Variant {field: u8}
// }
//
// #[derive(DiffPatch)]
// enum Bozz {
//     #[dipa(field_batching_strategy = "no_batching")]
//     Variant
// }

fn main() {}
