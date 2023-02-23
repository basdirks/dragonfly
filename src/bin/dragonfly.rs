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
        self,
        cli::{
            command::Command,
            help_build_message,
            help_check_message,
            help_message,
            io::{
                check_file,
                compile,
            },
            parse_args,
            version,
        },
    },
    std::{
        self,
        env,
    },
};

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
            }
        }
        Command::Check { input } => {
            if let Err(error) = check_file(&input) {
                println!("Error while checking `{input}`:\n{error}");
            } else {
                println!("No errors found in `{input}`.");
            }
        }
    }
}
