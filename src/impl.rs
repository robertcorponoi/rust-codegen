use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::field::Field;
use crate::formatter::{fmt_bounds, fmt_generics, Formatter};
use crate::function::Function;

use crate::r#type::Type;

/// Defines an impl block.
#[derive(Debug, Clone)]
pub struct Impl {
    /// The struct being implemented.
    target: Type,
    /// Impl level generics.
    generics: Vec<String>,
    /// If implementing a trait.
    impl_trait: Option<Type>,
    /// Associated types.
    assoc_tys: Vec<Field>,
    /// The bounds of the impl block.
    bounds: Vec<Bound>,
    /// The functions for the impl block.
    fns: Vec<Function>,
    /// The macros for the impl block.
    macros: Vec<String>,
}

impl Impl {
    /// Returns a new impl definition.
    ///
    /// # Arguments
    ///
    /// * `target` - The impl's target.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let foo_impl = Impl::new("Foo");
    /// ```
    pub fn new(target: impl Into<Type>) -> Self {
        Impl {
            target: target.into(),
            generics: vec![],
            impl_trait: None,
            assoc_tys: vec![],
            bounds: vec![],
            fns: vec![],
            macros: vec![],
        }
    }

    /// Add a generic to the impl block.
    ///
    /// This adds the generic for the block (`impl<T>`) and not the target
    /// type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the generic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.generic("T");
    /// ```
    pub fn generic(&mut self, name: &str) -> &mut Self {
        self.generics.push(name.to_string());
        self
    }

    /// Add a generic to the target type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The generic type to add to the target.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.target_generic("T");
    /// ```
    pub fn target_generic(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.target.generic(ty);
        self
    }

    /// Set the trait that the impl block is implementing.
    ///
    /// # Arguments
    ///
    /// * `ty` - The trait that the impl block is implementing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.impl_trait("T");
    /// ```
    pub fn impl_trait(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.impl_trait = Some(ty.into());
        self
    }

    /// Add a macro to the impl block (e.g. `"#[async_trait]"`)
    ///
    /// # Arguments
    ///
    /// * `macro` - The macro to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.r#macro("async_trait");
    /// ```
    pub fn r#macro(&mut self, r#macro: &str) -> &mut Self {
        self.macros.push(r#macro.to_string());
        self
    }

    /// Set an associated type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the associated type.
    /// * `ty` - The type of the associated type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::*;
    ///
    /// let mut scope = Scope::new();
    ///
    /// let trait_foo = scope.new_trait("Foo");
    /// let mut impl_bar = Impl::new("Bar");
    ///
    /// impl_bar.associate_type("A", "Foo");
    /// ```
    pub fn associate_type(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.assoc_tys.push(Field {
            name: name.to_string(),
            ty: ty.into(),
            documentation: Vec::new(),
            annotation: Vec::new(),
        });

        self
    }

    /// Add a `where` bound to the impl block.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the bound.
    /// * `ty` - The type of the bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.bound("T", "Default");
    pub fn bound(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.bounds.push(Bound {
            name: name.to_string(),
            bound: vec![ty.into()],
        });
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Impl;
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.new_fn("bar_fn");
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.push_fn(Function::new(name));
        self.fns.last_mut().unwrap()
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
    /// use rust_codegen::{Function,Impl};
    ///
    /// let mut foo_impl = Impl::new("Foo");
    /// let bar_fn = Function::new("bar");
    ///
    /// foo_impl.push_fn(bar_fn);
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.fns.push(item);
        self
    }

    /// Formats the impl block using the given formatter.
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
    /// let mut foo_impl = Impl::new("Foo");
    /// foo_impl.fmt( &mut fmt);
    /// ```
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.macros.iter() {
            writeln!(fmt, "{}", m)?;
        }
        write!(fmt, "impl")?;
        fmt_generics(&self.generics[..], fmt)?;

        if let Some(ref t) = self.impl_trait {
            write!(fmt, " ")?;
            t.fmt(fmt)?;
            write!(fmt, " for")?;
        }

        write!(fmt, " ")?;
        self.target.fmt(fmt)?;

        fmt_bounds(&self.bounds, fmt)?;

        fmt.block(|fmt| {
            // format associated types
            if !self.assoc_tys.is_empty() {
                for ty in &self.assoc_tys {
                    write!(fmt, "type {} = ", ty.name)?;
                    ty.ty.fmt(fmt)?;
                    writeln!(fmt, ";")?;
                }
            }

            for (i, func) in self.fns.iter().enumerate() {
                if i != 0 || !self.assoc_tys.is_empty() {
                    writeln!(fmt)?;
                }

                func.fmt(false, fmt)?;
            }

            Ok(())
        })
    }
}
