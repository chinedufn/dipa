# Attributes

There are three categories of attributes that can be used with the derive macro. Container, variant and field
attributes.

```rust
#[derive(DiffPatch)]
#[dipa(diff_derives = "Debug, Copy, Clone")]  // <-- this is a container attribute
struct S {
    #[dipa(todo_field_attribute_here)]  // <-- this is a field attribute
    f: i32,
}

#[derive(DiffPatch)]
#[dipa(patch_derives = "Debug, Serialize")]  // <-- this is a container attribute
enum E {
    #[dipa(todo_variant_attribute_here)]  // <-- this is a variant attribute
    A(String),
}
```

## Container Attributes

- `diff_derives = "SomeDerive, AnotherDerive"`
   
Used to add #[derive()]'s for the delta encoded diff type that dipa generates for your struct or enum.

One example use case is to derive `Debug` for the generated diff type if for some reason you want to be
able to print it out.

- `patch_derives = "SomeDerive, AnotherDerive"`

Used to add #[derive()]'s for the delta encoded diff type that dipa generates for your struct or enum.

One example use case is to derive `Debug` for the generated patch type if for some reason you want to be
able to print it out.
