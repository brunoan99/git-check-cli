use colored::Colorize;
use inquire::{Confirm, Select, Text};
use mylib::{cli, config};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{self, Command};
use std::str::from_utf8;
use std::{ffi::OsString, fs};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

// proceess

fn success_exit(msg: &str) {
  println!("{msg}");
  process::exit(0);
}

fn error_exit(msg: &str) {
  eprintln!("{msg}");
  process::exit(1);
}

fn path_of_projects_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

#[derive(Debug)]
struct EvalError(String);

impl Display for EvalError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Error Evaluating [error: {}]", self.0)
  }
}

fn eval_path_to_absolute(exp: &str) -> Result<String, EvalError> {
  let path_to_eval = ["/bin/echo", exp].join(" ");
  let mut cmd = Command::new("sh");
  cmd.args(["-c", &path_to_eval]);
  let mut output = match cmd.output() {
    Ok(out) => out,
    Err(err) => return Err(EvalError(err.to_string())),
  };
  output
    .stdout
    .resize(output.stdout.len() - 1, output.stdout[0]);
  let abs_path = from_utf8(&output.stdout);
  match abs_path {
    Ok(absolute_path) => Ok(String::from(absolute_path)),
    Err(err) => Err(EvalError(err.to_string())),
  }
}

// git

struct Unuptated(String);

impl Display for Unuptated {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.0)
  }
}

struct Uncommited(String);

impl Display for Uncommited {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Uncommited Changes: \n{}", self.0)
  }
}

struct Unpublished(String);

impl Display for Unpublished {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Unpublished Changes: \n{}", self.0)
  }
}

struct Unpulled(String);

impl Display for Unpulled {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Unpulled Changes: \n{}", self.0)
  }
}

fn git_checkouts(path: &str) -> Result<(), Unuptated> {
  let mut unupdated = String::new();
  match check_uncommited_changes(path) {
    Ok(_) => (),
    Err(Uncommited(msg)) => {
      unupdated.push_str(&msg);
    }
  };
  match check_unpublished_changes(path) {
    Ok(_) => (),
    Err(Unpublished(msg)) => {
      unupdated.push_str(&msg);
    }
  };
  if unupdated.is_empty() {
    Ok(())
  } else {
    Err(Unuptated(unupdated))
  }
}

fn check_uncommited_changes(path: &str) -> Result<(), Uncommited> {
  let check = Command::new("/bin/git")
    .arg("status")
    .arg("--short")
    .current_dir(path)
    .output()
    .unwrap();
  let result = from_utf8(&check.stdout).unwrap();
  if result.is_empty() {
    Err(Uncommited(String::from(result)))
  } else {
    Ok(())
  }
}

fn check_unpublished_changes(path: &str) -> Result<(), Unpublished> {
  let check = Command::new("/bin/git")
    .args([
      "log",
      "--branches",
      "--not",
      "--remotes",
      "--simplify-by-decoration",
      "--decorate",
      "--oneline",
    ])
    .current_dir(path)
    .output()
    .unwrap();
  let result = from_utf8(&check.stdout).unwrap();
  if result.is_empty() {
    Err(Unpublished(String::from(result)))
  } else {
    Ok(())
  }
}

// main

fn setup_config() -> (String, config::Config) {
  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Error finding home directory!");
    process::exit(1);
  });

  let config_file_path = path_of_projects_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Error getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(config_file_path.clone()).unwrap_or_else(|err| {
    eprintln!("Error reading projects list file, error: {err}");
    process::exit(1);
  });

  let configs_yaml = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Error parsing file, error: {err}");
    process::exit(1);
  });

  let configs = config::Config::try_from(configs_yaml).unwrap_or_else(|err| {
    eprintln!("{err}");
    process::exit(1);
  });

  (config_file_path, configs)
}

fn main() {
  let options = cli::get_cli_options();

  let (configs_path, mut configs) = setup_config();

  println!("Starting git-check-cli");

  match &options.command {
    cli::Commands::Check => {
      for project in configs.projects {
        let absolute_path = eval_path_to_absolute(&project.path).unwrap_or_else(|err| {
          eprintln!("Error getting absolute path: {err}");
          process::exit(1)
        });
        match git_checkouts(&absolute_path) {
          Ok(_) => println!(
            "\nChecking for ({}) completed successfully",
            project.name.clone().yellow().bold()
          ),
          Err(msg) => {
            println!("Project {}", project.name.clone().bold().yellow());
            println!("{msg}");
          }
        }
      }
    }
    cli::Commands::CheckPath {} => {
      let project = Select::new("chose to check:", configs.projects.clone())
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error selecting project, err: {err}");
          process::exit(1);
        });
      let absolute_path = eval_path_to_absolute(&project.path).unwrap_or_else(|err| {
        eprintln!("Error getting absolute path: {err}");
        process::exit(1)
      });
      match git_checkouts(&absolute_path) {
        Ok(_) => println!("\nChecking completed successfully"),
        Err(msg) => {
          println!("Project {}", project.name.bold().yellow());
          println!("{msg}");
        }
      }
    }
    cli::Commands::AddPath {} => {
      let name = Text::new("Project name:").prompt().unwrap_or_else(|err| {
        eprintln!("Error in response: {err}");
        process::exit(1);
      });
      let path = Text::new("Project path:").prompt().unwrap_or_else(|err| {
        eprintln!("Error in response: {err}");
        process::exit(1);
      });
      let new_project = config::Project::new(name, path);
      println!("{new_project}");
      let confirm = Confirm::new("Confirm to add to projects list")
        .with_default(true)
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error in response: {err}");
          process::exit(1);
        });
      if !confirm {
        success_exit("Project not added");
      }
      configs.add_project(new_project);
      let yaml_projects_list: Yaml = configs.into();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(configs_path, out_str) {
        Ok(_) => success_exit("Project added successfully"),
        Err(err) => error_exit(&format!("Failed to sync file, getting error: {err}")),
      }
    }
    cli::Commands::RemovePath {} => {
      let to_remove = Select::new("chose to remove: ", configs.projects.clone())
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
        success_exit("Project not removed");
      }
      configs.remove_project(&to_remove);
      let yaml_projects_list: Yaml = configs.into();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(configs_path, out_str) {
        Ok(_) => success_exit("Project removed successfully"),
        Err(err) => {
          eprintln!("failed to sync with file, err: {err}");
          process::exit(1)
        }
      }
    }
  }
}
