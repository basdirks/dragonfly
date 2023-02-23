//! The command line interface.
#![deny(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::if_then_some_else_none,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    unused_results,
    variant_size_differences
)]

use {
    dragonfly::{
        ast::Ast,
        cli::{
            command::Command,
            help_build_message,
            help_check_message,
            help_message,
            parse_args,
            version,
        },
        generator::{
            printer::Print,
            typescript::{
                Enum,
                Interface,
            },
        },
    },
    std::{
        env,
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
fn print_to_file<T: Print>(
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
fn check(input: &str) -> Result<(), String> {
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
fn generate(
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

    for (name, model) in ast.models {
        print_to_file(&name, &Interface::from(model), output)?;
    }

    for (name, r#enum) in ast.enums {
        print_to_file(&name, &Enum::from(r#enum), output)?;
    }

    Ok(())
}

/// Compile an input file.
///
/// # Arguments
///
/// * `input` - The input file.
/// * `output` - The output directory.
fn compile(
    input: &str,
    output: Option<&str>,
) -> Result<(), String> {
    let input = determine_input(input)?;
    let output = determine_output(output)?;

    generate(input, output)?;

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
fn determine_input(input: &str) -> Result<&Path, String> {
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

/// Parse the arguments, execute the command, and print the result.
pub fn main() {
    let args = env::args().collect::<Vec<_>>();

    match parse_args(&args) {
        Command::Help => {
            println!("{}", help_message());
        }
        Command::HelpCommand { command } => {
            if command.as_str() == "build" {
                println!("{}", help_build_message());
            } else if command.as_str() == "check" {
                println!("{}", help_check_message());
            } else {
                println!("Unknown command `{command}`.");
                println!("{}", help_message());
            }
        }
        Command::Version => {
            println!("{}", version());
        }
        Command::Build { input, output } => {
            if let Err(error) = compile(&input, output.as_deref()) {
                println!("An error occurred during compilation: {error}");
            } else {
                println!("Compiled successfully.");
            }
        }
        Command::Check { input } => {
            if let Err(error) = check(&input) {
                println!("Error while checking `{input}`:\n{error}");
            } else {
                println!("Checked `{input}` successfully.");
            }
        }
    }
}
