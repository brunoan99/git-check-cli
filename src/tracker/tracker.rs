use super::Option;
use super::Project;

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Tracker {
  pub options: Vec<Option>,
  pub projects: Vec<Project>,
}

impl TryFrom<Vec<Yaml>> for Tracker {
  type Error = String;

  fn try_from(value: Vec<Yaml>) -> Result<Self, Self::Error> {
    let yaml = &value[0];
    let mut errors = vec![];
    let configs_yaml = &yaml["configs"];
    let configs: Vec<Option> = configs_yaml
      .as_vec()
      .unwrap()
      .iter()
      .map(Option::from)
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

impl From<Tracker> for Yaml {
  fn from(value: Tracker) -> Self {
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();

    map.insert(
      Self::String("configs".into()),
      Yaml::Array(value.options.iter().cloned().map(Option::into).collect()),
    );
    map.insert(
      Self::String("projects-list".into()),
      Yaml::Array(value.projects.iter().cloned().map(Project::into).collect()),
    );

    Self::Hash(map)
  }
}

impl Tracker {
  pub fn add_project(&mut self, project: Project) {
    self.projects.insert(self.projects.len(), project);
  }

  pub fn remove_project(&mut self, project: &Project) {
    self.projects.retain(|p| !(p == project));
  }
}
