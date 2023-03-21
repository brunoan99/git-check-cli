use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,

  /// Turn verbose mode on
  #[arg(short, long)]
  pub verbose: bool,

  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<PathBuf>,

  /// Sets a custom project list file
  #[arg(short, long, value_name = "FILE")]
  pub paths: Option<PathBuf>,
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
