use std::path::{Path, PathBuf};

use clap::Parser;
use git_check_cli::{Cli, Commands};

fn main() {
  let cli = Cli::parse();

  let verbose = cli.verbose;

  let config_file = if let Some(config_path) = cli.config.as_deref() {
    config_path
  } else {
    Path::new("$HOME/.config/config.yaml")
  };

  let paths_file = if let Some(paths) = cli.paths.as_deref() {
    paths
  } else {
    Path::new("$HOME/.config/projects.yaml")
  };

  match &cli.command {
    Commands::Check => {
      //
    }
    Commands::CheckPath { name } => {
      //
    }
    Commands::AddPath { path } => {
      //
    }
    Commands::RemovePath { path } => {
      //
    }
    _ => println!("another command"),
  }
}
