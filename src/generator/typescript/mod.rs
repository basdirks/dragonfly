/// Expressions with type arguments.
pub mod expression_with_type_arguments;
/// JavaScript import declaration.
pub mod import;
/// TypeScript interface declaration.
pub mod interface;
/// Literal types.
pub mod literal;
/// Named specifier.
pub mod named_specifier;
/// TypeScript string enum declaration.
pub mod string_enum;
/// TypeScript types.
pub mod r#type;
/// Type parameters.
pub mod type_parameter;
pub use {
    expression_with_type_arguments::ExpressionWithTypeArguments,
    import::Import,
    interface::{
        Interface,
        Property as InterfaceProperty,
    },
    literal::Literal,
    named_specifier::NamedSpecifier,
    r#type::{
        FunctionArgument,
        Keyword,
        ObjectLiteralProperty,
        Type,
    },
    string_enum::{
        StringEnum as Enum,
        Variant as EnumVariant,
    },
    type_parameter::TypeParameter,
};
