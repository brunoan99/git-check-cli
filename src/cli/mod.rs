#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs, process};

use clap::{command, Parser, Subcommand};
use inquire::{Confirm, Select, Text};
use yaml_rust::{Yaml, YamlEmitter};

use crate::{file_tracker, git_tracker, Tracker, VerboseDisplay};

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

pub fn run_cli(tracker: &mut Tracker, config_path: String) {
  let options = get_cli_options();

  match &options.command {
    Commands::Check => {
      let git_results: Vec<VerboseDisplay> = tracker
        .projects
        .iter()
        .map(git_tracker::map_result_to_verbose_display)
        .collect();
      for result in git_results {
        println!("{result}");
      }
    }
    Commands::CheckPath => {
      let project = Select::new("chose to check:", tracker.projects.clone())
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error selecting project, err: {err}");
          process::exit(1);
        });
      let git_result = git_tracker::map_result_to_verbose_display(&project);
      println!("{git_result}");
    }
    Commands::AddPath => {
      let name = Text::new("Project name:").prompt().unwrap_or_else(|err| {
        eprintln!("Error in response: {err}");
        process::exit(1);
      });
      let path = Text::new("Project path:").prompt().unwrap_or_else(|err| {
        eprintln!("Error in response: {err}");
        process::exit(1);
      });
      let new_project = file_tracker::Project::new(name, path);
      println!("{new_project}");
      let confirm = Confirm::new("Confirm to add to projects list")
        .with_default(true)
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error in response: {err}");
          process::exit(1);
        });
      if !confirm {
        println!("Project not added");
      }
      tracker.add_project(new_project);
      let yaml_projects_list: Yaml = tracker.clone().into();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(config_path, out_str) {
        Ok(_) => println!("Project added successfully"),
        Err(err) => {
          eprintln!("Failed to sync file, getting error: {err}");
          process::exit(1);
        }
      }
    }
    Commands::RemovePath => {
      let to_remove = Select::new("chose to remove: ", tracker.projects.clone())
        .prompt()
        .unwrap_or_else(|_| {
          eprintln!("Error selecting project");
          process::exit(1);
        });
      let confirm = Confirm::new("Confirm to remove the selected path from projects list")
        .with_default(true)
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error in response: {err}");
          process::exit(1);
        });
      if !confirm {
        println!("Project not removed");
      }
      tracker.remove_project(&to_remove);
      let yaml_projects_list: Yaml = tracker.clone().into();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(config_path, out_str) {
        Ok(_) => println!("Project removed successfully"),
        Err(err) => {
          eprintln!("failed to sync with file, err: {err}");
          process::exit(1)
        }
      }
    }
  }
}
