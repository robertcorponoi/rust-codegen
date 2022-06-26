use rust_codegen::*;

#[test]
fn scoped_imports() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .import("bar", "Bar")
        .import("bar", "baz::Baz")
        .import("bar::quux", "quuux::Quuuux")
        .new_struct("Foo")
        .field("bar", "Bar")
        .field("baz", "baz::Baz")
        .field("quuuux", "quuux::Quuuux");

    let expect = r#"
mod foo {
    use bar::{Bar, baz};
    use bar::quux::quuux;

    struct Foo {
        bar: Bar,
        baz: baz::Baz,
        quuuux: quuux::Quuuux,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}