/// JavaScript import declaration.
pub mod import;
/// TypeScript interface declaration.
pub mod interface;
/// TypeScript string enum declaration.
pub mod string_enum;
/// TypeScript types.
pub mod r#type;

pub use {
    import::{
        Import,
        NamedSpecifier,
    },
    interface::{
        ExpressionWithTypeArguments,
        Interface,
        Property as InterfaceProperty,
        TypeParameter,
    },
    r#type::{
        FunctionArgument,
        Keyword,
        Literal,
        ObjectLiteralProperty,
        Type,
    },
    string_enum::{
        StringEnum as Enum,
        Variant as EnumVariant,
    },
};
