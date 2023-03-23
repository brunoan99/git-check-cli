use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Check all projects being tracked
  Check,
  /// Check a specific project
  CheckPath { name: String },
  /// Add a new project to the list of tracked projects
  AddPath { path: String },
  /// Remove a project from the list of tracked projects
  RemovePath { path: String },
}
