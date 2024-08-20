use std::fmt::{self, Write};

use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::scope::Scope;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::FormatCode;

/// Defines a module.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Module {
    /// The module's name.
    pub name: String,
    /// The module's visibility.
    vis: Option<String>,
    /// Module documentation.
    docs: Option<Docs>,
    /// Contents of the module.
    scope: Scope,
}

impl Module {
    /// Return a new, blank module.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let foo_module = Module::new("Foo");
    /// ```
    pub fn new(name: &str) -> Self {
        Module {
            name: name.to_string(),
            vis: None,
            docs: None,
            scope: Scope::new(),
        }
    }

    /// Returns a mutable reference to the module's scope.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// println!("{:?}", foo_module.scope());
    /// ```
    pub fn scope(&mut self) -> &mut Scope {
        &mut self.scope
    }

    /// Set the module visibility.
    ///
    /// # Arguments
    ///
    /// * `vis` - The visibility of the module.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.vis("pub");
    /// ```
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }

    /// Import a type into the module's scope.
    ///
    /// This results in a new `use` statement bein added to the beginning of
    /// the module.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the type to import.
    /// * `ty` - The type to import.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.import("rust_codegen", "Module");
    /// ```
    pub fn import(&mut self, path: &str, ty: &str) -> &mut Self {
        self.scope.import(path, ty);
        self
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_module("Bar");
    /// ```
    pub fn new_module(&mut self, name: &str) -> &mut Module {
        self.scope.new_module(name)
    }

    /// Returns a reference to a module if it is exists in this scope.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module to get.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_module("Bar");
    ///
    /// foo_module.get_module("Bar");
    /// ```
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.scope.get_module(name)
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module to get.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_module("Bar");
    ///
    /// foo_module.get_module("Bar");
    /// ```
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.scope.get_module_mut(name)
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module to get or create if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.get_or_new_module("Bar");
    /// ```
    pub fn get_or_new_module(&mut self, name: &str) -> &mut Module {
        self.scope.get_or_new_module(name)
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    ///
    /// # Arguments
    ///
    /// * `item` - The module to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// let mut bar_module = Module::new("Bar");
    ///
    /// foo_module.push_module(bar_module);
    /// ```
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        self.scope.push_module(item);
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the struct to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_struct("Bar");
    /// ```
    pub fn new_struct(&mut self, name: &str) -> &mut Struct {
        self.scope.new_struct(name)
    }

    /// Push a structure definition.
    ///
    /// # Arguments
    ///
    /// * `item` - The struct definition to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Module,Struct};
    ///
    /// let mut foo_module = Module::new("Foo");
    /// let mut bar_struct = Struct::new("Bar");
    ///
    /// foo_module.push_struct(bar_struct);
    /// ```
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.scope.push_struct(item);
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_fn("bar_fn");
    /// ```
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.scope.new_fn(name)
    }

    /// Push a function definition.
    ///
    /// # Arguments
    ///
    /// * `item` - The function definition to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Function,Module};
    ///
    /// let mut foo_module = Module::new("Foo");
    /// let mut bar_fn = Function::new("bar_fn");
    ///
    /// foo_module.push_fn(bar_fn);
    /// ```
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.scope.push_fn(item);
        self
    }

    /// Push a new enum definition, returning a mutable reference to it.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_enum("Bar");
    /// ```
    pub fn new_enum(&mut self, name: &str) -> &mut Enum {
        self.scope.new_enum(name)
    }

    /// Push an enum definition.
    ///
    /// # Arguments
    ///
    /// * `item` - The enum definition to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Enum,Module};
    ///
    /// let mut foo_module = Module::new("Foo");
    /// let mut bar_enum = Enum::new("Bar");
    ///
    /// foo_module.push_enum(bar_enum);
    /// ```
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.scope.push_enum(item);
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    ///
    /// # Arguments
    ///
    /// * `target` - The impl block to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Module;
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.new_impl("Bar");
    /// ```
    pub fn new_impl(&mut self, target: &str) -> &mut Impl {
        self.scope.new_impl(target)
    }

    /// Push an `impl` block.
    ///
    /// # Arguments
    ///
    /// * `item` - The impl definition to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Impl,Module};
    ///
    /// let mut foo_module = Module::new("Foo");
    /// let mut bar_impl = Impl::new("Bar");
    ///
    /// foo_module.push_impl(bar_impl);
    /// ```
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.scope.push_impl(item);
        self
    }

    /// Push a trait definition.
    ///
    /// # Arguments
    ///
    /// * `item` - The trait to push.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Module,Trait};
    ///
    /// let mut foo_module = Module::new("Foo");
    /// let mut bar_trait = Trait::new("Bar");
    ///
    /// foo_module.push_trait(bar_trait);
    /// ```
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.scope.push_trait(item);
        self
    }
}

impl FormatCode for Module {
    /// Formats the module using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::*;
    ///
    /// let mut dest = String::new();
    /// let mut fmt = Formatter::new(&mut dest);
    ///
    /// let mut foo_module = Module::new("Foo");
    /// foo_module.fmt_code(&mut fmt);
    /// ```
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        write!(fmt, "mod {}", self.name)?;
        fmt.block(|fmt| self.scope.fmt_code(fmt))
    }
}
