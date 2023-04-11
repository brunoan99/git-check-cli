use std::fmt::{Display, Formatter};

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    map.insert(Self::String("name".into()), yaml_name);
    map.insert(Self::String("path".into()), yaml_path);
    Self::Hash(map)
  }
}

impl Display for Project {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "Project [name: {}, path: {}]", self.name, self.path)
  }
}

impl Project {
  #[must_use]
  pub const fn new(name: String, path: String) -> Self {
    Self { name, path }
  }
}
