//! TypeScript code generation.
//!
//! TypeScript interfaces and enums can be generated from an intermediate
//! representation (`ir::Ir`) of a Dragonfly program.
#![feature(rustdoc_missing_doc_code_examples)]
#![deny(
    clippy::all,
    clippy::cognitive_complexity,
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

pub use {
    expression_with_type_arguments::ExpressionWithTypeArguments,
    import::Import,
    interface::Interface,
    named_specifier::NamedSpecifier,
    r#type::{
        FunctionArgument,
        Keyword,
        Literal,
        ObjectLiteralProperty,
        Type,
    },
    string_enum::StringEnum,
    type_parameter::TypeParameter,
};

/// Expressions with type arguments.
pub mod expression_with_type_arguments;
/// JavaScript import declaration.
pub mod import;
/// TypeScript interface declaration.
pub mod interface;
/// Named specifier.
pub mod named_specifier;
/// TypeScript string enum declaration.
pub mod string_enum;
/// TypeScript types.
pub mod r#type;
/// Type parameters.
pub mod type_parameter;

/// Tab size.
pub const TAB_SIZE: usize = 4;
