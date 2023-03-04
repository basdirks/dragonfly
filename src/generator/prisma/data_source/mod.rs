use {
    crate::generator::printer::{
        comma_separated,
        indent,
    },
    std::fmt::Display,
};
pub use {
    provider::Provider,
    relation_mode::RelationMode,
};

/// Data source providers.
pub mod provider;
/// Relation modes.
pub mod relation_mode;

/// A Prisma data source.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DataSource {
    /// The name of the data source.
    pub name: String,
    /// The provider.
    pub provider: Provider,
    /// The connection URL to the shadow database.
    pub shadow_database_url: Option<String>,
    /// The connection URL for direct connection to the database.
    pub direct_url: Option<String>,
    /// Whether referential integrity is enforced by foreign keys in the
    /// database or emulated in the client.
    pub relation_mode: RelationMode,
}

impl Display for DataSource {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            name,
            provider,
            shadow_database_url,
            direct_url,
            relation_mode,
        } = self;

        let indent = indent::psl(1);
        let mut keys = vec!["provider", "url"];
        let mut values =
            vec![provider.to_string(), format!("\"{}\"", provider.url())];

        if let Some(shadow_database_url) = shadow_database_url {
            keys.push("shadowDatabaseUrl");
            values.push(format!("\"{shadow_database_url}\""));
        }

        if let Some(direct_url) = direct_url {
            keys.push("directUrl");
            values.push(format!("\"{direct_url}\""));
        }

        keys.push("relationMode");
        values.push(relation_mode.to_string());

        if let Provider::PostgreSql { extensions, .. } = provider {
            keys.push("extensions");
            values.push(format!("[{}]", comma_separated(extensions)));
        }

        let max_key_length =
            keys.iter().map(|key| key.len()).max().map_or(0, |max| max);

        writeln!(f, "datasource {name} {{")?;

        for (key, value) in keys.iter().zip(values) {
            writeln!(f, "{indent}{key:<max_key_length$} = {value}")?;
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_relation_mode() {
        assert_eq!(RelationMode::ForeignKeys.to_string(), "\"foreignKeys\"");
        assert_eq!(RelationMode::Prisma.to_string(), "\"prisma\"");
    }

    #[test]
    fn test_display_provider() {
        assert_eq!(
            Provider::PostgreSql {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 5432,
                database: "database".to_owned(),
                schema: "public".to_owned(),
                extensions: vec![],
            }
            .to_string(),
            "\"postgresql\""
        );

        assert_eq!(
            Provider::MySql {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 3306,
                database: "database".to_owned(),
            }
            .to_string(),
            "\"mysql\""
        );

        assert_eq!(
            Provider::Sqlite {
                path: "path".to_owned(),
            }
            .to_string(),
            "\"sqlite\""
        );

        assert_eq!(
            Provider::MongoDb {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 27017,
                database: "database".to_owned(),
            }
            .to_string(),
            "\"mongodb\""
        );

        assert_eq!(
            Provider::SqlServer {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 1433,
                database: "database".to_owned(),
            }
            .to_string(),
            "\"sqlserver\""
        );

        assert_eq!(
            Provider::CockroachDb {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 26257,
                database: "database".to_owned(),
                schema: "public".to_owned(),
            }
            .to_string(),
            "\"cockroachdb\""
        );
    }

    #[test]
    fn test_display_data_source() {
        let data_source = DataSource {
            name: "db".to_owned(),
            provider: Provider::PostgreSql {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 5432,
                database: "database".to_owned(),
                schema: "public".to_owned(),
                extensions: vec![
                    "uuidOssp(map: \"uuid-ossp\")".to_owned(),
                    "pg_trgm".to_owned(),
                    "postgis(version: \"2.1\")".to_owned(),
                ],
            },
            shadow_database_url: Some("shadow_database_url".to_owned()),
            direct_url: Some("direct_url".to_owned()),
            relation_mode: RelationMode::ForeignKeys,
        };

        assert_eq!(
            data_source.to_string(),
            "

datasource db {
  provider          = \"postgresql\"
  url               = \
             \"postgresql://user:password@localhost:5432/database?\
             schema=public\"
  shadowDatabaseUrl = \"shadow_database_url\"
  directUrl         = \"direct_url\"
  relationMode      = \"foreignKeys\"
  extensions        = [uuidOssp(map: \"uuid-ossp\"), pg_trgm, postgis(version: \
             \"2.1\")]
}

"
            .trim()
        );
    }
}
