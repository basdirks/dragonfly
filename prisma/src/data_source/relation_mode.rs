use {
    print::PrintInline,
    std::io,
};

/// How referential integrity is enforced.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RelationMode {
    /// Foreign keys are enforced by the database.
    ForeignKeys,
    /// Foreign keys are emulated in the client.
    #[default]
    Prisma,
}

impl PrintInline for RelationMode {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::ForeignKeys => "foreignKeys",
                Self::Prisma => "prisma",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_foreign_keys() {
        let mode = RelationMode::ForeignKeys;
        let mut f = Vec::new();

        mode.print(&mut f).unwrap();

        assert_eq!(f, b"\"foreignKeys\"");
    }

    #[test]
    fn test_print_prisma() {
        let mode = RelationMode::Prisma;
        let mut f = Vec::new();

        mode.print(&mut f).unwrap();

        assert_eq!(f, b"\"prisma\"");
    }

    #[test]
    fn test_default() {
        assert_eq!(RelationMode::default(), RelationMode::Prisma);
    }
}
