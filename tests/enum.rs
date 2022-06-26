use rust_codegen::*;

#[test]
fn enum_with_repr() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .repr("u8")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[repr(u8)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn enum_with_allow() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .allow("dead_code")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn enum_with_multiple_allow() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .allow("dead_code")
        .allow("clippy::all")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[allow(dead_code)]
#[allow(clippy::all)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}