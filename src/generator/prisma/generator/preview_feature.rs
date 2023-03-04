use std::fmt::Display;

/// Preview features (updated 4.10.0).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PreviewFeature {
    /// `clientExtensions`, since 3.6.0.
    ClientExtensions,
    /// `deno`, since 4.5.0.
    Deno,
    /// `extendedWhereUnique`, since 4.5.0.
    ExtendedWhereUnique,
    /// `fieldReference`, since 4.3.0.
    FieldReference,
    /// `filteredRelationCount`, since 4.3.0.
    FilteredRelationCount,
    /// `fullTextIndex`, since 3.6.0.
    FullTextIndex,
    /// `fullTextSearch`, since 2.30.0.
    FullTextSearch,
    /// `metrics`, since 3.15.0.
    Metrics,
    /// `multiSchema`, since 4.3.0.
    MultiSchema,
    /// `orderByNulls`, since 4.1.0.
    OrderByNulls,
    /// `postgresqlExtensions`, since 4.5.0.
    PostgresqlExtensions,
    /// `tracing`, since 4.2.0.
    Tracing,
    /// `views`, since 4.9.0.
    Views,
}

impl Display for PreviewFeature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::ClientExtensions => "clientExtensions",
                Self::Deno => "deno",
                Self::ExtendedWhereUnique => "extendedWhereUnique",
                Self::FieldReference => "fieldReference",
                Self::FilteredRelationCount => "filteredRelationCount",
                Self::FullTextIndex => "fullTextIndex",
                Self::FullTextSearch => "fullTextSearch",
                Self::Metrics => "metrics",
                Self::MultiSchema => "multiSchema",
                Self::OrderByNulls => "orderByNulls",
                Self::PostgresqlExtensions => "postgresqlExtensions",
                Self::Tracing => "tracing",
                Self::Views => "views",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            PreviewFeature::ClientExtensions.to_string(),
            "\"clientExtensions\""
        );

        assert_eq!(PreviewFeature::Deno.to_string(), "\"deno\"");

        assert_eq!(
            PreviewFeature::ExtendedWhereUnique.to_string(),
            "\"extendedWhereUnique\""
        );

        assert_eq!(
            PreviewFeature::FieldReference.to_string(),
            "\"fieldReference\""
        );

        assert_eq!(
            PreviewFeature::FilteredRelationCount.to_string(),
            "\"filteredRelationCount\""
        );

        assert_eq!(
            PreviewFeature::FullTextIndex.to_string(),
            "\"fullTextIndex\""
        );

        assert_eq!(
            PreviewFeature::FullTextSearch.to_string(),
            "\"fullTextSearch\""
        );

        assert_eq!(PreviewFeature::Metrics.to_string(), "\"metrics\"");

        assert_eq!(PreviewFeature::MultiSchema.to_string(), "\"multiSchema\"");

        assert_eq!(
            PreviewFeature::OrderByNulls.to_string(),
            "\"orderByNulls\""
        );

        assert_eq!(
            PreviewFeature::PostgresqlExtensions.to_string(),
            "\"postgresqlExtensions\""
        );

        assert_eq!(PreviewFeature::Tracing.to_string(), "\"tracing\"");
        assert_eq!(PreviewFeature::Views.to_string(), "\"views\"");
    }
}
