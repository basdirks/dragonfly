//! The command line interface.
#![deny(
    clippy::all,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::missing_docs_in_private_items,
    clippy::mixed_read_write_in_expression,
    clippy::nursery,
    clippy::pedantic,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

use {
    dragonfly::{
        self,
        cli::{
            execute,
            parse_args,
        },
    },
    std::{
        self,
        env,
        io::stdout,
    },
};

/// Run the CLI.
///
/// # Errors
///
/// * Returns an error if the command line arguments could not be parsed.
/// * Returns an error if the command could not be executed.
/// * Returns an error if the output could not be written.
pub fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<_>>();
    let command = parse_args(&args);
    let mut stdout = stdout().lock();

    execute(command, &mut stdout)
}
