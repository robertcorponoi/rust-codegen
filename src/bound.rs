use crate::r#type::Type;

/// Used to add `where` bounds.
#[derive(Debug, Clone)]
pub struct Bound {
    /// The name of the bound.
    pub name: String,
    /// The types of the bound.
    pub bound: Vec<Type>,
}
