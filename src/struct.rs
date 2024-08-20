use std::fmt::{self, Write};

use crate::field::Field;
use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::type_def::TypeDef;

use crate::r#type::Type;

/// Defines a struct.
#[derive(Debug, Clone)]
pub struct Struct {
    type_def: TypeDef,
    /// Struct fields
    fields: Fields,
    /// The attributes for this struct.
    attributes: Vec<String>,
}

impl Struct {
    /// Return a structure definition with the provided name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the struct.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let foo_struct = Struct::new("Foo");
    /// ```
    pub fn new(name: &str) -> Self {
        Struct {
            type_def: TypeDef::new(name),
            fields: Fields::Empty,
            attributes: vec![],
        }
    }

    /// Returns a reference to the type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let foo_struct = Struct::new("Foo");
    /// println!("{:?}", foo_struct.ty());
    /// ```
    pub fn ty(&self) -> &Type {
        &self.type_def.ty
    }

    /// Set the structure visibility.
    /// 
    /// # Arguments
    /// 
    /// * `vis` - The visibility of the struct.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.vis("pub");
    /// ```
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the struct.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the generic.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.generic("T");
    /// ```
    pub fn generic(&mut self, name: &str) -> &mut Self {
        self.type_def.ty.generic(name);
        self
    }

    /// Add a `where` bound to the struct.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the bound.
    /// * `ty` - The type of the bound.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.bound("A", "TraitA");
    /// ```
    pub fn bound(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.type_def.bound(name, ty);
        self
    }

    /// Set the structure documentation.
    /// 
    /// # Arguments
    /// 
    /// * `docs` - The documentation to set for the struct.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.doc("Sample struct documentation.");
    /// ```
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add a new type that the struct should derive.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the type to derive.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.derive("Debug");
    /// ```
    pub fn derive(&mut self, name: &str) -> &mut Self {
        self.type_def.derive(name);
        self
    }

    /// Specify lint attribute to supress a warning or error.
    /// 
    /// # Arguments
    /// 
    /// * `allow` - The lint attribute to add.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.allow("dead_code");
    /// ```
    pub fn allow(&mut self, allow: &str) -> &mut Self {
        self.type_def.allow(allow);
        self
    }

    /// Specify representation.
    /// 
    /// * `repr` - The representation to specify.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.repr("C");
    /// ```
    pub fn repr(&mut self, repr: &str) -> &mut Self {
        self.type_def.repr(repr);
        self
    }

    /// Push a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `push_tuple_field`, but not both.
    /// 
    /// # Arguments
    /// 
    /// * `field` - The named field to push.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::{Field,Struct};
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// let mut bar_field = Field::new("bar", "i32");
    /// 
    /// foo_struct.push_field(bar_field);
    /// ```
    pub fn push_field(&mut self, field: Field) -> &mut Self {
        self.fields.push_named(field);
        self
    }

    /// Add a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `tuple_field`, but not both.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the field.
    /// * `ty` - The type of the field.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.field("bar", "i32");
    /// ```
    pub fn field(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the struct.
    ///
    /// A struct can either set tuple fields with this function or named fields
    /// with `field`, but not both.
    /// 
    /// # Arguments
    /// 
    /// * `ty` - The type of the tuple field to add.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::{Struct,Type};
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// let mut bar_type = Type::new("bar");
    /// 
    /// foo_struct.tuple_field(bar_type);
    /// ```
    pub fn tuple_field(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Adds an attribute to the struct (e.g. `"#[some_attribute]"`)
    /// 
    /// # Arguments
    /// 
    /// * `attribute` - The attribute to add.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Struct;
    /// 
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.attr("some_attribute");
    /// ```
    pub fn attr(&mut self, attribute: &str) -> &mut Self {
        self.attributes.push(attribute.to_string());
        self
    }

    /// Formats the struct using the given formatter.
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
    /// let mut foo_struct = Struct::new("Foo");
    /// foo_struct.fmt(&mut fmt);
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.attributes.iter() {
            writeln!(fmt, "{}", m)?;
        }
        
        self.type_def.fmt_head("struct", &[], fmt)?;
        self.fields.fmt(fmt)?;

        match self.fields {
            Fields::Empty => {
                writeln!(fmt, ";")?;
            }
            Fields::Tuple(..) => {
                writeln!(fmt, ";")?;
            }
            _ => {}
        }

        Ok(())
    }
}
