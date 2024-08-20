use std::fmt::{self, Write};

use crate::associated_type::AssociatedType;
use crate::bound::Bound;
use crate::formatter::{fmt_bound_rhs, Formatter};
use crate::function::Function;
use crate::type_def::TypeDef;

use crate::r#type::Type;

/// Defines a trait.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Trait {
    /// The type definition of the trait.
    type_def: TypeDef,
    /// The trait's parent types.
    parents: Vec<Type>,
    /// The trait's associated types.
    associated_tys: Vec<AssociatedType>,
    /// The trait's functions.
    fns: Vec<Function>,
    /// The trait's macros.
    macros: Vec<String>,
}

impl Trait {
    /// Return a trait definition with the provided name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let foo_trait = Trait::new("Foo");
    /// ```
    pub fn new(name: &str) -> Self {
        Trait {
            type_def: TypeDef::new(name),
            parents: vec![],
            associated_tys: vec![],
            fns: vec![],
            macros: vec![],
        }
    }

    /// Returns a reference to the type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let foo_trait = Trait::new("Foo");
    /// println!("{:?}", foo_trait.ty());
    /// ```
    pub fn ty(&self) -> &Type {
        &self.type_def.ty
    }

    /// Set the trait visibility.
    ///
    /// # Arguments
    ///
    /// * `vis` - The visibility to set for the trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.vis("pub");
    /// ```
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.generic("T");
    /// ```
    pub fn generic(&mut self, name: &str) -> &mut Self {
        self.type_def.ty.generic(name);
        self
    }

    /// Add a `where` bound to the trait.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the bound.
    /// * `ty` - The type of the bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.bound("A", "String");
    /// ```
    pub fn bound(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.type_def.bound(name, ty);
        self
    }

    /// Add a macro to the trait def (e.g. `"#[async_trait]"`).
    ///
    /// # Arguments
    ///
    /// * `r#macro` - The macro to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.r#macro("async_trait");
    /// ```
    pub fn r#macro(&mut self, r#macro: &str) -> &mut Self {
        self.type_def.r#macro(r#macro);
        self
    }

    /// Add a parent trait.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type of the parent trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.parent("Bar");
    /// ```
    pub fn parent(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.parents.push(ty.into());
        self
    }

    /// Set the trait documentation.
    ///
    /// # Arguments
    ///
    /// * `docs` - The documentation for the trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.doc("Sample trait documentation.");
    /// ```
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add an associated type. Returns a mutable reference to the new
    /// associated type for futher configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the associated type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.associated_type("A");
    /// ```
    pub fn associated_type(&mut self, name: &str) -> &mut AssociatedType {
        self.associated_tys.push(AssociatedType(Bound {
            name: name.to_string(),
            bound: vec![],
        }));

        self.associated_tys.last_mut().unwrap()
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
    /// use rust_codegen::Trait;
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.new_fn("bar_fn");
    /// ```
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        let mut func = Function::new(name);
        func.body = None;

        self.push_fn(func);
        self.fns.last_mut().unwrap()
    }

    /// Push a function definition.
    ///
    /// # Arguments
    ///
    /// * `item` - The function to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Function,Trait};
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// let mut bar_fn = Function::new("bar_fn");
    ///
    /// foo_trait.push_fn(bar_fn);
    /// ```
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.fns.push(item);
        self
    }

    /// Formats the scope using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Formatter,Trait};
    ///
    /// let mut dest = String::new();
    /// let mut fmt = Formatter::new(&mut dest);
    ///
    /// let mut foo_trait = Trait::new("Foo");
    /// foo_trait.fmt(&mut fmt);
    /// ```
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("trait", &self.parents, fmt)?;

        fmt.block(|fmt| {
            let assoc = &self.associated_tys;

            // format associated types
            if !assoc.is_empty() {
                for ty in assoc {
                    let ty = &ty.0;

                    write!(fmt, "type {}", ty.name)?;

                    if !ty.bound.is_empty() {
                        write!(fmt, ": ")?;
                        fmt_bound_rhs(&ty.bound, fmt)?;
                    }

                    writeln!(fmt, ";")?;
                }
            }

            for (i, func) in self.fns.iter().enumerate() {
                if i != 0 || !assoc.is_empty() {
                    writeln!(fmt)?;
                }

                func.fmt(true, fmt)?;
            }

            Ok(())
        })
    }
}
