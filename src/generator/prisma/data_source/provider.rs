use std::fmt::Display;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_postgresql() {
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
    }

    #[test]
    fn test_display_mysql() {
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
    }

    #[test]
    fn test_display_sqlite() {
        assert_eq!(
            Provider::Sqlite {
                path: "path".to_owned(),
            }
            .to_string(),
            "\"sqlite\""
        );
    }

    #[test]
    fn test_display_mongodb() {
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
    }

    #[test]
    fn test_display_sqlserver() {
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
    }

    #[test]
    fn test_display_cockroachdb() {
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
    fn test_url_postgresql() {
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
            .url(),
            "postgresql://user:password@localhost:5432/database?schema=public"
        );
    }

    #[test]
    fn test_url_mysql() {
        assert_eq!(
            Provider::MySql {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 3306,
                database: "database".to_owned(),
            }
            .url(),
            "mysql://user:password@localhost:3306/database"
        );
    }

    #[test]
    fn test_url_sqlite() {
        assert_eq!(
            Provider::Sqlite {
                path: "path".to_owned(),
            }
            .url(),
            "file:./path"
        );
    }

    #[test]
    fn test_url_mongodb() {
        assert_eq!(
            Provider::MongoDb {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 27017,
                database: "database".to_owned(),
            }
            .url(),
            "mongodb+srv://user:password@localhost:27017/database?ssl=true&\
             connectTimeoutMS=5000"
        );
    }

    #[test]
    fn test_url_sqlserver() {
        assert_eq!(
            Provider::SqlServer {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 1433,
                database: "database".to_owned(),
            }
            .url(),
            "sqlserver://localhost:1433;database=database;user=user;\
             password=password;encrypt=true"
        );
    }

    #[test]
    fn test_url_cockroachdb() {
        assert_eq!(
            Provider::CockroachDb {
                user: "user".to_owned(),
                password: "password".to_owned(),
                host: "localhost".to_owned(),
                port: 26257,
                database: "database".to_owned(),
                schema: "public".to_owned(),
            }
            .url(),
            "postgresql://user:password@localhost:26257/database?schema=public"
        );
    }
}
