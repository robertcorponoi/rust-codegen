use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::docs::Docs;
use crate::formatter::{fmt_bounds, Formatter};

use crate::r#type::Type;

/// Defines a type definition.
#[derive(Debug, Clone)]
pub struct TypeDef {
    /// The type of the definition.
    pub ty: Type,
    /// The type definition's visibility.
    vis: Option<String>,
    /// The type definition's documentation.
    docs: Option<Docs>,
    /// The types that the type definition should derive.
    derive: Vec<String>,
    /// The lint attribute to supress a warning or error for the type definition.
    allow: Vec<String>,
    /// The type definition's representation.
    repr: Option<String>,
    /// The type definition's bounds.
    bounds: Vec<Bound>,
    /// The type definition's macros.
    macros: Vec<String>,
}

impl TypeDef {
    /// Return a type definition with the provided name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the type definition.
    pub fn new(name: &str) -> Self {
        TypeDef {
            ty: Type::new(name),
            vis: None,
            docs: None,
            derive: vec![],
            allow: vec![],
            repr: None,
            bounds: vec![],
            macros: vec![],
        }
    }

    /// Sets the visibility of the type definition.
    ///
    /// # Arguments
    ///
    /// * `vis` - The visiblity of the type definition.
    pub fn vis(&mut self, vis: &str) {
        self.vis = Some(vis.to_string());
    }

    /// Add a `where` bound to the type definition.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the bound.
    /// * `ty` - The type of the bound.
    pub fn bound(&mut self, name: &str, ty: impl Into<Type>) {
        self.bounds.push(Bound {
            name: name.to_string(),
            bound: vec![ty.into()],
        });
    }

    /// Add a macro to the type definition (e.g. `"#[async_trait]"`)
    ///
    /// # Arguments
    ///
    /// * `macro` - The macro to add.
    pub fn r#macro(&mut self, r#macro: &str) {
        self.macros.push(r#macro.to_string());
    }

    /// Adds documentation to the type definition.
    ///
    /// * `docs` - The docs to add.
    ///
    /// # Examples
    pub fn doc(&mut self, docs: &str) {
        self.docs = Some(Docs::new(docs));
    }

    /// Add a new type that the type definition. should derive.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the derive.
    ///
    /// # Examples
    pub fn derive(&mut self, name: &str) {
        self.derive.push(name.to_string());
    }

    /// Specify lint attribute to supress a warning or error.
    ///
    /// # Arguments
    ///
    /// * `allow` - The lint attribute to apply.
    pub fn allow(&mut self, allow: &str) {
        self.allow.push(allow.to_string());
    }

    /// Specify representation.
    ///
    /// # Arguments
    ///
    /// * `repr` - The representation to specify.
    pub fn repr(&mut self, repr: &str) {
        self.repr = Some(repr.to_string());
    }

    /// Formats the type definition using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    pub fn fmt_head(
        &self,
        keyword: &str,
        parents: &[Type],
        fmt: &mut Formatter<'_>,
    ) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        self.fmt_allow(fmt)?;
        self.fmt_derive(fmt)?;
        self.fmt_repr(fmt)?;
        self.fmt_macros(fmt)?;

        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        write!(fmt, "{} ", keyword)?;
        self.ty.fmt(fmt)?;

        if !parents.is_empty() {
            for (i, ty) in parents.iter().enumerate() {
                if i == 0 {
                    write!(fmt, ": ")?;
                } else {
                    write!(fmt, " + ")?;
                }

                ty.fmt(fmt)?;
            }
        }

        fmt_bounds(&self.bounds, fmt)?;

        Ok(())
    }

    /// Formats the allow using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_allow(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for allow in &self.allow {
            writeln!(fmt, "#[allow({})]", allow)?;
        }

        Ok(())
    }

    /// Formats the representation using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_repr(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref repr) = self.repr {
            writeln!(fmt, "#[repr({})]", repr)?;
        }

        Ok(())
    }

    /// Formats the derive using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_derive(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.derive.is_empty() {
            write!(fmt, "#[derive(")?;

            for (i, name) in self.derive.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                write!(fmt, "{}", name)?;
            }

            writeln!(fmt, ")]")?;
        }

        Ok(())
    }

    /// Formats the macros using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_macros(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.macros.iter() {
            writeln!(fmt, "{}", m)?;
        }
        Ok(())
    }
}
