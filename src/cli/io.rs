use {
    crate::{
        ast::Ast,
        generator::{
            printer::Print,
            typescript::{
                Enum,
                Interface,
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

/// Generate a file for one printable entity.
///
/// # Arguments
///
/// * `name` - The name of the entity.
/// * `entity` - The entity to generate code from.
/// * `output` - The output directory.
///
/// # Errors
///
/// Returns an error if the file could not be written.
pub fn print_to_file<T: Print>(
    name: &str,
    entity: &T,
    output: &Path,
) -> Result<(), String> {
    let file = output.join(format!("{name}.ts"));
    let file_clone = file.clone();
    let file_display = file_clone.display();
    let source = entity.print(0);

    if let Err(error) = write(file, source) {
        return Err(format!(
            "Could not write file `{file_display}`: `{error}`"
        ));
    }

    println!("Generated `{file_display}`");

    Ok(())
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

/// Generate code from a source file.
///
/// # Arguments
///
/// * `input` - The input file.
/// * `output` - The output directory.
///
/// # Errors
///
/// Returns an error if the input file does not exist or contains errors.
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

    let typescript_path = output.join("typescript");

    if !typescript_path.is_dir() {
        create_dir(&typescript_path).map_err(|error| {
            format!("Could not create typescript output directory: {error}")
        })?;
    }

    for (name, model) in ast.models {
        print_to_file(&name, &Interface::from(model), &typescript_path)?;
    }

    for (name, r#enum) in ast.enums {
        print_to_file(&name, &Enum::from(r#enum), &typescript_path)?;
    }

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
