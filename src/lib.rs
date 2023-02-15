//! # Dragonfly
//!
//! Dragonfly is a toy DSL that explores ways to describe full-stack web
//! applications. You should not use it in production.
#![deny(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::if_then_some_else_none
)]
/// The Abstract Syntax Tree.
pub mod ast;
/// Source code generators.
pub mod generator;
/// Parser combinators.
pub mod parser;
