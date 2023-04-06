#![allow(dead_code)]
#![allow(unused_variables)]

use linked_hash_map::LinkedHashMap;
use std::{
  fmt::{Display, Formatter},
  io::Error,
};
use yaml_rust::Yaml;

struct Options {
  configs: Configs,
  project_list: Vec<Project>,
}

impl TryFrom<&Yaml> for Options {
  type Error = String;

  fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl Into<Yaml> for Options {
  fn into(self) -> Yaml {
    todo!()
  }
}

impl Options {
  fn add_project(&mut self, project: Project) -> Result<(), Error> {
    todo!()
  }

  fn remove_project(&mut self, name: &str) -> Result<(), Error> {
    todo!()
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

impl Into<Yaml> for Configs {
  fn into(self) -> Yaml {
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

impl Into<Yaml> for Project {
  fn into(self) -> Yaml {
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
