use {
    crate::generator::printer::{
        comma_separated,
        indent,
    },
    std::fmt::Display,
};

/// How referential integrity is enforced.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum RelationMode {
    /// Foreign keys are enforced by the database.
    ForeignKeys,
    /// Foreign keys are emulated in the client.
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

/// A data source provider.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Provider {
    /// A PostgreSQL database.
    PostgreSql {
        /// The username.
        user: String,
        /// The password.
        password: String,
        /// The host.
        host: String,
        /// The port.
        port: u16,
        /// The database name.
        database: String,
        /// The schema name.
        schema: String,
        /// Which PostgreSQL extensions to enable.
        /// TODO: find out how this is supposed to fit in the AST.
        extensions: Vec<String>,
    },
    /// A CockroachDB database.
    CockroachDb {
        /// The username.
        user: String,
        /// The password.
        password: String,
        /// The host.
        host: String,
        /// The port.
        port: u16,
        /// The database name.
        database: String,
        /// The schema name.
        schema: String,
    },
    /// A MySQL database.
    MySql {
        /// The username.
        user: String,
        /// The password.
        password: String,
        /// The host.
        host: String,
        /// The port.
        port: u16,
        /// The database name.
        database: String,
    },
    /// An SQL Server database.
    SqlServer {
        /// The username.
        user: String,
        /// The password.
        password: String,
        /// The host.
        host: String,
        /// The port.
        port: u16,
        /// The database name.
        database: String,
    },
    /// A MongoDB database.
    MongoDb {
        /// The username.
        user: String,
        /// The password.
        password: String,
        /// The host.
        host: String,
        /// The port.
        port: u16,
        /// The database name.
        database: String,
    },
    /// An SQLite database.
    Sqlite {
        /// The path to the database file.
        path: String,
    },
}

impl Display for Provider {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::CockroachDb { .. } => "cockroachdb",
                Self::MongoDb { .. } => "mongodb",
                Self::MySql { .. } => "mysql",
                Self::PostgreSql { .. } => "postgresql",
                Self::Sqlite { .. } => "sqlite",
                Self::SqlServer { .. } => "sqlserver",
            }
        )
    }
}

impl Provider {
    /// Returns the connection URL for the data source.
    ///
    /// # Examples
    ///
    /// ## `PostgreSql`
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::DataSourceProvider;
    ///
    /// let provider = DataSourceProvider::PostgreSql {
    ///     user: "user".to_owned(),
    ///     password: "password".to_owned(),
    ///     host: "localhost".to_owned(),
    ///     port: 5432,
    ///     database: "database".to_owned(),
    ///     schema: "public".to_owned(),
    ///     extensions: vec![],
    /// };
    ///
    /// assert_eq!(
    ///     provider.url(),
    ///     "postgresql://user:password@localhost:5432/database?schema=public"
    /// );
    /// ```
    ///
    /// ## `MySql`
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::DataSourceProvider;
    ///
    /// let provider = DataSourceProvider::MySql {
    ///     user: "user".to_owned(),
    ///     password: "password".to_owned(),
    ///     host: "localhost".to_owned(),
    ///     port: 3306,
    ///     database: "database".to_owned(),
    /// };
    ///
    /// assert_eq!(
    ///     provider.url(),
    ///     "mysql://user:password@localhost:3306/database"
    /// );
    /// ```
    ///
    /// ## `Sqlite`
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::DataSourceProvider;
    ///
    /// let provider = DataSourceProvider::Sqlite {
    ///     path: "path/to/database.db".to_owned(),
    /// };
    ///
    /// assert_eq!(provider.url(), "file:./path/to/database.db");
    /// ```
    ///
    /// ## `MongoDb`
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::DataSourceProvider;
    ///
    /// let provider = DataSourceProvider::MongoDb {
    ///     user: "user".to_owned(),
    ///     password: "password".to_owned(),
    ///     host: "localhost".to_owned(),
    ///     port: 27017,
    ///     database: "database".to_owned(),
    /// };
    ///
    /// assert_eq!(
    ///     provider.url(),
    ///     "mongodb+srv://user:password@localhost:27017/database?ssl=true&\
    ///      connectTimeoutMS=5000"
    /// );
    /// ```
    ///
    /// ## `SqlServer`, following JDBC URL format.
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::DataSourceProvider;
    ///
    /// let provider = DataSourceProvider::SqlServer {
    ///     user: "user".to_owned(),
    ///     password: "password".to_owned(),
    ///     host: "localhost".to_owned(),
    ///     port: 1433,
    ///     database: "database".to_owned(),
    /// };
    ///
    /// assert_eq!(
    ///     provider.url(),
    ///     "sqlserver://localhost:1433;database=database;user=user;\
    ///      password=password;encrypt=true"
    /// );
    /// ```
    ///
    /// ## `CockroachDb`
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::DataSourceProvider;
    ///
    /// let provider = DataSourceProvider::CockroachDb {
    ///     user: "user".to_owned(),
    ///     password: "password".to_owned(),
    ///     host: "localhost".to_owned(),
    ///     port: 26257,
    ///     database: "database".to_owned(),
    ///     schema: "public".to_owned(),
    /// };
    ///
    /// assert_eq!(
    ///     provider.url(),
    ///     "postgresql://user:password@localhost:26257/database?schema=public"
    /// );
    /// ```
    #[must_use]
    pub fn url(&self) -> String {
        match self {
            Self::PostgreSql {
                user,
                password,
                host,
                port,
                database,
                schema,
                ..
            }
            | Self::CockroachDb {
                user,
                password,
                host,
                port,
                database,
                schema,
            } => {
                format!(
                    "postgresql://{user}:{password}@{host}:{port}/{database}?\
                     schema={schema}"
                )
            }
            Self::MySql {
                user,
                password,
                host,
                port,
                database,
            } => {
                format!("mysql://{user}:{password}@{host}:{port}/{database}")
            }
            Self::Sqlite { path } => format!("file:./{path}"),
            Self::MongoDb {
                user,
                password,
                host,
                port,
                database,
            } => {
                format!(
                    "mongodb+srv://{user}:{password}@{host}:{port}/{database}?\
                     ssl=true&connectTimeoutMS=5000"
                )
            }
            Self::SqlServer {
                user,
                password,
                host,
                port,
                database,
            } => {
                format!(
                    "sqlserver://{host}:{port};database={database};\
                     user={user};password={password};encrypt=true"
                )
            }
        }
    }
}

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
    pub relation_mode: Option<RelationMode>,
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

        let mut lines = vec![
            format!("{indent}provider = {provider}"),
            format!("{indent}url = \"{}\"", provider.url()),
        ];

        if let Some(shadow_database_url) = shadow_database_url {
            lines.push(format!(
                "{indent}shadowDatabaseUrl = \"{shadow_database_url}\""
            ));
        }

        if let Some(direct_url) = direct_url {
            lines.push(format!("{indent}directUrl = \"{direct_url}\""));
        }

        if let Some(relation_mode) = relation_mode {
            lines.push(format!("{indent}relationMode = {relation_mode}"));
        }

        if let Provider::PostgreSql { extensions, .. } = provider {
            lines.push(format!(
                "{indent}extensions = [{}]",
                comma_separated(extensions)
            ));
        }

        write!(f, "datasource {name} {{\n{}\n}}", lines.join("\n"))
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
            relation_mode: Some(RelationMode::ForeignKeys),
        };

        assert_eq!(
            data_source.to_string(),
            "\
datasource db {
  provider = \"postgresql\"
  url = \"postgresql://user:password@localhost:5432/database?schema=public\"
  shadowDatabaseUrl = \"shadow_database_url\"
  directUrl = \"direct_url\"
  relationMode = \"foreignKeys\"
  extensions = [uuidOssp(map: \"uuid-ossp\"), pg_trgm, postgis(version: \
             \"2.1\")]
}"
        );
    }
}
