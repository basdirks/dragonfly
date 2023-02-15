//! # Dragonfly
//!
//! Dragonfly is a toy DSL that explores ways to describe the structure of
//! full-stack web applications. It is not meant to be used in production.
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::if_then_some_else_none
)]
/// The Abstract Syntax Tree of the Dragonfly DSL.
pub mod ast;
/// Source code generators.
pub mod generator;
/// Parser combinators.
pub mod parser;
