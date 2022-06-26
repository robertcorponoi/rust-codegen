use crate::function::Function;
use crate::module::Module;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;

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
