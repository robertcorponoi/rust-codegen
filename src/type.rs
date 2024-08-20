use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Defines a type.
#[derive(Debug, Clone)]
pub struct Type {
    /// The name of the type.
    name: String,
    /// The type's generics.
    generics: Vec<Type>,
}

impl Type {
    /// Return a new type with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Type;
    ///
    /// let foo_type = Type::new("Foo");
    /// ```
    pub fn new(name: &str) -> Self {
        Type {
            name: name.to_string(),
            generics: vec![],
        }
    }

    /// Add a generic to the type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The generic to add to the type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Type;
    ///
    /// let mut foo_type = Type::new("Foo");
    /// foo_type.generic("T");
    /// ```
    pub fn generic(&mut self, ty: impl Into<Type>) -> &mut Self {
        // Make sure that the name doesn't already include generics
        assert!(
            !self.name.contains("<"),
            "type name already includes generics"
        );

        self.generics.push(ty.into());
        self
    }

    /// Formats the struct using the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{Formatter,Type};
    ///
    /// let mut dest = String::new();
    /// let mut fmt = Formatter::new(&mut dest);
    ///
    /// let mut foo_type = Type::new("Foo");
    /// foo_type.fmt(&mut fmt);
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        Type::fmt_slice(&self.generics, fmt)
    }

    /// Formats the type using the given formatter with the given generics.
    ///
    /// # Arguments
    ///
    /// * `generics` - The generics to use.
    /// * `fmt` - The formatter to use.
    fn fmt_slice(generics: &[Type], fmt: &mut Formatter<'_>) -> fmt::Result {
        if !generics.is_empty() {
            write!(fmt, "<")?;

            for (i, ty) in generics.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                ty.fmt(fmt)?;
            }

            write!(fmt, ">")?;
        }

        Ok(())
    }
}

impl<'a> From<&'a str> for Type {
    fn from(src: &'a str) -> Self {
        Type::new(src)
    }
}

impl From<String> for Type {
    fn from(src: String) -> Self {
        Type {
            name: src,
            generics: vec![],
        }
    }
}

impl<'a> From<&'a String> for Type {
    fn from(src: &'a String) -> Self {
        Type::new(src)
    }
}

impl<'a> From<&'a Type> for Type {
    fn from(src: &'a Type) -> Self {
        src.clone()
    }
}
