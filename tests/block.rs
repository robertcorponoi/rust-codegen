use rust_codegen::*;

#[test]
fn block_one_line() {
    let mut scope = Scope::new();
    let new_fn = scope.new_fn("hello_world");

    let mut block = Block::new("");
    block.line("println!(\"Hello, world!\");");

    new_fn.push_block(block);

    let expected = r#"
fn hello_world() {
    {
        println!("Hello, world!");
    }
}"#;

    assert_eq!(scope.to_string(), &expected[1..]);
}

#[test]
fn block_multiple_lines() {
    let mut scope = Scope::new();
    let new_fn = scope.new_fn("hello_world");

    let mut block = Block::new("");
    block.line("println!(\"Hello, world!\");");
    block.line("println!(\"from Rust!\");");

    new_fn.push_block(block);

    let expected = r#"
fn hello_world() {
    {
        println!("Hello, world!");
        println!("from Rust!");
    }
}"#;

    assert_eq!(scope.to_string(), &expected[1..]);
}
