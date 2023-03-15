use {
    print::PrintInline,
    std::{
        borrow::Cow,
        fmt::Display,
        io,
    },
};

/// A data source provider.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Provider<'a> {
    /// A PostgreSQL database.
    PostgreSql {
        /// The username.
        user: Cow<'a, str>,
        /// The password.
        password: Cow<'a, str>,
        /// The host.
        host: Cow<'a, str>,
        /// The port.
        port: u16,
        /// The database name.
        database: Cow<'a, str>,
        /// The schema name.
        schema: Cow<'a, str>,
        /// Which PostgreSQL extensions to enable.
        extensions: Vec<Cow<'a, str>>,
    },
    /// A CockroachDB database.
    CockroachDb {
        /// The username.
        user: Cow<'a, str>,
        /// The password.
        password: Cow<'a, str>,
        /// The host.
        host: Cow<'a, str>,
        /// The port.
        port: u16,
        /// The database name.
        database: Cow<'a, str>,
        /// The schema name.
        schema: Cow<'a, str>,
    },
    /// A MySQL database.
    MySql {
        /// The username.
        user: Cow<'a, str>,
        /// The password.
        password: Cow<'a, str>,
        /// The host.
        host: Cow<'a, str>,
        /// The port.
        port: u16,
        /// The database name.
        database: Cow<'a, str>,
    },
    /// An SQL Server database.
    SqlServer {
        /// The username.
        user: Cow<'a, str>,
        /// The password.
        password: Cow<'a, str>,
        /// The host.
        host: Cow<'a, str>,
        /// The port.
        port: u16,
        /// The database name.
        database: Cow<'a, str>,
    },
    /// A MongoDB database.
    MongoDb {
        /// The username.
        user: Cow<'a, str>,
        /// The password.
        password: Cow<'a, str>,
        /// The host.
        host: Cow<'a, str>,
        /// The port.
        port: u16,
        /// The database name.
        database: Cow<'a, str>,
    },
    /// An SQLite database.
    Sqlite {
        /// The path to the database file.
        path: Cow<'a, str>,
    },
}

impl<'a> Provider<'a> {
    /// Returns the connection URL for the data source.
    #[must_use]
    pub fn url(&self) -> Cow<'a, str> {
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
                .into()
            }
            Self::MySql {
                user,
                password,
                host,
                port,
                database,
            } => {
                format!("mysql://{user}:{password}@{host}:{port}/{database}")
                    .into()
            }
            Self::Sqlite { path } => format!("file:./{path}").into(),
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
                .into()
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
                .into()
            }
        }
    }
}

impl Display for Provider<'_> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::PostgreSql { .. } => "postgresql",
                Self::CockroachDb { .. } => "cockroachdb",
                Self::MySql { .. } => "mysql",
                Self::Sqlite { .. } => "sqlite",
                Self::MongoDb { .. } => "mongodb",
                Self::SqlServer { .. } => "sqlserver",
            }
        )
    }
}

impl PrintInline for Provider<'_> {
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
    fn test_print_postgresql() {
        let provider = Provider::PostgreSql {
            user: "user".into(),
            password: "password".into(),
            host: "localhost".into(),
            port: 5432,
            database: "database".into(),
            schema: "public".into(),
            extensions: Vec::new(),
        };

        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"postgresql\"");
    }

    #[test]
    fn test_print_mysql() {
        let provider = Provider::MySql {
            user: "user".into(),
            password: "password".into(),
            host: "localhost".into(),
            port: 3306,
            database: "database".into(),
        };

        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"mysql\"");
    }

    #[test]
    fn test_print_sqlite() {
        let provider = Provider::Sqlite {
            path: "path".into(),
        };

        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"sqlite\"");
    }

    #[test]
    fn test_print_mongodb() {
        let provider = Provider::MongoDb {
            user: "user".into(),
            password: "password".into(),
            host: "localhost".into(),
            port: 27017,
            database: "database".into(),
        };

        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"mongodb\"");
    }

    #[test]
    fn test_print_sqlserver() {
        let provider = Provider::SqlServer {
            user: "user".into(),
            password: "password".into(),
            host: "localhost".into(),
            port: 1433,
            database: "database".into(),
        };

        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"sqlserver\"");
    }

    #[test]
    fn test_print_cockroachdb() {
        let provider = Provider::CockroachDb {
            user: "user".into(),
            password: "password".into(),
            host: "localhost".into(),
            port: 26257,
            database: "database".into(),
            schema: "public".into(),
        };

        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"cockroachdb\"");
    }

    #[test]
    fn test_url_postgresql() {
        assert_eq!(
            Provider::PostgreSql {
                user: "user".into(),
                password: "password".into(),
                host: "localhost".into(),
                port: 5432,
                database: "database".into(),
                schema: "public".into(),
                extensions: Vec::new(),
            }
            .url(),
            "postgresql://user:password@localhost:5432/database?schema=public"
        );
    }

    #[test]
    fn test_url_mysql() {
        assert_eq!(
            Provider::MySql {
                user: "user".into(),
                password: "password".into(),
                host: "localhost".into(),
                port: 3306,
                database: "database".into(),
            }
            .url(),
            "mysql://user:password@localhost:3306/database"
        );
    }

    #[test]
    fn test_url_sqlite() {
        assert_eq!(
            Provider::Sqlite {
                path: "path".into(),
            }
            .url(),
            "file:./path"
        );
    }

    #[test]
    fn test_url_mongodb() {
        assert_eq!(
            Provider::MongoDb {
                user: "user".into(),
                password: "password".into(),
                host: "localhost".into(),
                port: 27017,
                database: "database".into(),
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
                user: "user".into(),
                password: "password".into(),
                host: "localhost".into(),
                port: 1433,
                database: "database".into(),
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
                user: "user".into(),
                password: "password".into(),
                host: "localhost".into(),
                port: 26257,
                database: "database".into(),
                schema: "public".into(),
            }
            .url(),
            "postgresql://user:password@localhost:26257/database?schema=public"
        );
    }
}
