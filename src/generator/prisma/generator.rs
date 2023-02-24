use {
    crate::generator::printer::{
        comma_separated,
        indent,
    },
    std::fmt::Display,
};

/// A generator provider.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Provider {
    /// A file path.
    File(String),
    /// The standard Prisma client.
    #[default]
    PrismaClientJs,
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
                Self::File(path) => path,
                Self::PrismaClientJs => "prisma-client-js",
            }
        )
    }
}

/// A binary target.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BinaryTarget {
    /// `linux-musl`
    ///
    /// OpenSSL: 1.1.x
    /// Distros: Alpine Linux <= 3.16
    AlpineOpenSsl1_1,
    /// `linux-musl-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: Alpine Linux >= 3.17
    AlpineOpenSsl3_0,
    /// `linux-arm64-openssl-1.0.x`
    ///
    /// OpenSSL: 1.0.x
    /// Distros: ARM64-based linux
    Arm64OpenSsl1_0,
    /// `linux-arm64-openssl-1.1.x`
    ///
    /// OpenSSL: 1.1.x
    /// Distros: ARM64-based linux
    Arm64OpenSsl1_1,
    /// `linux-arm64-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: ARM64-based linux
    Arm64OpenSsl3_0,
    /// `darwin`
    ///
    /// macOS Intel x86
    Darwin,
    /// `darwin-arm64`
    ///
    /// macOS ARM64
    DarwinArm64,
    /// `debian-openssl-1.0.x`
    ///
    /// OpenSSL: 1.0.x
    /// Distros:
    /// * Debian 8
    /// * Ubuntu 16.04
    /// * Ubuntu 18.04
    /// * Mint 18
    DebianOpenSsl1_0,
    /// `debian-openssl-1.1.x`
    ///
    /// OpenSSL: 1.1.x
    /// Distros:
    /// * Arch 2019.09.01
    /// * Debian 9
    /// * Debian 10
    /// * Debian 11
    /// * Mint 19
    /// * Ubuntu 18.04
    /// * Ubuntu 19.04
    /// * Ubuntu 20.04
    /// * Ubuntu 21.04
    DebianOpenSsl1_1,
    /// `debian-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: Ubuntu 22.04
    DebianOpenSsl3_0,
    /// `native`
    ///
    /// The native platform.
    #[default]
    Native,
    /// `rhel-openssl-1.0.x`
    ///
    /// OpenSSL: 1.0.x
    /// Distros:
    /// * CentOS 6
    /// * CentOS 7
    RhelOpenSsl1_0,
    /// `rhel-openssl-1.1.x`
    ///
    /// OpenSSL: 1.1.x
    /// Distros:
    /// * Fedora 28
    /// * Fedora 29
    /// * Fedora 30
    RhelOpenSsl1_1,
    /// `rhel-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: Fedora 31
    RhelOpenSsl3_0,
    /// `windows`
    ///
    /// Windows
    Windows,
}

impl Display for BinaryTarget {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::AlpineOpenSsl1_1 => "linux-musl",
                Self::AlpineOpenSsl3_0 => "linux-musl-openssl-3.0.x",
                Self::Arm64OpenSsl1_0 => "linux-arm64-openssl-1.0.x",
                Self::Arm64OpenSsl1_1 => "linux-arm64-openssl-1.1.x",
                Self::Arm64OpenSsl3_0 => "linux-arm64-openssl-3.0.x",
                Self::Darwin => "darwin",
                Self::DarwinArm64 => "darwin-arm64",
                Self::DebianOpenSsl1_0 => "debian-openssl-1.0.x",
                Self::DebianOpenSsl1_1 => "debian-openssl-1.1.x",
                Self::DebianOpenSsl3_0 => "debian-openssl-3.0.x",
                Self::Native => "native",
                Self::RhelOpenSsl1_0 => "rhel-openssl-1.0.x",
                Self::RhelOpenSsl1_1 => "rhel-openssl-1.1.x",
                Self::RhelOpenSsl3_0 => "rhel-openssl-3.0.x",
                Self::Windows => "windows",
            }
        )
    }
}

/// An engine type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EngineType {
    /// A library.
    #[default]
    Library,
    /// A binary.
    Binary,
}

impl Display for EngineType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::Library => "library",
                Self::Binary => "binary",
            }
        )
    }
}

/// Preview features (updated 4.10.0).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

/// A generator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Generator {
    /// The name of the generator.
    pub name: String,
    /// The provider.
    pub provider: Provider,
    /// The location of the generated client. Defaults to
    /// `node_modules/@prisma/client`.
    pub output: Option<String>,
    /// The binary targets of the generator.
    pub binary_targets: Vec<BinaryTarget>,
    /// The preview features to enable. Updated 4.10.0.
    pub preview_features: Vec<PreviewFeature>,
    /// The engine type.
    pub engine_type: Option<EngineType>,
}

impl Display for Generator {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            name,
            provider,
            output,
            binary_targets,
            preview_features,
            engine_type,
        } = self;

        let indent = indent::psl(1);
        let mut lines = vec![format!("{indent}provider = {provider}")];

        if let Some(output) = output {
            lines.push(format!("{indent}output = \"{output}\""));
        }

        if !binary_targets.is_empty() {
            lines.push(format!(
                "{indent}binaryTargets = [{}]",
                comma_separated(binary_targets)
            ));
        }

        if !preview_features.is_empty() {
            lines.push(format!(
                "{indent}previewFeatures = [{}]",
                comma_separated(preview_features)
            ));
        }

        if let Some(engine_type) = engine_type {
            lines.push(format!("{indent}engineType = {engine_type}"));
        }

        write!(f, "generator {name} {{\n{}\n}}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_provider() {
        assert_eq!(
            Provider::File("path/to/file".to_owned()).to_string(),
            "\"path/to/file\""
        );

        assert_eq!(
            Provider::PrismaClientJs.to_string(),
            "\"prisma-client-js\""
        );
    }

    #[test]
    fn test_display_binary_target() {
        assert_eq!(
            BinaryTarget::AlpineOpenSsl1_1.to_string(),
            "\"linux-musl\""
        );

        assert_eq!(
            BinaryTarget::AlpineOpenSsl3_0.to_string(),
            "\"linux-musl-openssl-3.0.x\""
        );

        assert_eq!(
            BinaryTarget::Arm64OpenSsl1_0.to_string(),
            "\"linux-arm64-openssl-1.0.x\""
        );

        assert_eq!(
            BinaryTarget::Arm64OpenSsl1_1.to_string(),
            "\"linux-arm64-openssl-1.1.x\""
        );

        assert_eq!(
            BinaryTarget::Arm64OpenSsl3_0.to_string(),
            "\"linux-arm64-openssl-3.0.x\""
        );

        assert_eq!(BinaryTarget::Darwin.to_string(), "\"darwin\"");

        assert_eq!(BinaryTarget::DarwinArm64.to_string(), "\"darwin-arm64\"");

        assert_eq!(
            BinaryTarget::DebianOpenSsl1_0.to_string(),
            "\"debian-openssl-1.0.x\""
        );

        assert_eq!(
            BinaryTarget::DebianOpenSsl1_1.to_string(),
            "\"debian-openssl-1.1.x\""
        );

        assert_eq!(
            BinaryTarget::DebianOpenSsl3_0.to_string(),
            "\"debian-openssl-3.0.x\""
        );

        assert_eq!(BinaryTarget::Native.to_string(), "\"native\"");

        assert_eq!(
            BinaryTarget::RhelOpenSsl1_0.to_string(),
            "\"rhel-openssl-1.0.x\""
        );

        assert_eq!(
            BinaryTarget::RhelOpenSsl1_1.to_string(),
            "\"rhel-openssl-1.1.x\""
        );

        assert_eq!(
            BinaryTarget::RhelOpenSsl3_0.to_string(),
            "\"rhel-openssl-3.0.x\""
        );

        assert_eq!(BinaryTarget::Windows.to_string(), "\"windows\"");
    }

    #[test]
    fn test_display_engine_type() {
        assert_eq!(EngineType::Binary.to_string(), "\"binary\"");
        assert_eq!(EngineType::Library.to_string(), "\"library\"");
    }

    #[test]
    fn test_display_preview_feature() {
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

    #[test]
    fn test_display_generator() {
        assert_eq!(
            Generator {
                name: "client".to_owned(),
                provider: Provider::PrismaClientJs,
                output: None,
                binary_targets: vec![],
                preview_features: vec![],
                engine_type: None,
            }
            .to_string(),
            "\
generator client {
  provider = \"prisma-client-js\"
}"
        );

        assert_eq!(
            Generator {
                name: "client".to_owned(),
                provider: Provider::PrismaClientJs,
                output: Some("path/to/client".to_owned()),
                binary_targets: vec![
                    BinaryTarget::AlpineOpenSsl3_0,
                    BinaryTarget::Arm64OpenSsl3_0,
                    BinaryTarget::DebianOpenSsl3_0,
                    BinaryTarget::RhelOpenSsl3_0,
                ],
                preview_features: vec![
                    PreviewFeature::ClientExtensions,
                    PreviewFeature::Deno,
                    PreviewFeature::ExtendedWhereUnique,
                    PreviewFeature::FieldReference,
                    PreviewFeature::FilteredRelationCount,
                    PreviewFeature::FullTextIndex,
                    PreviewFeature::FullTextSearch,
                    PreviewFeature::Metrics,
                    PreviewFeature::MultiSchema,
                    PreviewFeature::OrderByNulls,
                    PreviewFeature::PostgresqlExtensions,
                    PreviewFeature::Tracing,
                    PreviewFeature::Views,
                ],
                engine_type: Some(EngineType::Binary),
            }
            .to_string(),
            "\
generator client {
  provider = \"prisma-client-js\"
  output = \"path/to/client\"
  binaryTargets = [\"linux-musl-openssl-3.0.x\", \
             \"linux-arm64-openssl-3.0.x\", \"debian-openssl-3.0.x\", \
             \"rhel-openssl-3.0.x\"]
  previewFeatures = [\"clientExtensions\", \"deno\", \"extendedWhereUnique\", \
             \"fieldReference\", \"filteredRelationCount\", \
             \"fullTextIndex\", \"fullTextSearch\", \"metrics\", \
             \"multiSchema\", \"orderByNulls\", \"postgresqlExtensions\", \
             \"tracing\", \"views\"]
  engineType = \"binary\"
}"
        );
    }
}
