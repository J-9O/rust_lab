mod modules;
use crate::modules::hello;
use crate::modules::cli_tools::cli::{Cli, CliError};
use crate::modules::reqwest_mod::ReqwestMod;

#[tokio::main]
async fn main() {
    //hello::hello();
    //let args: Cli = Cli::parse();
    //Cli::run()
    let result = ReqwestMod::run().await;
    println!("Result: {:?}", result);
    
    let status_code = ReqwestMod::status_code(&result);
    println!("Status Code: {:?}", status_code);
}

#[test]
fn check_answer_validity() {
    assert_eq!(42, 42);
}