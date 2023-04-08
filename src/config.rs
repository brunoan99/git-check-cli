#![allow(dead_code)]
#![allow(unused_variables)]

use linked_hash_map::LinkedHashMap;
use std::fmt::{Display, Formatter};
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Config {
  pub options: Vec<Options>,
  pub projects: Vec<Project>,
}

impl TryFrom<Vec<Yaml>> for Config {
  type Error = String;

  fn try_from(value: Vec<Yaml>) -> Result<Self, Self::Error> {
    let yaml = &value[0];
    let mut errors = vec![];
    let configs_yaml = &yaml["configs"];
    let configs: Vec<Options> = configs_yaml
      .as_vec()
      .unwrap()
      .iter()
      .map(Options::from)
      .collect();
    let projects_yaml = &yaml["projects-list"];
    let projects: Vec<Project> = projects_yaml
      .as_vec()
      .unwrap()
      .iter()
      .map(Project::try_from)
      .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
      .collect();
    if errors.is_empty() {
      Ok(Self {
        options: configs,
        projects,
      })
    } else {
      Err("some error occurred while loading projects and configs".to_string())
    }
  }
}

impl From<Config> for Yaml {
  fn from(value: Config) -> Self {
    todo!()
  }
}

impl Config {
  pub fn add_project(&mut self, project: Project) {
    self.projects.insert(self.projects.len(), project);
  }

  pub fn remove_project(&mut self, project: &Project) {
    self.projects.retain(|p| !(p == project));
  }
}

#[derive(Debug)]
pub enum Options {
  BadOption,
  Verbose(bool), // ...
}

impl From<&Yaml> for Options {
  fn from(value: &Yaml) -> Self {
    match value {
      Yaml::Hash(hash) => {
        let Some(Yaml::String(key)) = hash.keys().next() else {
          return Self::BadOption;
        };
        match key.as_str() {
          "verbose" => {
            let Some(Yaml::Boolean(value)) = hash.values().next() else {
              return Self::BadOption;
            };
            Self::Verbose(value.to_owned())
          }
          _ => Self::BadOption,
        }
      }
      Yaml::String(option) => match option.as_str() {
        "verbose" => Self::Verbose(true),
        _ => Self::BadOption,
      },
      _ => Self::BadOption,
    }
  }
}

impl From<Options> for Yaml {
  fn from(value: Options) -> Self {
    todo!()
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
  pub name: String,
  pub path: String,
}

impl TryFrom<&Yaml> for Project {
  type Error = ();

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    let name = if let Yaml::String(name) = &value["name"] {
      name.to_string()
    } else {
      return Err(());
    };
    let path = if let Yaml::String(path) = &value["path"] {
      path.to_string()
    } else {
      return Err(());
    };
    Ok(Self::new(name, path))
  }
}

impl From<Project> for Yaml {
  fn from(val: Project) -> Self {
    let yaml_name = Self::String(val.name);
    let yaml_path = Self::String(val.path);
    let mut map: LinkedHashMap<Self, Self> = LinkedHashMap::new();
    map.insert(Self::String(String::from("name")), yaml_name);
    map.insert(Self::String(String::from("path")), yaml_path);
    Self::Hash(map)
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

impl Project {
  #[must_use]
  pub const fn new(name: String, path: String) -> Self {
    Self { name, path }
  }
}
