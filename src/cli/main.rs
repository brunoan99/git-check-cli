use super::Options;

use clap::Parser;
use colored::Colorize;
use inquire::{Confirm, Select, Text};
use yaml_rust::Yaml;

use crate::{cli::Commands, file_mgr, file_tracker, Display, Tracker};

#[must_use]
pub fn get_cli_options() -> Options {
  Options::parse()
}

fn get_in_text(msg: &str) -> String {
  match Text::new(&format!("{msg}:")).prompt() {
    Ok(resp) => resp,
    Err(err) => {
      println!("Error in response: {err}");
      std::process::exit(1);
    }
  }
}

fn get_in_confirm(msg: &str, default: bool) -> bool {
  match Confirm::new(msg).with_default(true).prompt() {
    Ok(resp) => resp,
    Err(err) => {
      eprintln!("Error in confirm: {err}");
      std::process::exit(1);
    }
  }
}

fn get_in_select<T: std::fmt::Display>(msg: &str, options: Vec<T>) -> T {
  match Select::new(msg, options).prompt() {
    Ok(resp) => resp,
    Err(err) => {
      eprintln!("Error in select: {err}");
      std::process::exit(1);
    }
  }
}

fn success(msg: &str) {
  println!("{msg}");
  std::process::exit(0);
}

fn fail(msg: &str) {
  println!("{msg}");
  std::process::exit(1);
}

pub fn run(tracker: &mut Tracker, config_path: String) {
  println!("{}", "Starting git-check-cli".bold());

  let options = get_cli_options();

  let mut option_set = tracker.options.clone();
  if let Some(v) = options.verbose {
    option_set.verbose = v;
  }

  match &options.command {
    Commands::Check => tracker
      .projects
      .iter()
      .map(|project| println!("{}", Display::from(project, &option_set)))
      .for_each(drop),
    Commands::CheckProject { name: name_op } => {
      let project_to_check = name_op.as_ref().map_or_else(
        || get_in_select("chose to remove:", tracker.projects.clone()),
        |name| match tracker.find_project(name) {
          Some(project) => project.clone(),
          None => get_in_select("chose to remove:", tracker.projects.clone()),
        },
      );
      println!("{}", Display::from(&project_to_check, &option_set));
    }
    Commands::AddProject {
      name: name_op,
      path: path_op,
    } => {
      let name = name_op
        .as_ref()
        .map_or_else(|| get_in_text("Name"), String::from);
      let path = path_op
        .as_ref()
        .map_or_else(|| get_in_text("Path"), String::from);
      let project_to_add = file_tracker::Project::new(name, path);
      println!("{project_to_add}");
      let confirmation = get_in_confirm("Confirm to add", true);
      if !confirmation {
        success("Project not added");
      }
      tracker.add_project(project_to_add);
      let yaml_tracker: Yaml = tracker.to_owned().into();
      match file_mgr::write_yaml_file(&config_path, &yaml_tracker) {
        Ok(_) => success("Project added successfully"),
        Err(err) => fail(&err.to_string()),
      }
    }
    Commands::RemoveProject { name: name_op } => {
      let project_to_remove = name_op.as_ref().map_or_else(
        || get_in_select("chose to remove:", tracker.projects.clone()),
        |name| match tracker.find_project(name) {
          Some(project) => project.clone(),
          None => get_in_select("chose to remove:", tracker.projects.clone()),
        },
      );
      println!("{project_to_remove}");
      let confirmation = get_in_confirm("Confirm to remove", true);
      if !confirmation {
        success("Project not removed");
      }
      tracker.remove_project(&project_to_remove);
      let yaml_tracker: Yaml = tracker.to_owned().into();
      match file_mgr::write_yaml_file(&config_path, &yaml_tracker) {
        Ok(_) => success("Project removed successfully"),
        Err(err) => fail(&err.to_string()),
      }
    }
  }
}
