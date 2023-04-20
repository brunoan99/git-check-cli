#![allow(dead_code)]
#![allow(unused_variables)]

mod main;
pub use main::*;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Options {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
  /// Check all projects being tracked, and do some computations on project list.
  Check,
  /// Check a specific project, if both parameters (name, path) was not provided Cli will request to user select from the projects list.
  CheckProject {
    /// The name of the project to be checked, if in projects list has 2 projects with same name, the first found with equivalent name will be checked.
    name: Option<String>,
    /// The Path of the project to be checked, if in projects list has 2 projects with same path, the first found with equivalent name will be checked.
    path: Option<String>,
  },
  /// Add a new project to the list of tracked projects, if parameters (name, path) was not provided Cli will request to user.
  AddPath {
    /// Name to be added to projects list, if was not provided Cli will request for user.
    name: Option<String>,
    /// Path to be added to projects list, if was not provided Cli will request for user.
    path: Option<String>,
  },
  /// Remove a project from the list of tracked projects, if both parameteres (name, path) was not provided Cli will request to user select from the projects list.
  RemovePath {
    /// If in projects list has 2 projects with same name, the first found with equivalent name will be removed.
    name: Option<String>,
    /// If in projects list has 2 projects with same path, the first found with equivalent name will be removed.
    path: Option<String>,
  },
}
