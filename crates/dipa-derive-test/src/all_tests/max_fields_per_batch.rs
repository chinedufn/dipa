// Verify that max_fields_per_batch allows us to use more than 5 fields with the
// one_batch strategy.
#[derive(DiffPatch)]
#[dipa(max_fields_per_batch = 6, field_batching_strategy = "one_batch")]
struct MyStruct {
    field1: (),
    field2: (),
    field3: (),
    field4: (),
    field5: (),
    field6: (),
}
