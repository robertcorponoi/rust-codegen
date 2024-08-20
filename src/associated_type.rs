use crate::bound::Bound;
use crate::r#type::Type;

/// Defines an associated type.
#[derive(Debug, Clone)]
pub struct AssociatedType(pub Bound);

impl AssociatedType {
    /// Add a bound to the associated type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The associated type's bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::{AssociatedType, Trait};
    ///
    /// let mut trait_foo = Trait::new("Foo");
    /// let mut trait_bar = Trait::new("Bar");
    ///
    /// trait_bar.associated_type("A").bound("Foo");
    /// ```
    pub fn bound(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.0.bound.push(ty.into());
        self
    }
}
