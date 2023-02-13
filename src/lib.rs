#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::if_then_some_else_none
)]
pub mod ast;
pub mod generator;
pub mod parser;
