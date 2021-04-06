# Optimizations

Over time more and more ideas will pop up on how to make the dipa macro perform more delta encoding optimizations.

This section contains a list of optimizations that we plan to tackle at some point.

## Booleans to Bitflags

If a struct has two or more boolean fields we can use [bitflags] to pack the diffs into a single integer.

TODO: Link to corresponding issue

```rust
#[derive(DiffPatch)]
struct Foo {
    fielda: bool,
    fieldb: bool,
    fieldc: Vec<u32>,
    fieldc: bool
}

// This is just a quick example delta. Needs more planning and design work.
enum FooDelta {
    NoChange,
    Changed_0(BitFlagsU8WithOneBitForEachBoolField),
    Changed_1(DiffForTheVecu32),
    Changed_0_1(BitFlagsU8WithOneBitForEachBoolField, DiffForTheVecu32)
}
```

[bitflags]: https://github.com/bitflags/bitflags

## Global Token Information

Say that you have the following data structures:

```rust
#[derive(DiffPatch)]
struct Outer {
    fielda: bool,
    fieldb: bool,
    inner: Inner
}

#[derive(DiffPatch)]
struct Inner {
    fieldc: bool,
    fieldd: bool
    fielde: HashSet<i8>
}
```

If the derive macro invocations for `Outer` knew about the structure of `Inner` then we could generate code such
that `Outer's` delta looked like this:

```
enum OuterDelta {
    NoChange,
    Change_0(Bitflag U8 that is used to represent all four booleans here),
    // ...
}
```

In the above example we are using a single `u8` bitflag to encode the changes of all of the bools in both `Outer`
and `Inner`.

We could do further than this as well. If `Outer` was given a `#[dipa(max_delta_batch = 6)]` attribute, the derive
macro could generate an `OuterDelta` that was essentially a `Delta6` with all of the combinations of the 6 fields.

There are just two quick examples. There should be other things that we can do when creating a type's delta if we know
information about its nested types.

Essentially, knowing about other types that are using the macro would allow us to perform optimizations that
we could not otherwise.

One way to make this happen would be to make use of the filesystem, but this would mean that whenever you changed your
types you would need to build once before all of the right information was cached. No good.

A better way would be if rustc had support for allowing a procedural macro to execute twice. On the first invocation
it would simply process all of the tokens and cache the information to `env!(OUT_DIR)` or something like that.

Then on the second invocation it could use this cached information in order to apply optimizations like those described
above.

This would need more thought and design and an RFC, but it could be an interesting avenue to pursue.
