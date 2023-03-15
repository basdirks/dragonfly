#![feature(rustdoc_missing_doc_code_examples)]
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
    rustdoc::missing_doc_code_examples,
    rustdoc::missing_crate_level_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

//! GraphQL schema generation.

/// Arguments.
pub mod argument;
/// Const arguments.
pub mod const_argument;
/// Const directives.
pub mod const_directive;
/// Directives.
pub mod directive;
/// Enums.
pub mod r#enum;
/// Selection fields.
pub mod field;
/// Fragment spreads.
pub mod fragment_spread;
/// Inline fragments.
pub mod inline_fragment;
/// Queries.
pub mod query;
/// Selections.
pub mod selection;
/// Types.
pub mod r#type;
/// Values.
pub mod value;

/// Tab size.
pub const TAB_SIZE: usize = 2;

pub use {
    argument::Argument,
    const_argument::ConstArgument,
    const_directive::ConstDirective,
    directive::Directive,
    field::Field,
    fragment_spread::FragmentSpread,
    inline_fragment::InlineFragment,
    query::{
        Query,
        Variable as QueryVariable,
    },
    r#enum::Enum,
    r#type::Type,
    selection::Selection,
    value::{
        Const,
        ConstObjectField,
        ObjectField,
        Value,
    },
};
