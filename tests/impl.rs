use rust_codegen::*;

#[test]
fn impl_with_macros() {
    let mut scope = Scope::new();
    scope.new_struct("Bar");
    let imp = scope.new_impl("Bar");
    imp.impl_trait("Foo");
    imp.r#macro("#[async_trait]");
    imp.r#macro("#[toby_is_cute]");

    let f = imp.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting Toby many times because he is such a good boi\");");

    let expect = r#"
struct Bar;

#[async_trait]
#[toby_is_cute]
impl Foo for Bar {
    async fn pet_toby() {
        println!("petting Toby many times because he is such a good boi");
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
