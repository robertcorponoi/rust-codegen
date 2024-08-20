use std::fmt::{self, Write};

use crate::fields::Fields;
use crate::formatter::Formatter;

use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    /// The name of the variant.
    name: String,
    /// The variant's fields.
    fields: Fields,
}

impl Variant {
    /// Return a new enum variant with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Variant;
    ///
    /// let foo_variant = Variant::new("Foo");
    /// ```
    pub fn new(name: &str) -> Self {
        Variant {
            name: name.to_string(),
            fields: Fields::Empty,
        }
    }

    /// Add a named field to the variant.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `ty` - The type of the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Variant;
    ///
    /// let mut foo_variant = Variant::new("Foo");
    /// foo_variant.named("Bar", "String");
    /// ```
    pub fn named(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type of the tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Variant;
    ///
    /// let mut foo_variant = Variant::new("Foo");
    /// foo_variant.tuple("i32");
    /// ```
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Formats the variant using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Formatter,Variant};
    ///
    /// let mut dest = String::new();
    /// let mut fmt = Formatter::new(&mut dest);
    ///
    /// let mut foo_variant = Variant::new("Foo");
    /// foo_variant.fmt(&mut fmt);
    /// ```
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        writeln!(fmt, ",")?;

        Ok(())
    }
}
