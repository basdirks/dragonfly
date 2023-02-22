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
        cli::{
            command::Command,
            parse_args,
            usage,
            version,
        },
        generator::{
            printer::Print,
            typescript::Interface,
        },
    },
    std::{
        env,
        fs::{
            create_dir,
            write,
        },
        path::Path,
    },
};

/// Generate code from a file.
///
/// # Arguments
///
/// * `input` - The input file.
/// * `output` - The output directory.
fn generate(
    input: &Path,
    output: &Path,
) -> Result<(), String> {
    let input = match std::fs::read_to_string(input) {
        Ok(input) => input,
        Err(error) => {
            return Err(format!("Could not read input file: {error}"));
        }
    };

    let (ast, _) = match dragonfly::ast::Ast::parse(&input) {
        Ok(ast) => ast,
        Err(error) => {
            return Err(format!("Could not parse input file: {error}"));
        }
    };

    if let Err(error) = ast.check() {
        return Err(error.to_string());
    }

    for (name, model) in ast.models {
        let file = output.join(format!("{name}.ts"));
        let file_clone = file.clone();
        let file_display = file_clone.display();
        let source = Interface::from(model).print(0);

        if let Err(error) = write(file, source) {
            return Err(format!(
                "Could not write TypeScript file \"{file_display}\": {error}"
            ));
        }

        println!("Wrote TypeScript interface to {file_display}");
    }

    Ok(())
}

/// Print usage information.
fn print_usage() {
    println!("{}", usage());
}

/// Print version information.
fn print_version() {
    println!("{}", version());
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
fn determine_input(input: &str) -> Result<&Path, &str> {
    let input = Path::new(input);

    if input.is_file() {
        Ok(input)
    } else {
        Err("Input file does not exist.")
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
fn determine_output(output: Option<&str>) -> Result<&Path, &str> {
    output.map_or_else(
        || {
            let path = Path::new("./out");

            if !path.is_dir() && create_dir(path).is_err() {
                return Err("Could not create output directory: \"./out\"");
            }

            Ok(path)
        },
        |output| {
            let path = Path::new(output);

            if !path.is_dir() {
                println!("Output directory does not exist: \"{output}\"");
            }

            Ok(path)
        },
    )
}

/// The entry point for the command line interface.
pub fn main() {
    let args = env::args().collect::<Vec<_>>();

    if let Some(command) = parse_args(&args) {
        match command {
            Command::Help => print_usage(),
            Command::Version => print_version(),
            Command::Compile { input, output } => {
                if let Err(error) = compile(&input, output.as_deref()) {
                    println!("Could not compile: {error}");
                } else {
                    println!("Compiled successfully.");
                }

                return;
            }
        }
    }

    print_usage();
}
