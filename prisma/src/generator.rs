pub use {
    binary_target::BinaryTarget,
    engine_type::EngineType,
    preview_feature::PreviewFeature,
    provider::Provider,
};
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

/// Binary targets.
pub mod binary_target;
/// Engine types.
pub mod engine_type;
/// Preview features.
pub mod preview_feature;
/// Generator providers.
pub mod provider;

/// A generator.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Generator<'a> {
    /// The name of the generator.
    pub name: Cow<'a, str>,
    /// The provider.
    pub provider: Provider<'a>,
    /// The location of the generated client.
    pub output: Option<Cow<'a, str>>,
    /// The binary targets of the generator.
    pub binary_targets: Vec<BinaryTarget>,
    /// The preview features to enable. Updated 4.10.0.
    pub preview_features: Vec<PreviewFeature>,
    /// The engine type.
    pub engine_type: Option<EngineType>,
}

impl Print for Generator<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn Write,
    ) -> io::Result<()> {
        let Self {
            name,
            provider,
            output,
            binary_targets,
            preview_features,
            engine_type,
        } = self;

        let indent_outer = Self::indent(level);
        let indent_inner = Self::indent(level + 1);
        let mut keys = vec!["provider"];
        let mut values = vec![provider.to_string()];

        if let Some(output) = output {
            keys.push("output");
            values.push(format!("\"{output}\""));
        }

        if !binary_targets.is_empty() {
            keys.push("binaryTargets");
            let mut g: Vec<u8> = Vec::new();

            write!(g, "[")?;
            PrintInline::intercalate(binary_targets.clone(), &mut g, ", ")?;
            write!(g, "]")?;

            values.push(String::from_utf8_lossy(&g).to_string());
        }

        if !preview_features.is_empty() {
            keys.push("previewFeatures");
            let mut g: Vec<u8> = Vec::new();

            write!(g, "[")?;
            PrintInline::intercalate(preview_features.clone(), &mut g, ", ")?;
            write!(g, "]")?;

            values.push(String::from_utf8_lossy(&g).to_string());
        }

        if let Some(engine_type) = engine_type {
            keys.push("engineType");
            values.push(engine_type.to_string());
        }

        writeln!(f, "{indent_outer}generator {name} {{")?;

        let max_key_length = keys.iter().map(|s| s.len()).max().unwrap_or(0);

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
    fn test_print_simple() {
        let generator = Generator {
            name: "client".into(),
            provider: Provider::PrismaClientJs,
            output: None,
            binary_targets: Vec::new(),
            preview_features: Vec::new(),
            engine_type: None,
        };

        let mut f = Vec::new();

        generator.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "generator client {
  provider = \"prisma-client-js\"
}
"
        );
    }

    #[test]
    fn test_print_full() {
        let generator = Generator {
            name: "client".into(),
            provider: Provider::PrismaClientJs,
            output: Some("path/to/client".into()),
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
        };

        let mut f = Vec::new();

        generator.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "generator client {
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
        );
    }
}
