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

    let configs = value.as_vec().map_or_else(
      || {
        bad_options.push(value.clone());
        vec![]
      },
      std::clone::Clone::clone,
    );

    for config in &configs {
      match config {
        Yaml::Hash(hash) => {
          let key_opt = hash.keys().next();
          let value_opt = hash.values().next();
          match (key_opt, value_opt) {
            (Some(Yaml::String(key)), Some(Yaml::Boolean(value))) => match key.as_str() {
              "verbose" => {
                verbose = value.to_owned();
              }
              _ => bad_options.push(config.clone()),
            },
            _ => bad_options.push(config.clone()),
          }
        }
        _ => bad_options.push(config.clone()),
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
    let mut config_array: Vec<Self> = value.bad_options;

    let mut verbose_map: LinkedHashMap<Self, Self> = LinkedHashMap::new();
    verbose_map.insert(Self::String("verbose".into()), Self::Boolean(value.verbose));
    config_array.push(Self::Hash(verbose_map));

    Self::Array(config_array)
  }
}
