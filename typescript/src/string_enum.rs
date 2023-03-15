use {
    ir,
    ord_str_map::OrdStrMap,
    print::Print,
    std::{
        borrow::Cow,
        io,
    },
};

/// A TypeScript enum variant, usually called `member` in TypeScript ASTs. A
/// variant's value may differ from its name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variant<'a> {
    /// The name of the variant. Must be unique within the enum. Usually
    /// pascal case.
    pub name: Cow<'a, str>,
    /// The value of the variant. May differ from the name.
    pub value: Cow<'a, str>,
}

impl<'a, S> From<S> for Variant<'a>
where
    S: Into<Cow<'a, str>> + Clone,
{
    fn from(name: S) -> Self {
        Self {
            name: name.clone().into(),
            value: name.into(),
        }
    }
}

impl Print for Variant<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self { name, value } = self;

        writeln!(f, "{}{name} = \"{value}\",", Self::indent(level))
    }
}

/// A TypeScript enum declaration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StringEnum<'a> {
    /// The name of the enum.
    pub identifier: Cow<'a, str>,
    /// Enum variants, usually called `members` in TypeScript ASTs.
    pub variants: OrdStrMap<Variant<'a>>,
}

impl Print for StringEnum<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier: name,
            variants,
        } = self;

        let indent = Self::indent(level);

        writeln!(f, "{indent}enum {name} {{")?;

        for variant in variants.values() {
            variant.print(level + 1, f)?;
        }

        writeln!(f, "{indent}}}\n")
    }
}

impl<'a> From<ir::Enum<'a>> for StringEnum<'a> {
    fn from(ir_enum: ir::Enum<'a>) -> Self {
        let ir::Enum { name, values } = ir_enum;

        let mut r#enum = Self {
            identifier: name,
            variants: OrdStrMap::new(),
        };

        for value in values.iter() {
            let _: Option<Variant> = r#enum
                .variants
                .insert(value.to_owned(), Variant::from(value.to_owned()));
        }

        r#enum
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        token_set::TokenSet,
    };

    #[test]
    fn test_from_ir_enum() {
        assert_eq!(
            StringEnum::from(ir::Enum {
                name: "CountryName".into(),
                values: TokenSet::from_iter(["France", "Germany", "Italy"]),
            }),
            StringEnum {
                identifier: "CountryName".into(),
                variants: OrdStrMap::from_iter([
                    ("France", Variant::from("France")),
                    ("Germany", Variant::from("Germany")),
                    ("Italy", Variant::from("Italy")),
                ])
            }
        );
    }

    #[test]
    fn test_print() {
        let r#enum = StringEnum {
            identifier: "CountryName".into(),
            variants: OrdStrMap::from_iter([
                ("France", Variant::from("France")),
                ("Germany", Variant::from("Germany")),
                ("Italy", Variant::from("Italy")),
                ("Spain", Variant::from("Spain")),
                ("UnitedKingdom", Variant::from("UnitedKingdom")),
                ("UnitedStates", Variant::from("UnitedStates")),
            ]),
        };

        let mut f = Vec::new();

        r#enum.print(1, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "    enum CountryName {
        France = \"France\",
        Germany = \"Germany\",
        Italy = \"Italy\",
        Spain = \"Spain\",
        UnitedKingdom = \"UnitedKingdom\",
        UnitedStates = \"UnitedStates\",
    }

"
        );
    }

    #[test]
    fn test_print_variant() {
        let variant = Variant {
            name: "France".into(),
            value: "France".into(),
        };

        let mut f = Vec::new();

        variant.print(1, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "    France = \"France\",\n");
    }

    #[test]
    fn test_enum_from_ir_enum() {
        let ir_enum = ir::Enum {
            name: "CountryName".into(),
            values: TokenSet::from_iter([
                "France",
                "Germany",
                "Italy",
                "Spain",
                "UnitedKingdom",
                "UnitedStates",
            ]),
        };

        let expected = StringEnum {
            identifier: "CountryName".into(),
            variants: OrdStrMap::from_iter([
                ("France", Variant::from("France")),
                ("Germany", Variant::from("Germany")),
                ("Italy", Variant::from("Italy")),
                ("Spain", Variant::from("Spain")),
                ("UnitedKingdom", Variant::from("UnitedKingdom")),
                ("UnitedStates", Variant::from("UnitedStates")),
            ]),
        };

        assert_eq!(StringEnum::from(ir_enum.clone()), expected);
        assert_eq!(StringEnum::from(ir_enum), expected);
    }
}
