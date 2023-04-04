use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Options {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Check all projects being tracked, and do some computations on project list
  Check,
  /// Check a specific project
  CheckPath,
  /// Add a new project to the list of tracked projects
  AddPath,
  /// Remove a project from the list of tracked projects
  RemovePath,
}

pub fn get_cli_options() -> Options {
  Options::parse()
}
