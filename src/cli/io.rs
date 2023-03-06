use {
    crate::{
        ast::Ast,
        generator::{
            printer::Print,
            prisma,
            typescript,
        },
        ir::Ir,
    },
    std::{
        fs::{
            create_dir,
            read_to_string,
            write,
        },
        path::Path,
    },
};

/// The output sub-directory for generated Prisma files.
const PRISMA_OUTPUT_DIR: &str = "prisma";

/// The output sub-directory for generated TypeScript files.
const TYPESCRIPT_OUTPUT_DIR: &str = "typescript";

/// The file extension for generated Prisma files.
const PRISMA_FILE_EXTENSION: &str = "prisma";

/// The file extension for generated TypeScript files.
const TYPESCRIPT_FILE_EXTENSION: &str = "ts";

/// Check a source file for errors.
///
/// # Arguments
///
/// * `input` - The input file.
///
/// # Errors
///
/// Returns an error if the input file does not exist or contains errors.
pub fn check_file(input: &str) -> Result<(), String> {
    let input = determine_input(input)?;

    let input = read_to_string(input)
        .map_err(|error| format!("Could not read input file. {error}"))?;

    let (ast, _) = Ast::parse(&input)
        .map_err(|error| format!("Could not parse input file. {error}"))?;

    let _ir = Ir::from(&ast).map_err(|error| {
        format!("Could not generate intermediate representation. {error}")
    })?;

    Ok(())
}

/// Generate Prisma model and enums from an AST.
///
/// # Arguments
///
/// * `ast` - The AST to generate code from.
/// * `output` - The output directory.
///
/// # Errors
///
/// * Returns an error if the output directory does not exist and could not be
///   created.
/// * Returns an error if a file could not be written.
pub fn generate_prisma(
    ir: &Ir,
    output: &Path,
) -> Result<(), String> {
    let path = output.join(PRISMA_OUTPUT_DIR);

    if !path.is_dir() {
        create_dir(&path).map_err(|error| {
            format!("Could not create prisma output directory. {error}")
        })?;
    }

    let file = path.join(format!("application.{PRISMA_FILE_EXTENSION}"));
    let schema = prisma::Schema::from(ir);
    let source = schema.to_string();

    write(file, source)
        .map_err(|error| format!("Could not write prisma file. {error}"))
}

/// Generate TypeScript interfaces and enums from an AST.
///
/// # Arguments
///
/// * `ast` - The AST to generate code from.
/// * `output` - The output directory.
///
/// # Errors
///
/// * Returns an error if the output directory does not exist and could not be
///   created.
/// * Returns an error if a file could not be written.
pub fn generate_typescript(
    ast: &Ir,
    output: &Path,
) -> Result<(), String> {
    let path = output.join(TYPESCRIPT_OUTPUT_DIR);

    if !path.is_dir() {
        create_dir(&path).map_err(|error| {
            format!("Could not create typescript output directory. {error}")
        })?;
    }

    let mut source = Vec::new();

    for model in ast.models.values() {
        source.push(typescript::Interface::from(model).print(0));
    }

    for r#enum in ast.enums.values() {
        source.push(typescript::Enum::from(r#enum).print(0));
    }

    write(
        path.join(format!("application.{TYPESCRIPT_FILE_EXTENSION}")),
        source.join("\n\n"),
    )
    .map_err(|error| format!("Could not write typescript file. {error}"))
}

/// Generate code from a source file.
///
/// # Arguments
///
/// * `input` - The input file.
/// * `output` - The output directory.
///
/// # Errors
///
/// * Returns an error if the input file does not exist or contains errors.
/// * Returns an error if TypeScript files could not be generated.
/// * Returns an error if Prisma files could not be generated.
pub fn generate_all(
    input: &Path,
    output: &Path,
) -> Result<(), String> {
    let input = read_to_string(input)
        .map_err(|error| format!("Could not read input file. {error}"))?;

    let (ast, _) = Ast::parse(&input)
        .map_err(|error| format!("Could not parse input file. {error}"))?;

    let ir = Ir::from(&ast).map_err(|error| {
        format!("Could not generate intermediate representation. {error}")
    })?;

    generate_typescript(&ir, output)?;
    generate_prisma(&ir, output)?;

    Ok(())
}

/// Compile an input file.
///
/// # Arguments
///
/// * `input` - The input file.
/// * `output` - The output directory.
///
/// # Errors
///
/// * Returns an error if the input file does not exist or contains errors.
/// * Returns an error if the output directory does not exist and could not be
///   created.
pub fn compile(
    input: &str,
    output: Option<&str>,
) -> Result<(), String> {
    let input = determine_input(input)?;
    let output = determine_output(output)?;

    generate_all(input, output)?;

    Ok(())
}

/// Determine the input file.
///
/// # Arguments
///
/// * `input` - The input file.
///
/// # Errors
///
/// Returns an error if the input file does not exist.
pub fn determine_input(input: &str) -> Result<&Path, String> {
    let path = Path::new(input);

    if path.is_file() {
        Ok(path)
    } else {
        Err(format!("input file `{input}` does not exist."))
    }
}

/// Determine the output directory, creating it if necessary.
///
/// # Arguments
///
/// * `output` - The output directory.
///
/// # Errors
///
/// Returns an error if the output directory does not exist and could not be
/// created.
fn determine_output(output: Option<&str>) -> Result<&Path, String> {
    let output = output.map_or_else(|| "out", |output| output);
    let path = Path::new(output);

    if path.is_dir() || create_dir(path).is_ok() {
        Ok(path)
    } else {
        Err(format!("failed to create output directory `{output}`."))
    }
}
