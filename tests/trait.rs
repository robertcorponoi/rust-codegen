use rust_codegen::*;

#[test]
fn trait_with_macros() {
    let mut scope = Scope::new();
    let trt = scope.new_trait("Foo");
    trt.r#macro("#[async_trait]");
    trt.r#macro("#[toby_is_cute]");

    let f = trt.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting toby because he is a good boi\");");

    let expect = r#"
#[async_trait]
#[toby_is_cute]
trait Foo {
    async fn pet_toby() {
        println!("petting toby because he is a good boi");
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
