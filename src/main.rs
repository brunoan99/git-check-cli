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
struct Project {
  id: i64,
  name: String,
  path: String,
}

impl Project {
  fn from_yaml(yaml: &Yaml) -> Project {
    Self {
      id: yaml["id"].as_i64().unwrap(),
      name: String::from(String::from(yaml["name"].as_str().unwrap())),
      path: String::from(String::from(yaml["path"].as_str().unwrap())),
    }
  }
}

impl Display for Project {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Project [name: {}, path: {}]", self.name, self.path)
  }
}

impl Clone for Project {
  fn clone(&self) -> Self {
    Self {
      id: self.id.clone(),
      name: self.name.clone(),
      path: self.path.clone(),
    }
  }
}

#[derive(Debug)]
struct ProjectList {
  vec: Vec<Project>,
}

impl ProjectList {
  fn from_yaml_vec(yaml: &Vec<Yaml>) -> Option<ProjectList> {
    let project_list = &yaml[0]["project-list"];
    let vec_projects = project_list.as_vec().unwrap();

    let mut vec_paths: Vec<Project> = vec![];
    for item in vec_projects {
      let project = Project::from_yaml(item);
      vec_paths.push(project)
    }

    Some(Self { vec: vec_paths })
  }

  fn filter_by_id(&self, id: i64) -> ProjectList {
    let mut new_project_list: Vec<Project> = vec![];

    for item in self.vec.iter() {
      if item.id != id {
        new_project_list.push(item.clone())
      }
    }

    ProjectList {
      vec: new_project_list,
    }
  }
}

impl Clone for ProjectList {
  fn clone(&self) -> Self {
    Self {
      vec: self.vec.clone(),
    }
  }
}

fn error_in_selection() {
  eprintln!("Error in selection, please try again");
  process::exit(1);
}

fn path_of_project_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

struct _EvalError(String);
fn _eval_path_to_absolute(exp: String) -> Result<String, _EvalError> {
  let path_to_eval = ["/bin/echo", &exp].join(" ");
  let mut cmd = Command::new("sh");
  cmd.args(["-c", &path_to_eval]);
  let mut output = match cmd.output() {
    Ok(out) => out,
    Err(err) => return Err(_EvalError(String::from(err.to_string()))),
  };
  output
    .stdout
    .resize(output.stdout.len() - 1, output.stdout[0]);
  Ok(String::from(from_utf8(&output.stdout).unwrap_or_else(
    |err| {
      eprintln!("failed to parse evaluated path: {}", err);
      process::exit(1);
    },
  )))
}

fn main() {
  println!("Starting git-check-cli");

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

  let project_list = ProjectList::from_yaml_vec(&configs.clone()).unwrap_or_else(|| {
    eprintln!("Problem parsing yaml to projec list");
    process::exit(1);
  });

  let options = cli::Options::parse();

  match &options.command {
    cli::Commands::Check => {
      // projects_file
    }
    cli::Commands::CheckPath {} => {
      let check_ans = Select::new("chose to check: ", project_list.vec).prompt();
      match check_ans {
        Ok(to_check) => println!("chose to be checked: {}", to_check),
        _ => error_in_selection(),
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
              let id = project_list.vec.last().unwrap().id + 1;
              println!("Generated id: {}", id)
            }
            _ => error_in_selection(),
          }
        }
        _ => error_in_selection(),
      }
    }
    cli::Commands::RemovePath {} => {
      let remove_ans = Select::new("chose to remove: ", project_list.vec.clone()).prompt();
      match remove_ans {
        Ok(to_remove) => {
          println!("chose to remove: {} | id: {}", to_remove, to_remove.id);
          let new_project_list = project_list.filter_by_id(to_remove.id);
          println!("{:#?}", new_project_list)
        }
        _ => error_in_selection(),
      }
    }
  }
}
