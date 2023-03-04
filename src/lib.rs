//! # Dragonfly
//!
//! Dragonfly is a toy DSL that explores ways to describe full-stack web
//! applications. You should not use it in production.
#![deny(
    clippy::all,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::missing_docs_in_private_items,
    clippy::mixed_read_write_in_expression,
    clippy::nursery,
    clippy::pedantic,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
/// The Abstract Syntax Tree.
pub mod ast;
/// The command-line interface.
pub mod cli;
/// Source code generators.
pub mod generator;
/// Intermediate representation.
pub mod ir;
/// Parser combinators.
pub mod parser;
