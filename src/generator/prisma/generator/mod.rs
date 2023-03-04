use {
    crate::generator::printer::{
        comma_separated,
        indent,
    },
    std::fmt::Display,
};
pub use {
    binary_target::BinaryTarget,
    engine_type::EngineType,
    preview_feature::PreviewFeature,
    provider::Provider,
};

/// Binary targets.
pub mod binary_target;
/// Engine types.
pub mod engine_type;
/// Preview features.
pub mod preview_feature;
/// Generator providers.
pub mod provider;

/// A generator.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
        let mut keys = vec!["provider"];
        let mut values = vec![provider.to_string()];

        if let Some(output) = output {
            keys.push("output");
            values.push(format!("\"{output}\""));
        }

        if !binary_targets.is_empty() {
            keys.push("binaryTargets");
            values.push(format!("[{}]", comma_separated(binary_targets)));
        }

        if !preview_features.is_empty() {
            keys.push("previewFeatures");
            values.push(format!("[{}]", comma_separated(preview_features)));
        }

        if let Some(engine_type) = engine_type {
            keys.push("engineType");
            values.push(engine_type.to_string());
        }

        let max_key_length =
            keys.iter().map(|s| s.len()).max().map_or(0, |n| n);

        writeln!(f, "generator {name} {{")?;

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
    fn test_display() {
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
            "

generator client {
  provider = \"prisma-client-js\"
}

"
            .trim()
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
            "

generator client {
  provider        = \"prisma-client-js\"
  output          = \"path/to/client\"
  binaryTargets   = [\"linux-musl-openssl-3.0.x\", \
             \"linux-arm64-openssl-3.0.x\", \"debian-openssl-3.0.x\", \
             \"rhel-openssl-3.0.x\"]
  previewFeatures = [\"clientExtensions\", \"deno\", \"extendedWhereUnique\", \
             \"fieldReference\", \"filteredRelationCount\", \
             \"fullTextIndex\", \"fullTextSearch\", \"metrics\", \
             \"multiSchema\", \"orderByNulls\", \"postgresqlExtensions\", \
             \"tracing\", \"views\"]
  engineType      = \"binary\"
}

"
            .trim()
        );
    }
}
