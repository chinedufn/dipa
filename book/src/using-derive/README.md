# Using Derive

dipa provides a derive macro to generate implementations of the `Diffable` and `Patchable` traits for
data structures defined in your crate.

To enable the macro use the `derive` feature. Then use `#[derive(DiffPatch)]` on types that you want to
be able to delta encode.

```toml
# Cargo.toml

# ...

[dependencies]
dipa = { version = "0.x", features = ["derive"] }
serde = { version = "1", features = ["derive"] }

# ...
```

```rust
# lib.rs

use dipa::DiffPatch;

#[derive(DiffPatch)]
struct MyStruct {
    field_a: MyEnum,
    field_b: Vec<f64>
}

#[derive(DiffPatch)]
struct MyEnum {
    field: Vec<f64>
}
```
