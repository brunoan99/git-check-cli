use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct OptionSet {
  pub bad_options: Vec<Yaml>,
  pub verbose: bool,
}

impl From<&Yaml> for OptionSet {
  fn from(value: &Yaml) -> Self {
    let mut bad_options = vec![];
    let mut verbose = false;
    for config in value.as_vec().unwrap().iter() {
      match config {
        Yaml::Hash(hash) => {
          let key_opt = hash.keys().next();
          let value_opt = hash.values().next();
          match (key_opt, value_opt) {
            (Some(Yaml::String(key)), Some(Yaml::Boolean(value))) => match key.as_str() {
              "verbose" => {
                verbose = value.to_owned();
              }
              _ => bad_options.push(config.to_owned()),
            },
            _ => bad_options.push(config.to_owned()),
          }
        }
        _ => bad_options.push(config.to_owned()),
      }
    }
    Self {
      bad_options,
      verbose,
    }
  }
}

impl From<OptionSet> for Yaml {
  fn from(value: OptionSet) -> Self {
    let mut config_array: Vec<Yaml> = value.bad_options;

    let mut verbose_map: LinkedHashMap<Self, Self> = LinkedHashMap::new();
    verbose_map.insert(Self::String("verbose".into()), Self::Boolean(value.verbose));
    config_array.push(Self::Hash(verbose_map));

    Self::Array(config_array)
  }
}
