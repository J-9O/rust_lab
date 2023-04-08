use clap::Parser;

mod modules;
use crate::modules::cli::CliError;
use crate::modules::hello;
use crate::modules::cli;

fn main() -> Result<(), cli::CliError> {
    hello::hello();
    /* Function snippet for parsing input via the command line */
    let args = cli::Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CliError(format!("Error reading: `{:?}`: {}", &args.path, err)))?;

    println!("{}", content);
    Ok(())
}