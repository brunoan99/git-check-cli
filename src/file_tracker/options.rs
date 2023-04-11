use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub enum Option {
  BadOption(Yaml),
  Verbose(bool), // ...
}

impl From<&Yaml> for Option {
  fn from(value: &Yaml) -> Self {
    match value {
      Yaml::Hash(hash) => {
        let Some(Yaml::String(key)) = hash.keys().next() else {
          return Self::BadOption(value.clone());
        };
        match key.as_str() {
          "verbose" => {
            let Some(Yaml::Boolean(value)) = hash.values().next() else {
              return Self::BadOption(value.clone());
            };
            Self::Verbose(value.to_owned())
          }
          _ => Self::BadOption(value.clone()),
        }
      }
      _ => Self::BadOption(value.clone()),
    }
  }
}

impl From<Option> for Yaml {
  fn from(value: Option) -> Self {
    match value {
      Option::Verbose(value) => {
        let key = Self::String("verbose".into());
        let value = Self::Boolean(value);
        let mut map: LinkedHashMap<Self, Self> = LinkedHashMap::new();
        map.insert(key, value);
        Self::Hash(map)
      }
      Option::BadOption(value) => value, // needed to not remove an bad option input from the original file when updating projects
    }
  }
}
