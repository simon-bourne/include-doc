#![no_std]

//! Include Rust source files as doctests.
//!
//! # Examples
//!
//! ## Using [`source_file!`]
//!
//! We'll write our example in the file `examples/my_example.rs`, and use
//! [`source_file!`] to add the example to our documentation. The contents of
//! `examples/my_example.rs` are:
//!
//! ```
#![doc = include_str!("../examples/my_example.rs")]
//! ```
//! 
//! Using `#[doc = source_file!("examples/my_example.rs")]` will hide imports and
//! include the body of the main function, leaving us with:
//! ```
#![doc = source_file!("examples/my_example.rs")]
//! ```
//! 
//! ## Using [`function_body!`]
//!
//! [`function_body!`] is similar to [`source_file!`], but allows us to specify which
//! function body to use as the doctest. This reduces boilerplate for imports
//! and supporting code, as we can put many examples in one file. We can
//! also specify which parts of the supporting code to include.
//!
//! Usage is:
//! ```
//! include_doc::function_body!(
//!     "example.rs",
//!     example_fn,
//!     [fn_dependency, StructDependency, etc]
//! );
//! ```
//! 
//! In `tests/doc.rs`, we've put 2 examples, `my_first_example` and `my_second_example`.
//! There are 2 different setup functions, but both use `MyStruct`. Here's the contents of
//! `tests/doc.rs`:
//! ```
#![doc = include_str!("../tests/doc.rs")]
//! ```
//! 
//! We want to include only the example code and dependencies for
//! `my_first_example`, so we write
//! `#[doc = function_body!("tests/doc.rs", my_first_example, [MyStruct, setup_first_example])]`,
//! giving us:
//! ```
#![doc = function_body!("tests/doc.rs", my_first_example, [MyStruct, setup_first_example])]
//! ```
//! 
//! For `my_second_example`, we use
//! `#[doc = function_body!("tests/doc.rs", my_second_example, [MyStruct, setup_second_example])]`,
//! giving:
//! ```
#![doc = function_body!("tests/doc.rs", my_second_example, [MyStruct, setup_second_example])]
//! ```

/// Include the function body from a Rust file as a doctest.
///
/// See [module][self] documentation.
pub use include_doc_macro::function_body;
/// Include a Rust file as a doctest.
///
/// See [module][self] documentation.
pub use include_doc_macro::source_file;
