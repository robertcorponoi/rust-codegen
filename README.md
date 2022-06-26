# Rust Codegen

Rust Codegen aims to help you generate Rust code programmatically with a simple builder API.

## Installation

To use `rust-codegen`, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-codegen = "0.1.1"
```

## Usage

While usage can vary based on what you need, a basic flow is creating a `Scope` and then adding what you need onto it. Below is a simple example of creating a struct with a couple of fields.

```rust
use rust_codegen::Scope;

// A `Scope` is the root of the builder. Everything should be added on to it.
let mut scope = Scope::new();

// Creates a new struct named Foo that derives from `Debug` and have two fields.
scope.new_struct("Foo")
    .derive("Debug")
    .field("one", "usize")
    .field("two", "String");

// Once turned into a string, the above looks like:
// #[derive(Debug)]
// struct Foo {
//    one: usize,
//    two: String,
// }
println!("{}", scope.to_string());
```

**Note:** You should not rely on the formatted output as it's very basic. You should instead run the generated code through `rustfmt`.

Make sure to check out the [documentation](https://github.com/robertcorponoi/rust-codegen) for all of the available features with examples.

## Acknowledgements

This was originally a fork of [carllerche's codegen repo](https://github.com/carllerche/codegen) with some updates. However, due to the amount of updates and the fact that I needed to publish it on crates.io for other projects, I made it its own thing.

## License

[MIT](./LICENSE)
