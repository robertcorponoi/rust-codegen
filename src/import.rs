/// Defines an import (`use` statement).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Import {
    /// The contents of the import.
    line: String,
    /// Import visibility.
    pub vis: Option<String>,
}

impl Import {
    /// Returns a new import.
    /// 
    /// * `path` - The path to the base import.
    /// * `ty` - The type to import.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Import;
    /// 
    /// let rust_codegen_fn_import = Import::new("rust_codegen", "Function");
    /// ```
    pub fn new(path: &str, ty: &str) -> Self {
        Import {
            line: format!("{}::{}", path, ty),
            vis: None,
        }
    }

    /// Set the import visibility.
    /// 
    /// # Arguments
    /// 
    /// * `vis` - The visibility of the import.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_codegen::Import;
    /// 
    /// let mut rust_codegen_fn_import = Import::new("rust_codegen", "Function");
    /// rust_codegen_fn_import.vis("pub");
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }
}
