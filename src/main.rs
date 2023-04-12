mod modules;
use crate::modules::hello;
use crate::modules::cli_tools::cli;
use cli::Cli as Cli;

fn main() -> Result<(), cli::CliError> {
    hello::hello();
    //let args: Cli = Cli::parse();

    Cli::run()
}

#[test]
fn check_answer_validity() {
    assert_eq!(42, 42);
}