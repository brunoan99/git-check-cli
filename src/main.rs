#[path = "cli/cli.rs"]
mod cli;
use inquire::{Select, Text};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{self, Command};
use std::str::from_utf8;
use std::{ffi::OsString, fs};
use yaml_rust::{Yaml, YamlLoader};

use clap::Parser;

#[derive(Debug)]
struct ProjectList {
  id: i64,
  name: String,
  path: String,
}

impl ProjectList {
  fn fromYaml(yaml: &Yaml) -> ProjectList {
    Self {
      id: yaml["id"].as_i64().unwrap(),
      name: String::from(String::from(yaml["name"].as_str().unwrap())),
      path: String::from(String::from(yaml["path"].as_str().unwrap())),
    }
  }

  fn fromYamlVec(yaml: &Vec<Yaml>) -> Option<Vec<ProjectList>> {
    let project_list = &yaml[0]["project-list"];
    let vec_projects = project_list.as_vec().unwrap();

    let mut vec_paths: Vec<ProjectList> = vec![];
    for item in vec_projects {
      let project = ProjectList::fromYaml(item);
      vec_paths.push(project)
    }

    Some(vec_paths)
  }
}

impl Display for ProjectList {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Project [name: {}, path: {}]", self.name, self.path)
  }
}

fn path_of_project_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

struct EvalError(String);
fn eval_path_to_absolute(exp: String) -> Result<String, EvalError> {
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

  Some(vec_paths)
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

  let project_list = if let Some(vec) = ProjectList::fromYamlVec(&configs.clone()) {
    vec
  } else {
    eprintln!("Problem parsing yaml to projec list");
    process::exit(1);
  };

  match &options.command {
    cli::Commands::Check => {
      // projects_file
    }
    cli::Commands::CheckPath {} => {
      let check_ans = Select::new("chose to check: ", project_list).prompt();
      match check_ans {
        Ok(to_check) => println!("chose to be checked: {}", to_check),
        _ => {
          eprintln!("Error in selection, please try again");
          process::exit(1);
        }
      }
    }
    cli::Commands::AddPath {} => {
      let name_ans = Text::new("Project name").prompt();
      match name_ans {
        Ok(name) => {
          println!("Project name selected: {}", name);
          let path_ans = Text::new("Project path").prompt();
          match path_ans {
            Ok(path) => {
              println!("Project path selected: {}", path);
              let id = project_list.last().unwrap().id + 1;
              println!("Generated id: {}", id)
            }
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
    cli::Commands::RemovePath {} => {
      let remove_ans = Select::new("chose to remove: ", project_list).prompt();
      match remove_ans {
        Ok(to_remove) => println!("chose to remove: {}", to_remove),
        _ => {
          eprintln!("Error in selection, please try again");
          process::exit(1);
        }
      }
    }
  }
}
