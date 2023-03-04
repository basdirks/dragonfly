use std::fmt::Display;

/// How referential integrity is enforced.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RelationMode {
    /// Foreign keys are enforced by the database.
    ForeignKeys,
    /// Foreign keys are emulated in the client.
    #[default]
    Prisma,
}

impl Display for RelationMode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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
    fn test_default() {
        assert_eq!(RelationMode::default(), RelationMode::Prisma);
    }

    #[test]
    fn test_display() {
        assert_eq!(RelationMode::ForeignKeys.to_string(), "\"foreignKeys\"");
        assert_eq!(RelationMode::Prisma.to_string(), "\"prisma\"");
    }
}
