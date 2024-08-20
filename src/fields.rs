use std::fmt::{self, Write};

use crate::field::Field;
use crate::formatter::Formatter;

use crate::r#type::Type;
use crate::FormatCode;

/// Defines a set of fields.
#[derive(Debug, Clone)]
pub enum Fields {
    Empty,
    /// Tuple types.
    Tuple(Vec<Type>),
    /// Named fields.
    Named(Vec<Field>),
}

impl Fields {
    /// Pushed a named field passed in as a `Field` type.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to push.
    pub fn push_named(&mut self, field: Field) -> &mut Self {
        match *self {
            Fields::Empty => {
                *self = Fields::Named(vec![field]);
            }
            Fields::Named(ref mut fields) => {
                fields.push(field);
            }
            _ => panic!("field list is named"),
        }

        self
    }

    /// Pushes a named field by its name and type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `ty` - The type of the field.
    pub fn named(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.push_named(Field {
            name: name.to_string(),
            ty: ty.into(),
            documentation: Vec::new(),
            annotation: Vec::new(),
        })
    }

    /// Pushes a type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to push.
    pub fn tuple(&mut self, ty: impl Into<Type>) -> &mut Self {
        match *self {
            Fields::Empty => {
                *self = Fields::Tuple(vec![ty.into()]);
            }
            Fields::Tuple(ref mut fields) => {
                fields.push(ty.into());
            }
            _ => panic!("field list is tuple"),
        }

        self
    }
}
impl FormatCode for Fields {
    /// Formats the fields using the provided formatter.
    ///
    /// * `fmt` - The formatter to use.
    fn fmt_code(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Fields::Named(ref fields) => {
                assert!(!fields.is_empty());

                fmt.block(|fmt| {
                    for f in fields {
                        if !f.documentation.is_empty() {
                            for doc in &f.documentation {
                                writeln!(fmt, "/// {}", doc)?;
                            }
                        }
                        if !f.annotation.is_empty() {
                            for ann in &f.annotation {
                                writeln!(fmt, "{}", ann)?;
                            }
                        }
                        write!(fmt, "{}: ", f.name)?;
                        f.ty.fmt_code(fmt)?;
                        writeln!(fmt, ",")?;
                    }

                    Ok(())
                })?;
            }
            Fields::Tuple(ref tys) => {
                assert!(!tys.is_empty());

                write!(fmt, "(")?;

                for (i, ty) in tys.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ")?;
                    }
                    ty.fmt_code(fmt)?;
                }

                write!(fmt, ")")?;
            }
            Fields::Empty => {}
        }

        Ok(())
    }
}
