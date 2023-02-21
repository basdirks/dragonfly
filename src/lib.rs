//! # Dragonfly
//!
//! Dragonfly is a toy DSL that explores ways to describe full-stack web
//! applications. You should not use it in production.
#![deny(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::if_then_some_else_none,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    unused_results,
    variant_size_differences
)]
/// The Abstract Syntax Tree.
pub mod ast;
/// The command-line interface.
pub mod cli;
/// Source code generators.
pub mod generator;
/// Parser combinators.
pub mod parser;
