#![allow(dead_code)]
#![allow(unused_variables)]

use linked_hash_map::LinkedHashMap;
use std::fmt::{Display, Formatter};
use yaml_rust::Yaml;

pub struct Tracker {
  configs: Configs,
  projects: Vec<Project>,
}

impl TryFrom<&[Yaml]> for Tracker {
  type Error = String;

  fn try_from(value: &[Yaml]) -> Result<Self, Self::Error> {
    let yaml = &value[0];
    let mut errors = vec![];
    let configs_yaml = &yaml["configs"];
    let Ok(configs) = Configs::try_from(configs_yaml) else {
        return Err("some error occured while loading configs".to_string());
    };
    let projects_yaml = &yaml["projects-list"];
    let projects: Vec<Project> = projects_yaml
      .as_vec()
      .unwrap()
      .iter()
      .map(Project::try_from)
      .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
      .collect();
    if errors.is_empty() {
      Ok(Self { configs, projects })
    } else {
      Err("some error occurred while loading projects".to_string())
    }
  }
}

impl From<Tracker> for Yaml {
  fn from(value: Tracker) -> Self {
    todo!()
  }
}

impl Tracker {
  fn add_project(&mut self, project: Project) {
    self.projects.insert(self.projects.len(), project);
  }

  fn remove_project(&mut self, project: &Project) {
    self
      .projects
      .retain(|p| !(p.name == project.name && p.path == project.path));
  }
}

enum Configs {
  Empty,
  Some { verbose: bool }, // ...
}

impl TryFrom<&Yaml> for Configs {
  type Error = ();

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl From<Configs> for Yaml {
  fn from(value: Configs) -> Self {
    todo!()
  }
}

pub struct Project {
  pub name: String,
  pub path: String,
}

impl TryFrom<&Yaml> for Project {
  type Error = ();

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    let name = match value["name"].as_str() {
      Some(name) => name.to_owned(),
      None => return Err(()),
    };
    let path = match value["path"].as_str() {
      Some(path) => path.to_owned(),
      None => return Err(()),
    };
    Ok(Self { name, path })
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
