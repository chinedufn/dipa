# Space Guarantees

FIXME: Move this documentation to a subchapter within the optimizations chapter where we talk about the one_batch strategy.

## Single Byte No Change Rule

Given a struct or enum type that uses `#[dipa(delta_strategy = "one_batch")]`, or any type in the standard library
that we've implemented the `Diffable` trait for.

Its unchanged delta encoding is guaranteed to be serialize-able to a single byte.

---

Note that this rule is true for nested data structures. If all nested types properly implement
`Diffable` and your root type uses the derive macro, it can be delta encoded down to 1 byte when
it has not changed.

Note that this rule applies to enum **fields** not variants. i.e. in `MyEnum::A(1, 2, 3)`, the
fields are "1, 2, 3". This rule applies to enums with any number f variants.

Your nested types do **not** need to use the `one_batch` strategy. The strategy that they use does
not matter for this rule.

---

In the following code snippet, all three types can be delta compressed down to a single byte
when they haven't changed.

```rust
// If this type has not changed its delta can be serialized to
// a single byte.
#[derive(DiffPatch)]
MyStruct {
    field_a: Vec<f32>,
    field_b: HashMap<u8, HashSet<MyEnum>>,
    field_c: AnotherStruct
}

// If this type has not changed its delta can be serialized to
// a single byte.
// Remember: this rule applies to fields, not variants.
#[derive(DiffPatch, Hash, Eq, Ord)]
enum MyEnum {
    A,
    B([Vec<u8>; 2], i32),
    C { some_field: i128 },
    D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W,
    X, Y, Z
}

// If this type has not changed its delta can be serialized to
// a single byte.
#[derive(DiffPatch)]
struct AnotherStruct(i8, u8, MyEnum);

fn main () {
    let my_struct = make_my_struct();
    let other = make_my_struct();

    let diff = my_struct.create_diff_towards(&other);

    let serialized: Vec<u8> = bincode::options()
        .with_varint_encoding()
        .serialize(&diff)
        .unwrap();

    // True for all types that properly implement the
    // Diffable trait.
    assert_eq!(serialized.len(), 1);
}

fn make_my_struct () -> MyStruct {
    let mut hash_set = HashSet::new();
    hash_set.inset(MyEnum::A);

    let mut field_b = HashMap::new();
    field_b.insert(200, hash_set);

	MyStruct {
        field_a: vec![1., 2., 3., 4., 5.],
        field_b,
        field_c: AnotherStruct(
            -1,
            2,
            MyEnum::B([vec![6, 7], vec![8]], -3)
        )
    }
}
```

### Why 5 fields?

The derive macro generates a `Diffable::Delta` associated type that is an enum containing every possible combination of changed fields.

This means that there are `2<super>n</super>` enum variants that get generated, where `n` is the number of fields in your struct or within an
enum variant. Since this is exponential, it grows quickly.

For now we choose `5` as as starting point for the maximum number of fields that we combine into a single `Delta` enum in this way, but in the future we
will experiment with larger numbers in order to find the sweet spot where the number is as high as it can be before the potential impact on compile
times becomes non-negligible.

We will take real applications that use `#[derive(DiffPatch)]` on non trivial data structures and benchmark the fresh and incremental compile times as `n`
increases.

Or, perhaps we'll expose feature flags that allow you to increase `n` yourself. Or better yet a procedural macro attribute that can configure the `n` value.

Note that your structs can still have more than 5 fields. They'll just diff to `(field_count / 5.0).ceil()` bytes when unchanged.

So, currently, if you have 9 fields in a struct it will diff to 2 bytes when unchanged.
