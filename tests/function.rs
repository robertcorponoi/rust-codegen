use rust_codegen::*;


#[test]
fn single_fn() {
    let mut scope = Scope::new();
    scope
        .new_fn("my_fn")
        .vis("pub")
        .arg("foo", Type::new("uint"))
        .ret(Type::new("uint"))
        .line("let res = foo + 1;")
        .line("res");

    let expect = r#"
pub fn my_fn(foo: uint) -> uint {
    let res = foo + 1;
    res
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn function_with_async() {
    let mut scope = Scope::new();
    let trt = scope.new_trait("Foo");

    let f = trt.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting toby because he is a good boi\");");

    let expect = r#"
trait Foo {
    async fn pet_toby() {
        println!("petting toby because he is a good boi");
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}