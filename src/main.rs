use clap::Parser;
use git_check_cli::{Cli, Commands};

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Check => println!("Check command is available"),
  }
}
