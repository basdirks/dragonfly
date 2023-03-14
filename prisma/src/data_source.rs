use {
    printer::{
        Print,
        PrintInline,
    },
    std::{
        borrow::Cow,
        io::{
            self,
            Write,
        },
    },
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
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataSource<'a> {
    /// The name of the data source.
    pub name: Cow<'a, str>,
    /// The provider.
    pub provider: Provider<'a>,
    /// The connection URL to the shadow database.
    pub shadow_database_url: Option<Cow<'a, str>>,
    /// The connection URL for direct connection to the database.
    pub direct_url: Option<Cow<'a, str>>,
    /// Whether referential integrity is enforced by foreign keys in the
    /// database or emulated in the client.
    pub relation_mode: RelationMode,
}

impl Print for DataSource<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn Write,
    ) -> io::Result<()> {
        let Self {
            name,
            provider,
            shadow_database_url,
            direct_url,
            relation_mode,
        } = self;

        let indent_outer = Self::indent(level);
        let indent_inner = Self::indent(level + 1);
        let url = provider.url();
        let provider_value = provider.to_string();
        let mut keys = vec!["provider", "url"];
        let mut values = vec![provider_value, format!("\"{url}\"")];

        if let Some(shadow_database_url) = shadow_database_url {
            keys.push("shadowDatabaseUrl");
            values.push(format!("\"{shadow_database_url}\""));
        }

        if let Some(direct_url) = direct_url {
            keys.push("directUrl");
            values.push(format!("\"{direct_url}\""));
        }

        keys.push("relationMode");
        values.push({
            let mut f = Vec::new();
            relation_mode.print(&mut f)?;
            String::from_utf8_lossy(&f).to_string()
        });

        if let Provider::PostgreSql { extensions, .. } = provider {
            keys.push("extensions");
            values.push(format!("[{}]", extensions.join(", ")));
        }

        let max_key_length = keys.iter().map(|s| s.len()).max().unwrap_or(0);

        writeln!(f, "{indent_outer}datasource {name} {{")?;

        for (key, value) in keys.iter().zip(values) {
            writeln!(f, "{indent_inner}{key:<max_key_length$} = {value}")?;
        }

        writeln!(f, "{indent_outer}}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let data_source = DataSource {
            name: "db".into(),
            provider: Provider::PostgreSql {
                user: "user".into(),
                password: "password".into(),
                host: "localhost".into(),
                port: 5432,
                database: "database".into(),
                schema: "public".into(),
                extensions: vec![
                    "uuidOssp(map: \"uuid-ossp\")".into(),
                    "pg_trgm".into(),
                    "postgis(version: \"2.1\")".into(),
                ],
            },
            shadow_database_url: Some("shadow_database_url".into()),
            direct_url: Some("direct_url".into()),
            relation_mode: RelationMode::ForeignKeys,
        };

        let mut f = Vec::new();

        data_source.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "datasource db {
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
        );
    }
}
