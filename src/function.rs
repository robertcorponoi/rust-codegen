use std::fmt::{self, Write};

use crate::block::Block;
use crate::body::Body;
use crate::bound::Bound;
use crate::docs::Docs;
use crate::field::Field;
use crate::formatter::Formatter;
use crate::formatter::{fmt_bounds, fmt_generics};

use crate::r#type::Type;

/// Defines a function.
#[derive(Debug, Clone)]
pub struct Function {
    /// The name of the function.
    name: String,
    /// The function's documentation.
    docs: Option<Docs>,
    /// A lint attribute used to suppress a warning or error.
    allow: Option<String>,
    /// Function visibility.
    vis: Option<String>,
    /// Function generics.
    generics: Vec<String>,
    /// If the function takes `&self` or `&mut self`.
    arg_self: Option<String>,
    /// Function arguments.
    args: Vec<Field>,
    /// Return type.
    ret: Option<Type>,
    /// Where bounds.
    bounds: Vec<Bound>,
    /// Body contents.
    pub body: Option<Vec<Body>>,
    /// Function attributes, e.g., `#[no_mangle]`.
    attributes: Vec<String>,
    /// Function `extern` ABI.
    extern_abi: Option<String>,
    /// Whether or not this function is `async` or not.
    r#async: bool,
}

impl Function {
    /// Return a new function definition.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let foo_fn = Function::new("foo_fn");
    /// ```
    pub fn new(name: &str) -> Self {
        Function {
            name: name.to_string(),
            docs: None,
            allow: None,
            vis: None,
            generics: vec![],
            arg_self: None,
            args: vec![],
            ret: None,
            bounds: vec![],
            body: Some(vec![]),
            attributes: vec![],
            extern_abi: None,
            r#async: false,
        }
    }

    /// Set the function documentation.
    ///
    /// # Arguments
    ///
    /// * `docs` - The docs to set for the function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.doc("Sample Foo function documentation.");
    /// ```
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.docs = Some(Docs::new(docs));
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
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.allow("dead_code");
    /// ```
    pub fn allow(&mut self, allow: &str) -> &mut Self {
        self.allow = Some(allow.to_string());
        self
    }

    /// Set the function visibility.
    ///
    /// # Arguments
    ///
    /// * `vis` - The visiblity to set for the function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.vis("pub");
    /// ```
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }

    /// Set whether this function is async or not.
    ///
    /// # Arguments
    ///
    /// * `async` - Indicates whether this function is async or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.set_async(true);
    /// ```
    pub fn set_async(&mut self, r#async: bool) -> &mut Self {
        self.r#async = r#async;
        self
    }

    /// Add a generic to the function.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the generic to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.generic("T");
    /// ```
    pub fn generic(&mut self, name: &str) -> &mut Self {
        self.generics.push(name.to_string());
        self
    }

    /// Add `self` as a function argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.arg_self();
    /// ```
    pub fn arg_self(&mut self) -> &mut Self {
        self.arg_self = Some("self".to_string());
        self
    }

    /// Add `&self` as a function argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.arg_ref_self();
    /// ```
    pub fn arg_ref_self(&mut self) -> &mut Self {
        self.arg_self = Some("&self".to_string());
        self
    }

    /// Add `&mut self` as a function argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.arg_mut_self();
    /// ```
    pub fn arg_mut_self(&mut self) -> &mut Self {
        self.arg_self = Some("&mut self".to_string());
        self
    }

    /// Add a function argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `ty` - The type of the argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.arg("name", "&str");
    /// ```
    pub fn arg(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.args.push(Field {
            name: name.to_string(),
            ty: ty.into(),
            // While a `Field` is used here, both `documentation`
            // and `annotation` does not make sense for function arguments.
            // Simply use empty strings.
            documentation: Vec::new(),
            annotation: Vec::new(),
        });

        self
    }

    /// Set the function return type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The return type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.ret("String");
    /// ```
    pub fn ret(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.ret = Some(ty.into());
        self
    }

    /// Add a `where` bound to the function.
    ///
    /// # Arguments
    ///
    /// * `name ` - The name of the bound.
    /// * `ty` - The type of the bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.bound("A", "TraitA");
    /// ```
    pub fn bound(&mut self, name: &str, ty: impl Into<Type>) -> &mut Self {
        self.bounds.push(Bound {
            name: name.to_string(),
            bound: vec![ty.into()],
        });
        self
    }

    /// Push a line to the function implementation.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to add to the function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.line("println!(\"Hello, world!\")");
    /// ```
    pub fn line(&mut self, line: impl ToString) -> &mut Self {
        self.body
            .get_or_insert(vec![])
            .push(Body::String(line.to_string()));

        self
    }

    /// Add an attribute to the function.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.attr("test");
    /// ```
    pub fn attr(&mut self, attribute: &str) -> &mut Self {
        self.attributes.push(attribute.to_string());
        self
    }

    /// Specify an `extern` ABI for the function.
    ///
    /// # Arguments
    ///
    /// * `abi` - The extern ABI to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::Function;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.extern_abi("C");
    /// ```
    pub fn extern_abi(&mut self, abi: &str) -> &mut Self {
        self.extern_abi.replace(abi.to_string());
        self
    }

    /// Push a block to the function implementation.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to push to the function.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_codegen::*;
    ///
    /// let mut foo_fn = Function::new("foo_fn");
    ///
    /// let mut block = Block::new("");
    /// block.line("println!(\"Hello, world!\");");
    ///
    /// foo_fn.push_block(block);
    /// ```
    pub fn push_block(&mut self, block: Block) -> &mut Self {
        self.body.get_or_insert(vec![]).push(Body::Block(block));

        self
    }

    /// Formats the function using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `is_trait` - Indicates whether it is a trait or not.
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
    /// let mut foo_fn = Function::new("foo_fn");
    /// foo_fn.fmt(false, &mut fmt);
    /// ```
    pub fn fmt(&self, is_trait: bool, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        if let Some(ref allow) = self.allow {
            writeln!(fmt, "#[allow({})]", allow)?;
        }

        for attr in self.attributes.iter() {
            writeln!(fmt, "#[{}]", attr)?;
        }

        if is_trait {
            assert!(
                self.vis.is_none(),
                "trait fns do not have visibility modifiers"
            );
        }

        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        if let Some(ref extern_abi) = self.extern_abi {
            write!(fmt, "extern \"{extern_abi}\" ", extern_abi = extern_abi)?;
        }

        if self.r#async {
            write!(fmt, "async ")?;
        }

        write!(fmt, "fn {}", self.name)?;
        fmt_generics(&self.generics, fmt)?;

        write!(fmt, "(")?;

        if let Some(ref s) = self.arg_self {
            write!(fmt, "{}", s)?;
        }

        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 || self.arg_self.is_some() {
                write!(fmt, ", ")?;
            }

            write!(fmt, "{}: ", arg.name)?;
            arg.ty.fmt(fmt)?;
        }

        write!(fmt, ")")?;

        if let Some(ref ret) = self.ret {
            write!(fmt, " -> ")?;
            ret.fmt(fmt)?;
        }

        fmt_bounds(&self.bounds, fmt)?;

        match self.body {
            Some(ref body) => fmt.block(|fmt| {
                for b in body {
                    b.fmt(fmt)?;
                }

                Ok(())
            }),
            None => {
                if !is_trait {
                    panic!("impl blocks must define fn bodies");
                }

                writeln!(fmt, ";")
            }
        }
    }
}
