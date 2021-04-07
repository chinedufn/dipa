#[derive(DiffPatch)]
#[dipa(field_batching_strategy = "no_batching")]
struct NoBatching {
    field_a: u16,
    field_b: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the delta types for a struct with no field batching are properly created.
    /// The no_batching strategy creates a delta struct with one field per original field, so
    /// here we very that that is the case.
    #[test]
    fn no_batching_delta() {
        // let _ = NoBatchingDelta {
        //     field_a: Some(0u16),
        //     field_b: Some(0u32),
        // };
        //
        // let _ = NoBatchingDeltaOwned {
        //     field_a: Some(0u16),
        //     field_b: Some(0u32),
        // };
    }
}
