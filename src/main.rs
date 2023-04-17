use colored::Colorize;
use inquire::{Confirm, Select, Text};
use mylib::file_tracker::Tracker;
use mylib::{cli, file_tracker, git_tracker, ShortDisplay, VerboseDisplay};
use std::path::PathBuf;
use std::process;
use std::{ffi::OsString, fs};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

// proceess

fn path_of_projects_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

// main

fn setup_config() -> (String, file_tracker::Tracker) {
  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Error finding home directory!");
    process::exit(1);
  });

  let config_path = path_of_projects_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Error getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(config_path.clone()).unwrap_or_else(|err| {
    eprintln!("Error reading projects list file, error: {err}");
    process::exit(1);
  });

  let configs_yaml = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Error parsing file, error: {err}");
    process::exit(1);
  });

  let tracker = Tracker::try_from(configs_yaml).unwrap_or_else(|err| {
    eprintln!("{err}");
    process::exit(1);
  });

  (config_path, tracker)
}

fn main() {
  let options = cli::get_cli_options();

  let (configs_path, mut tracker) = setup_config();

  println!("{}", "Starting git-check-cli".bold());

  match &options.command {
    cli::Commands::Check => {
      let git_results: Vec<VerboseDisplay> = tracker
        .projects
        .iter()
        .map(git_tracker::map_result_to_verbose_display)
        .collect();
      for result in git_results {
        println!("{result}");
      }
    }
    cli::Commands::CheckPath => {
      let project = Select::new("chose to check:", tracker.projects.clone())
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error selecting project, err: {err}");
          process::exit(1);
        });
      let git_result = git_tracker::map_result_to_verbose_display(&project);
      println!("{git_result}");
    }
    cli::Commands::AddPath => {
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
      let yaml_projects_list: Yaml = tracker.into();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(configs_path, out_str) {
        Ok(_) => println!("Project added successfully"),
        Err(err) => {
          eprintln!("Failed to sync file, getting error: {err}");
          process::exit(1);
        }
      }
    }
    cli::Commands::RemovePath => {
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
      let yaml_projects_list: Yaml = tracker.into();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(configs_path, out_str) {
        Ok(_) => println!("Project removed successfully"),
        Err(err) => {
          eprintln!("failed to sync with file, err: {err}");
          process::exit(1)
        }
      }
    }
  }
}
