use {
    crate::{
        ast::Ast,
        generator::{
            printer::Print,
            prisma::{
                Enum as PrismaEnum,
                Model as PrismaModel,
            },
            typescript::{
                Enum as TypescriptEnum,
                Interface as TypescriptInterface,
            },
        },
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

/// Write a string to a file.
///
/// # Arguments
///
/// * `file` - The file to write to.
/// * `source` - The source code to write.
///
/// # Errors
///
/// Returns an error if the file could not be written.
fn write_to_file(
    file: &Path,
    source: String,
) -> Result<(), String> {
    let file_display = file.display();

    if let Err(error) = write(file, source) {
        return Err(format!(
            "Could not write file `{file_display}`: `{error}`"
        ));
    }

    println!("Generated `{file_display}`");

    Ok(())
}

/// Generate a file for one printable entity.
///
/// # Arguments
///
/// * `name` - The name of the entity.
/// * `entity` - The entity to generate code from.
/// * `output` - The output directory.
/// * `extension` - The file extension.
///
/// # Errors
///
/// Returns an error if the file could not be written.
pub fn print_to_file<T: Print>(
    name: &str,
    entity: &T,
    output: &Path,
    extension: &str,
) -> Result<(), String> {
    let path = output.join(format!("{name}.{extension}"));

    write_to_file(&path, entity.print(0))
}

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
        .map_err(|error| format!("Could not read input file: {error}"))?;

    let (ast, _) = Ast::parse(&input)
        .map_err(|error| format!("Could not parse input file: {error}"))?;

    if let Err(error) = ast.check() {
        return Err(error.to_string());
    }

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
    ast: &Ast,
    output: &Path,
) -> Result<(), String> {
    let path = output.join(PRISMA_OUTPUT_DIR);

    if !path.is_dir() {
        create_dir(&path).map_err(|error| {
            format!("Could not create prisma output directory: {error}")
        })?;
    }

    let mut source = String::new();

    for model in ast.models.values() {
        source.push_str(&PrismaModel::from(model).print(0));
        source.push_str("\n\n");
    }

    for r#enum in ast.enums.values() {
        source.push_str(&PrismaEnum::from(r#enum).print(0));
        source.push_str("\n\n");
    }

    let file = path.join(format!("application.{PRISMA_FILE_EXTENSION}"));

    write_to_file(&file, source)
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
    ast: &Ast,
    output: &Path,
) -> Result<(), String> {
    let path = output.join(TYPESCRIPT_OUTPUT_DIR);

    if !path.is_dir() {
        create_dir(&path).map_err(|error| {
            format!("Could not create typescript output directory: {error}")
        })?;
    }

    for (name, model) in &ast.models {
        print_to_file(
            name,
            &TypescriptInterface::from(model),
            &path,
            TYPESCRIPT_FILE_EXTENSION,
        )?;
    }

    for (name, r#enum) in &ast.enums {
        print_to_file(
            name,
            &TypescriptEnum::from(r#enum),
            &path,
            TYPESCRIPT_FILE_EXTENSION,
        )?;
    }

    Ok(())
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
        .map_err(|error| format!("Could not read input file: {error}"))?;

    let (ast, _) = Ast::parse(&input)
        .map_err(|error| format!("Could not parse input file: {error}"))?;

    if let Err(error) = ast.check() {
        return Err(error.to_string());
    }

    generate_typescript(&ast, output)?;
    generate_prisma(&ast, output)?;

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
