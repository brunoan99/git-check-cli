#[path = "cli/cli.rs"]
mod cli;
use inquire::Select;
use std::path::PathBuf;
use std::process::{self, Command};
use std::str::from_utf8;
use std::{ffi::OsString, fs};
use yaml_rust::{Yaml, YamlLoader};

use clap::Parser;

fn path_of_project_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

struct EvalError(String);
fn eval_path_to_absolute(exp: String) -> Result<String, EvalError> {
  println!("eval_path_to_absolute called with {}", exp);
  let path_to_eval = ["/bin/echo", &exp].join(" ");
  let mut cmd = Command::new("sh");
  cmd.args(["-c", &path_to_eval]);
  let mut output = match cmd.output() {
    Ok(out) => out,
    Err(err) => return Err(EvalError(String::from(err.to_string()))),
  };
  output
    .stdout
    .resize(output.stdout.len() - 1, output.stdout[0]);
  Ok(String::from(
    from_utf8(&output.stdout).expect("failed to parse output"),
  ))
}

fn hidrate_project_list(obj: &Vec<Yaml>) -> Option<Vec<String>> {
  let project_list = &obj[0]["project-list"];
  let vec_projects = project_list.as_vec().unwrap();

  let mut vec_paths: Vec<String> = vec![];
  for item in vec_projects {
    let path = item["path"].as_str().unwrap();
    vec_paths.push(String::from(path));
  }

  let mut vec_absolute_paths: Vec<String> = vec![];
  for item in vec_paths {
    let path = match item.contains("$") {
      true => match eval_path_to_absolute(item) {
        Ok(absolute_path) => absolute_path,
        Err(_) => return None,
      },
      false => item,
    };
    vec_absolute_paths.push(path);
  }

  Some(vec_absolute_paths)
}

fn main() {
  println!("Starting git-check-cli");

  let options = cli::Options::parse();

  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Problem finding home directory!");
    process::exit(1);
  });

  let projects_file = path_of_project_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Problem getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(projects_file).unwrap_or_else(|err| {
    eprintln!("Problem reading projects list file, error: {}", err);
    process::exit(1);
  });

  let configs = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Problem parsing file, error: {}", err);
    process::exit(1);
  });

  match &options.command {
    cli::Commands::Check => {
      // projects_file
    }
    cli::Commands::CheckPath { name: _ } => {
      //
    }
    cli::Commands::AddPath { path: _, name: _ } => {
      //
    }
    cli::Commands::RemovePath {} => {
      let options: Vec<&str> = vec!["name", "path"];
      let ans = Select::new("remove by: ", options).prompt();

      match ans {
        Ok("name") => println!("name! was chosen"), // TODO: another Select prompt to select which project by name to remove
        Ok("path") => {
          let hydrated_vec = match hidrate_project_list(&configs.clone()) {
            Some(vec) => vec,
            None => {
              eprintln!("failed to get project list");
              process::exit(1);
            }
          };
          let remove_ans = Select::new("chose to remove: ", hydrated_vec).prompt();
          match remove_ans {
            Ok(to_remove) => println!("chose to remove: {}", to_remove),
            _ => {
              eprintln!("Error in selection, please try again");
              process::exit(1);
            }
          }
        }
        _ => {
          eprintln!("Error in selection, please try again");
          process::exit(1);
        }
      }
    }
  }
}
