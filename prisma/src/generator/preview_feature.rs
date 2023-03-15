use {
    print::PrintInline,
    std::{
        fmt::Display,
        io,
    },
};

/// Preview features (updated 4.10.0).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl PrintInline for PreviewFeature {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_client_extensions() {
        let preview_feature = PreviewFeature::ClientExtensions;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"clientExtensions\"");
    }

    #[test]
    fn test_print_deno() {
        let preview_feature = PreviewFeature::Deno;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"deno\"");
    }

    #[test]
    fn test_print_extended_where_unique() {
        let preview_feature = PreviewFeature::ExtendedWhereUnique;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"extendedWhereUnique\"");
    }

    #[test]
    fn test_print_field_reference() {
        let preview_feature = PreviewFeature::FieldReference;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"fieldReference\"");
    }

    #[test]
    fn test_print_filtered_relation_count() {
        let preview_feature = PreviewFeature::FilteredRelationCount;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"filteredRelationCount\"");
    }

    #[test]
    fn test_print_full_text_index() {
        let preview_feature = PreviewFeature::FullTextIndex;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"fullTextIndex\"");
    }

    #[test]
    fn test_print_full_text_search() {
        let preview_feature = PreviewFeature::FullTextSearch;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"fullTextSearch\"");
    }

    #[test]
    fn test_print_metrics() {
        let preview_feature = PreviewFeature::Metrics;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"metrics\"");
    }

    #[test]
    fn test_print_multi_schema() {
        let preview_feature = PreviewFeature::MultiSchema;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"multiSchema\"");
    }

    #[test]
    fn test_print_order_by_nulls() {
        let preview_feature = PreviewFeature::OrderByNulls;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"orderByNulls\"");
    }

    #[test]
    fn test_print_postgresql_extensions() {
        let preview_feature = PreviewFeature::PostgresqlExtensions;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"postgresqlExtensions\"");
    }

    #[test]
    fn test_print_tracing() {
        let preview_feature = PreviewFeature::Tracing;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"tracing\"");
    }

    #[test]
    fn test_print_views() {
        let preview_feature = PreviewFeature::Views;
        let mut f = Vec::new();

        preview_feature.print(&mut f).unwrap();

        assert_eq!(f, b"\"views\"");
    }
}
