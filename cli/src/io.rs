use {
    ast::Ast,
    ir::Ir,
    print::Print,
    prisma,
    std::{
        fs::{
            create_dir_all,
            read_to_string,
            File,
        },
        io::Write,
        path::Path,
    },
    typescript,
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
pub fn check_file<P>(input: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let input = read_to_string(input)
        .map_err(|error| format!("Could not read input file. {error}"))?;

    let (ast, _) = Ast::parse(&input)
        .map_err(|error| format!("Could not parse input file. {error}"))?;

    let _ir = Ir::try_from(ast).map_err(|error| {
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
pub fn generate_prisma<P>(
    ir: Ir,
    output: P,
) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let path = output.as_ref().join(PRISMA_OUTPUT_DIR);

    if !path.is_dir() {
        create_dir_all(&path).map_err(|error| {
            format!("Could not create prisma output directory. {error}")
        })?;
    }

    let file = path.join(format!("application.{PRISMA_FILE_EXTENSION}"));
    let schema = prisma::Schema::try_from(ir)
        .map_err(|_| "Could not generate prisma schema.")?;
    let mut source = Vec::new();

    schema
        .print(0, &mut source)
        .map_err(|error| format!("Could not write prisma schema. {error}"))?;

    let mut file = File::create(file).map_err(|error| {
        format!("Could not create prisma schema file. {error}")
    })?;

    file.write_all(&source)
        .map_err(|error| format!("Could not write prisma schema. {error}"))?;

    Ok(())
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
pub fn generate_typescript<P>(
    ast: &Ir,
    output: P,
) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let path = output.as_ref().join(TYPESCRIPT_OUTPUT_DIR);

    if !path.is_dir() {
        create_dir_all(&path).map_err(|error| {
            format!("Could not create typescript output directory. {error}")
        })?;
    }

    let mut file_path = path.join("index");
    let _: bool = file_path.set_extension(TYPESCRIPT_FILE_EXTENSION);

    let mut file = File::create(file_path).map_err(|error| {
        format!("Could not create typescript index file. {error}")
    })?;

    for model in ast.models.values() {
        typescript::Interface::from(model.clone())
            .print(0, &mut file)
            .map_err(|error| {
                format!(
                    "Could not write typescript interface for model `{}`. \
                     {error}",
                    model.name()
                )
            })?;
    }

    for r#enum in ast.enums.values() {
        typescript::Enum::from(r#enum.clone())
            .print(0, &mut file)
            .map_err(|error| {
                format!(
                    "Could not write typescript enum for enum `{}`. {error}",
                    r#enum.name
                )
            })?;
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
pub fn compile<P>(
    input: P,
    output: P,
) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let input = read_to_string(input)
        .map_err(|error| format!("Could not read input file. {error}"))?;

    let (ast, _) = Ast::parse(&input)
        .map_err(|error| format!("Could not parse input file. {error}"))?;

    let ir = Ir::try_from(ast).map_err(|error| {
        format!("Could not generate intermediate representation. {error}")
    })?;

    generate_typescript(&ir, &output)?;
    generate_prisma(ir, &output)?;

    Ok(())
}
