# Implementing Diffable

Here's a look at the `Diffable` trait.

```rust
pub trait Diffable<'p, Other> {
    type Delta;

    type DeltaOwned;

    fn create_delta_towards(&self, end_state: &'p Other)
      -> CreateDeltaTowardsReturn<Self::Delta>;
}
```

The `create_delta_towards` method takes some end value and generates a delta encoding that can later
be used to turn your start value (`&self`) into this end value.

Let's walk through a simple implementation of `Diffable` for an unsigned 128 bit integer.

```rust
impl Diffable<'p, i128> for i128 {
    // Note, the real implementation does not use a reference here since i128
    // is Copy. This is simply for illustration.
    type Delta = Option<&'a i128>;

    type DeltaOwned = Option<i128>;

    fn create_delta_towards(&self, end_state: &'p i128) ->
		CreateDeltaTowardsReturn<Self::Delta> {
	    let mut hints = MacroOptimizationHints {
	        did_change: false
		};

		return if self == end_state {
		    (None, hints)
		} else {
		    hints.did_change = true;
			(Some(end_state), hints)
		};
    }
}
```

The `i128` in `impl Diffable<'p, i128>` is the type that we are diffing our `&self` type against.

A `Diffable` implementation can diff a type against any other type. The main use case for this is being able
to diff `T` and `&T`.

The `Diffable::Delta` associated type is the type of your delta encoding. For simple types like integers
the implementations that `dipa` provides use `Option<T>` as the `Delta`.

For more complex types such as `Vec<T>`, custom data structures are used for the `Delta`.

As we see in `type Delta = Option<&'a i28>` a `Delta` can borrow values from the `Other` type in order to avoid
clones.

> NOTE: The real `dipa` implementation for `i128` does not use a reference for the delta since `i128` is a small copy
> type. It simply uses `Option<i128>`.

`type DeltaOwned` is just the `Delta` type with all of the reference types replaced with owned types.

You'll typically serialize to `Diffable::Delta`, send the bytes across the wire (potentially after further compressing
them using either a general purpose or custom compression algorithm), and then  deserialize them to the
`Diffable::DeserializeOwned` type.

The `did_change` is used by the `#[derive(DiffPatch)]` in the code that it automatically generates for you.
The `Delta` that the macro generates for is an enum with different variants that get used based on which
of those fields within the `Diffable` type have changed.
