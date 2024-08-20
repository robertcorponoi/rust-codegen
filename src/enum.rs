use std::fmt;

use crate::formatter::Formatter;
use crate::type_def::TypeDef;
use crate::variant::Variant;

use crate::r#type::Type;
use crate::FormatCode;

/// Defines an enumeration.
#[derive(Debug, Clone)]
pub struct Enum {
    type_def: TypeDef,
    variants: Vec<Variant>,
}

impl Enum {
    /// Returns a enum definition with the provided name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let foo_enum = Enum::new("Foo");
    /// ```
    pub fn new(name: &str) -> Self {
        Enum {
            type_def: TypeDef::new(name),
            variants: vec![],
        }
    }

    /// Returns a reference to the enum's type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let foo_enum = Enum::new("Foo");
    /// println!("{:?}", foo_enum.ty());
    /// ```
    pub fn ty(&self) -> &Type {
        &self.type_def.ty
    }

    /// Set the enum's visibility.
    ///
    /// # Arguments
    ///
    /// * `vis` - The visibility of the enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.vis("pub");
    /// ```
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the generic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.generic("T");
    /// ```
    pub fn generic(&mut self, name: &str) -> &mut Self {
        self.type_def.ty.generic(name);
        self
    }

    /// Add a `where` bound to the enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the bound.
    /// * `ty` - The type of the bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.bound("T", "Default");
    /// ```
    pub fn bound(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.type_def.bound(name, ty);
        self
    }

    /// Set the enum's documentation.
    ///
    /// # Arguments
    ///
    /// * `docs` - The docs to set for the enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.doc("Sample Foo enum documentation");
    /// ```
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add a new type that the enum should derive.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the derive.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.derive("Debug");
    /// ```
    pub fn derive(&mut self, name: &str) -> &mut Self {
        self.type_def.derive(name);
        self
    }

    /// Specify lint attribute to supress a warning or error.
    ///
    /// # Arguments
    ///
    /// * `allow` - The lint attribute to apply.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.allow("dead_code");
    /// ```
    pub fn allow(&mut self, allow: &str) -> &mut Self {
        self.type_def.allow(allow);
        self
    }

    /// Specify representation.
    ///
    /// # Arguments
    ///
    /// * `repr` - The representation to specify.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.repr("C");
    /// ```
    pub fn repr(&mut self, repr: &str) -> &mut Self {
        self.type_def.repr(repr);
        self
    }

    /// Push a variant to the enum, returning a mutable reference to it.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Enum;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    /// foo_enum.new_variant("FirstVariant");
    /// ```
    pub fn new_variant(&mut self, name: &str) -> &mut Variant {
        self.push_variant(Variant::new(name));
        self.variants.last_mut().unwrap()
    }

    /// Push a variant to the enum.
    ///
    /// # Arguments
    ///
    /// * `item` - The variant to push to the enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::*;
    ///
    /// let mut foo_enum = Enum::new("Foo");
    ///
    /// let foo_enum_first_variant = Variant::new("FirstVariant");
    /// foo_enum.push_variant(foo_enum_first_variant);
    /// ```
    pub fn push_variant(&mut self, item: Variant) -> &mut Self {
        self.variants.push(item);
        self
    }
}

impl FormatCode for Enum {
    /// Formats the enum using the given formatter.
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
    /// let foo_enum = Enum::new("Foo");
    /// foo_enum.fmt_code(&mut fmt);
    /// ```
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("enum", &[], fmt)?;

        fmt.block(|fmt| {
            for variant in &self.variants {
                variant.fmt_code(fmt)?;
            }

            Ok(())
        })
    }
}
