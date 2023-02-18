/// Common printer functions.
pub mod common;
/// Indentation level.
pub mod indent;
/// The print trait.
pub mod print;

pub use {
    common::{
        comma_separated,
        newline_separated,
        separated,
        space_separated,
    },
    print::Print,
};
