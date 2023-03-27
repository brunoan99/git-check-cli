#[path = "cli/cli.rs"]
mod cli;
use inquire::{Confirm, Select, Text};
use linked_hash_map::LinkedHashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{self, Command};
use std::str::from_utf8;
use std::{ffi::OsString, fs};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

use clap::Parser;

#[derive(Debug)]
struct Project {
  name: String,
  path: String,
}

impl Project {
  fn new(name: String, path: String) -> Project {
    Project { name, path }
  }

  fn from_yaml(yaml: &Yaml) -> Project {
    Self {
      name: String::from(String::from(yaml["name"].as_str().unwrap())),
      path: String::from(String::from(yaml["path"].as_str().unwrap())),
    }
  }

  fn to_yaml(&self) -> Yaml {
    let yaml_name = Yaml::String(self.name.clone());
    let yaml_path = Yaml::String(self.path.clone());
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    map.insert(Yaml::String(String::from("name")), yaml_name);
    map.insert(Yaml::String(String::from("path")), yaml_path);
    Yaml::Hash(map)
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
    let projects_list = &yaml[0]["projects-list"];
    let vec_projects = projects_list.as_vec().unwrap();

    let mut vec_paths: Vec<Project> = vec![];
    for item in vec_projects {
      let project = Project::from_yaml(item);
      vec_paths.push(project)
    }

    Some(Self { vec: vec_paths })
  }

  fn to_yaml_vec(&self) -> Yaml {
    let array: Vec<Yaml> = self.vec.iter().map(|item| item.to_yaml()).collect();
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    map.insert(
      Yaml::String(String::from("projects-list")),
      Yaml::Array(array),
    );
    Yaml::Hash(map)
  }

  fn remove_by_name(&self, name: String) -> ProjectList {
    let mut new_projects_list: Vec<Project> = vec![];

    for item in self.vec.iter() {
      if item.name != name {
        new_projects_list.push(item.clone())
      }
    }

    ProjectList {
      vec: new_projects_list,
    }
  }

  fn add_new_project(self, new: Project) -> ProjectList {
    let mut project_list = self.vec.clone();
    project_list.push(new);
    ProjectList { vec: project_list }
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

fn path_of_projects_list_file(home_path: PathBuf) -> Result<String, OsString> {
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
  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Problem finding home directory!");
    process::exit(1);
  });

  let projects_file = path_of_projects_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Problem getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(projects_file.clone()).unwrap_or_else(|err| {
    eprintln!("Problem reading projects list file, error: {}", err);
    process::exit(1);
  });

  let configs = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Problem parsing file, error: {}", err);
    process::exit(1);
  });

  let projects_list = ProjectList::from_yaml_vec(&configs.clone()).unwrap_or_else(|| {
    eprintln!("Problem parsing yaml to projec list");
    process::exit(1);
  });

  let options = cli::Options::parse();

  println!("Starting git-check-cli");

  match &options.command {
    cli::Commands::Check => {
      // projects_file
    }
    cli::Commands::CheckPath {} => {
      let check_ans = Select::new("chose to check: ", projects_list.vec).prompt();
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
              let new_project = Project::new(name, path);
              println!("{}", new_project);
              let confirm_ans = Confirm::new("Confirm to add to projects list")
                .with_default(true)
                .prompt();
              match confirm_ans {
                Ok(true) => {
                  let yaml_projects_list = projects_list.add_new_project(new_project).to_yaml_vec();
                  let mut out_str = String::new();
                  let mut emitter = YamlEmitter::new(&mut out_str);
                  emitter.dump(&yaml_projects_list).unwrap();
                  match fs::write(projects_file, out_str) {
                    Ok(_) => {
                      println!("Project added successfully");
                      process::exit(0);
                    }
                    Err(err) => {
                      eprintln!("failed to sync with file, err: {}", err);
                      process::exit(1)
                    }
                  }
                }
                Ok(false) => {
                  println!("Project not added");
                  process::exit(0);
                }
                Err(_) => error_in_selection(),
              }
            }
            _ => error_in_selection(),
          }
        }
        _ => error_in_selection(),
      }
    }
    cli::Commands::RemovePath {} => {
      let remove_ans = Select::new("chose to remove: ", projects_list.vec.clone()).prompt();
      match remove_ans {
        Ok(to_remove) => {
          let confirm_ans = Confirm::new("Confirm to remove the selected path from projects list")
            .with_default(true)
            .prompt();
          match confirm_ans {
            Ok(true) => {
              let yaml_projects_list = projects_list.remove_by_name(to_remove.name).to_yaml_vec();
              let mut out_str = String::new();
              let mut emitter = YamlEmitter::new(&mut out_str);
              emitter.dump(&yaml_projects_list).unwrap();
              match fs::write(projects_file, out_str) {
                Ok(_) => {
                  println!("Project removed successfully");
                  process::exit(0);
                }
                Err(err) => {
                  eprintln!("failed to sync with file, err: {}", err);
                  process::exit(1)
                }
              }
            }
            Ok(false) => {
              println!("Project not removed");
              process::exit(0);
            }
            Err(_) => error_in_selection(),
          }
        }
        _ => error_in_selection(),
      }
    }
  }
}
