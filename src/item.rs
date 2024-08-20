use crate::function::Function;
use crate::module::Module;
use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::FormatCode;
use std::fmt::Write;

/// The items that can be created with the Scope.
#[derive(Debug, Clone)]
pub enum Item {
    Module(Module),
    Struct(Struct),
    Function(Function),
    Trait(Trait),
    Enum(Enum),
    Impl(Impl),
    Raw(String),
}

impl FormatCode for Item {
    fn fmt_code(&self, fmt: &mut crate::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Item::Module(ref v) => v.fmt_code(fmt),
            Item::Struct(ref v) => v.fmt_code(fmt),
            Item::Function(ref v) => v.fmt_code(fmt),
            Item::Trait(ref v) => v.fmt_code(fmt),
            Item::Enum(ref v) => v.fmt_code(fmt),
            Item::Impl(ref v) => v.fmt_code(fmt),
            Item::Raw(ref v) => {
                writeln!(fmt, "{}", v)
            }
        }
    }
}
