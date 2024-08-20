#![deny(missing_debug_implementations, missing_docs)]
#![doc(html_root_url = "https://docs.rs/rust_codegen/0.1.0")]

//! Provides a builder API for generating Rust code.
//!
//! The general strategy for using the crate is as follows:
//!
//! 1. Create a `Scope` instance.
//! 2. Use the builder API to add elements to the scope.
//! 3. Call `to_string()` method of the `FormatCode` Trait to get the generated code.
//!
//! For example:
//!
//! ```rust
//! use rust_codegen::{Scope, FormatCode};
//!
//! let mut scope = Scope::new();
//!
//! scope.new_struct("Foo")
//!     .derive("Debug")
//!     .field("one", "usize")
//!     .field("two", "String");
//!
//! println!("{}", scope.to_string());
//! ```

mod associated_type;
mod block;
mod body;
mod bound;
mod docs;
mod field;
mod fields;
mod formatter;
mod function;
mod import;
mod item;
mod module;
mod scope;
mod type_def;
mod variant;

mod r#enum;
mod r#impl;
mod r#struct;
mod r#trait;
mod r#type;

pub use associated_type::*;
pub use block::*;
pub use field::*;
pub use formatter::*;
pub use function::*;
pub use import::*;
pub use module::*;
pub use scope::*;
pub use variant::*;

pub use r#enum::*;
pub use r#impl::*;
pub use r#struct::*;
pub use r#trait::*;
pub use r#type::*;

macro_rules! impl_display {
    ($struct:ty) => {
        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut my_fmt = crate::formatter::Formatter::new(f);
                crate::formatter::FormatCode::fmt_code(self, &mut my_fmt)?;
                Ok(())
            }
        }
    };
}

impl_display!(Block);
impl_display!(Enum);
impl_display!(Function);
impl_display!(Impl);
impl_display!(Module);
impl_display!(Scope);
impl_display!(Struct);
impl_display!(Trait);
impl_display!(Type);
impl_display!(Variant);

impl_display!(docs::Docs);
impl_display!(body::Body);
impl_display!(fields::Fields);
impl_display!(item::Item);
