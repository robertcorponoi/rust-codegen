use crate::r#type::Type;

/// Defines a struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,

    /// Field type
    pub ty: Type,

    /// Field documentation
    pub documentation: Vec<String>,

    /// Field annotation
    pub annotation: Vec<String>,
}

impl Field {
    /// Return a field definition with the provided name and type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `ty` - The type of the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Field;
    ///
    /// let count_field = Field::new("count", "i32");
    /// ```
    pub fn new(name: &str, ty: impl Into<Type>) -> Self {
        Field {
            name: name.into(),
            ty: ty.into(),
            documentation: Vec::new(),
            annotation: Vec::new(),
        }
    }

    /// Set the field's documentation.
    ///
    /// # Arguments
    ///
    /// * `documentation` - The documentation to set for the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Field;
    ///
    /// let count_field = Field::new("count", "i32").doc(Vec::from(["The number of Foos"]));
    pub fn doc(&mut self, documentation: Vec<&str>) -> &mut Self {
        self.documentation = documentation.iter().map(|doc| doc.to_string()).collect();
        self
    }

    /// Set the field's annotation.
    ///
    /// # Arguments
    ///
    /// * `annotation` - The annotation to set for the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Field;
    ///
    /// let count_field = Field::new("count", "i32").annotation(Vec::from(["serde(rename = \"name\")"]));
    pub fn annotation(&mut self, annotation: Vec<&str>) -> &mut Self {
        self.annotation = annotation.iter().map(|ann| ann.to_string()).collect();
        self
    }
}
