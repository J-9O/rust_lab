pub mod cli {
  use clap::Parser;

  #[derive(Parser)]
  pub struct Cli {
    pattern: char,
    path: std::path::PathBuf,
  }

  impl Cli {
    fn parse_options(s: &char) -> OPTIONS {
        match s {
            'a' => OPTIONS::A,
            'b' => OPTIONS::B,
             _  => OPTIONS::OpErr
        }
    }

    pub fn run() -> Result<(), CliError> {
        let args: Cli = Cli::parse();
        let option: OPTIONS = Self::parse_options(&args.pattern);
        let opt_content: String = option.handle();

        let content: String = std::fs::read_to_string(&args.path)
            .map_err(|err| 
                CliError( format!("Error reading: `{:?}`: {}", &args.path, err) ) 
            )?;

        println!("{}", content);
        println!("Selection: {}", opt_content);
        Ok(())
      }
  }

  enum OPTIONS {
      A,
      B,
      OpErr
  }

  impl OPTIONS {
      pub fn handle(self: Self) -> String {
          match self {
              OPTIONS::A => String::from("You picked option A"),
              OPTIONS::B => String::from("You picked option B"),
              OPTIONS::OpErr => String::from("You chose an invalid option")
          }
      }
  }

  #[derive(Parser)]
  pub struct Cli2 {
      pub option: String,
      pub file: String
  }

  #[derive(Debug)]
  pub struct CliError(pub String);
}