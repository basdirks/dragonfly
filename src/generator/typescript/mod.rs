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

pub use {
    import::Import,
    interface::{
        ExpressionWithTypeArguments,
        Interface,
        Property as InterfaceProperty,
        TypeParameter,
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
};
