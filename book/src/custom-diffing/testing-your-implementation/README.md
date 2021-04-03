# Testing Your Implementation

By enabling the `impl-tester` feature, you gain access to machinery that makes it easy to test
your custom `Diffable` and `Patchable` implementations.

The `DiffPatchTestCase` will.

1. Delta encode your provided start and end values.

2. Serialize the delta using `bincode` with variable integer encoding.

3. Assert that the number of bytes is what you expect.

4. Deserialize the delta back to `DeltaOwned`.

5. Apply the delta to your original start value.

6. Ensure that your start value now equals your end value.

```toml
# Cargo.toml

[dev-dependencies]
dipa = {version = "0.1", features = ["impl-tester"]}
```

```rust
use dipa::CreateDeltaTowardsReturn;

struct MyStruct {
    field: u8
}

enum MyDelta<'a> {
   New(&'a u8)
}

enum MyDeltaOwned {
   New(u8)
}

impl Diffable<'p, MyStruct> for MyStruct {
    type Delta = MyDelta;
    type DeltaOwned = MyDeltaOwned;

	fn create_delta_towards (&self) -> CreateDeltaTowardsReturn<Self::Delta> {
	    todo!()
	}
}

impl 

#[cfg(test)]
mod tests {
    use dipa::DiffPatchTestCase;

    #[test]
    fn diff_my_struct_changed() {
        DiffPatchTestCase {
            label: Some("Diff MyStruct changed"),
            start: MyStruct { field: 2 },
            end: &MyStruct { field: 5 },
            expected_delta: MyDelta::New(&5),
            expected_serialized_patch_size: 2,
			expected_macro_hints: MacroOptimizationHints {
			    did_change: true
			},
        }
        .test();
    }

    #[test]
    fn diff_my_struct_no_change() {
        DiffPatchTestCase {
            label: Some("Diff MyStruct no change"),
            start: MyStruct { field: 2 },
            end: &MyStruct { field: 2 },
            expected_delta: MyDelta::New(&2),
            expected_serialized_patch_size: 2,
			expected_macro_hints: MacroOptimizationHints {
			    did_change: false
			}
        }
        .test();
    }

}
```
