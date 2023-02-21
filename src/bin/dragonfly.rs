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
            Error,
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
) {
    let input = match std::fs::read_to_string(input) {
        Ok(input) => input,
        Err(error) => {
            println!("Could not read input file: {error}");
            return;
        }
    };

    let (ast, _) = match dragonfly::ast::Ast::parse(&input) {
        Ok(ast) => ast,
        Err(error) => {
            println!("Could not parse input file: {error}");
            return;
        }
    };

    for (name, model) in ast.models {
        let typescript_file = output.join(format!("{name}.ts"));
        let typescript_source = Interface::from(model).print(0);

        if let Err(error) = write(typescript_file, typescript_source) {
            println!("Could not write TypeScript file: {error}");
        }
    }
}

/// The entry point for the command line interface.
pub fn main() {
    let args = env::args().collect::<Vec<_>>();

    match parse_args(&args) {
        Ok(Command::Help) => println!("{}", usage()),
        Ok(Command::Version) => println!("{}", version()),
        Ok(Command::Compile { input, output }) => {
            let input = Path::new(&input);

            if !input.is_file() {
                println!("Input file does not exist.");
            }

            output.map_or_else(
                || {
                    let output = Path::new("./out");

                    if !output.is_dir() {
                        if let Err(error) = create_dir(output) {
                            println!(
                                "Could not create output directory: {error}"
                            );
                        }
                    }

                    generate(input, output);
                },
                |output| {
                    let output = Path::new(&output);

                    if !output.is_dir() {
                        println!("Output directory does not exist.");
                    }

                    generate(input, output);
                },
            );
        }
        Err(Error::ParseArgs) => {
            println!("Could not parse arguments.\n\n{}", usage());
        }
        _ => {
            println!("Unimplemented.\n{}", usage());
        }
    }
}
