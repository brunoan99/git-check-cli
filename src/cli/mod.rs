#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Parser, Subcommand};

use crate::file_tracker;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Options {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Clone, Copy)]
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

#[must_use]
pub fn get_cli_options() -> Options {
  Options::parse()
}

// GET CLI FUNCTIONS TO HERE
pub fn process_input(option: Commands, configs: &mut file_tracker::Tracker) -> Result<(), ()> {
  Ok(())
}
