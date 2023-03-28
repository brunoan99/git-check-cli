#[path = "cli/cli.rs"]
mod cli;
use inquire::{Confirm, Select, Text};
use linked_hash_map::LinkedHashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process;
use std::{ffi::OsString, fs};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

use clap::Parser;

#[derive(Debug)]
struct Project {
  name: String,
  path: String,
}

impl Project {
  const fn new(name: String, path: String) -> Self {
    Self { name, path }
  }

  fn from_yaml(yaml: &Yaml) -> Self {
    let name = String::from(yaml["name"].as_str().unwrap_or_else(|| {
      eprintln!("Error getting project name");
      process::exit(1);
    }));
    let path = String::from(yaml["path"].as_str().unwrap_or_else(|| {
      eprintln!("Error getting project path");
      process::exit(1);
    }));
    Self { name, path }
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
struct ProjectList(Vec<Project>);

impl Clone for ProjectList {
  fn clone(&self) -> Self {
    Self(self.0.iter().map(Clone::clone).collect())
  }
}

impl ProjectList {
  const fn new(vec: Vec<Project>) -> Self {
    Self(vec)
  }

  fn from_yaml_vec(yaml: &[Yaml]) -> Option<Self> {
    let projects_list = &yaml[0]["projects-list"];
    projects_list.as_vec().map(|vec_projects| {
      let vec_paths = vec_projects.iter().map(Project::from_yaml).collect();
      Self(vec_paths)
    })
  }

  fn to_yaml_vec(&self) -> Yaml {
    let array: Vec<Yaml> = self.0.iter().map(Project::to_yaml).collect();
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    map.insert(
      Yaml::String(String::from("projects-list")),
      Yaml::Array(array),
    );
    Yaml::Hash(map)
  }

  fn remove_by_name(&self, name: &str) -> Self {
    let new_projects_list = self
      .0
      .iter()
      .cloned()
      .filter(|item| item.name != name)
      .collect::<Vec<Project>>();
    Self::new(new_projects_list)
  }

  fn add_new_project(self, new: Project) -> Self {
    let mut project_list = self.0;
    project_list.push(new);
    Self(project_list)
  }
}

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

fn setup_project_list() -> (String, ProjectList) {
  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Error finding home directory!");
    process::exit(1);
  });

  let projects_path = path_of_projects_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Error getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(projects_path.clone()).unwrap_or_else(|err| {
    eprintln!("Error reading projects list file, error: {err}");
    process::exit(1);
  });

  let configs = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Error parsing file, error: {err}");
    process::exit(1);
  });

  let projects_list = ProjectList::from_yaml_vec(&configs).unwrap_or_else(|| {
    eprintln!("Error parsing yaml to projec list");
    process::exit(1);
  });

  (projects_path, projects_list)
}

fn main() {
  let options = cli::Options::parse();

  let (projects_file_path, projects_list) = setup_project_list();

  println!("Starting git-check-cli");

  match &options.command {
    cli::Commands::Check => {
      // projects_file
    }
    cli::Commands::CheckPath {} => {
      let project = Select::new("chose to check:", projects_list.0)
        .prompt()
        .unwrap_or_else(|err| {
          eprintln!("Error selecting project, err: {err}");
          process::exit(1);
        });
      println!("chose to be checked: {project}");
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
      let new_project = Project::new(name, path);
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
      let yaml_projects_list = projects_list.add_new_project(new_project).to_yaml_vec();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(projects_file_path, out_str) {
        Ok(_) => success_exit("Project added successfully"),
        Err(err) => error_exit(format!("Failed to sync file, getting error: {err}").as_str()),
      }
    }
    cli::Commands::RemovePath {} => {
      let to_remove = Select::new("chose to remove: ", projects_list.0.clone())
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
      let yaml_projects_list = projects_list
        .remove_by_name(to_remove.name.as_str())
        .to_yaml_vec();
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(&yaml_projects_list).unwrap_or_else(|err| {
        eprintln!("Failed to write yaml projects list in file: {err}");
        process::exit(1);
      });
      match fs::write(projects_file_path, out_str) {
        Ok(_) => success_exit("Project removed successfully"),
        Err(err) => {
          eprintln!("failed to sync with file, err: {err}");
          process::exit(1)
        }
      }
    }
  }
}
