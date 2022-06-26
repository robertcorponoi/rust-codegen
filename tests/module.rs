use rust_codegen::*;

#[test]
fn module_mut() {
    let mut scope = Scope::new();
    scope.new_module("foo").import("bar", "Bar");

    scope
        .get_module_mut("foo")
        .expect("module_mut")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn get_or_new_module() {
    let mut scope = Scope::new();
    assert!(scope.get_module("foo").is_none());

    scope.get_or_new_module("foo").import("bar", "Bar");

    scope
        .get_or_new_module("foo")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}