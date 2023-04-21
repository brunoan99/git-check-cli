use super::options::OptionSet;
use super::Project;

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Tracker {
  pub options: OptionSet,
  pub projects: Vec<Project>,
}

impl TryFrom<Vec<Yaml>> for Tracker {
  type Error = String;

  fn try_from(value: Vec<Yaml>) -> Result<Self, Self::Error> {
    let yaml = &value[0];
    let mut errors = vec![];

    let configs = OptionSet::from(&yaml["configs"]);

    let projects: Vec<Project> = yaml["projects-list"]
      .as_vec()
      .unwrap()
      .iter()
      .map(Project::try_from)
      .filter_map(|res| res.map_err(|err| errors.push(err)).ok())
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
    let mut map: LinkedHashMap<Self, Self> = LinkedHashMap::new();

    map.insert(Self::String("configs".into()), value.options.into());
    map.insert(
      Self::String("projects-list".into()),
      Self::Array(value.projects.iter().cloned().map(Project::into).collect()),
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

  #[must_use]
  pub fn find_project(&self, name: &str) -> Option<&Project> {
    self.projects.iter().find(|p| p.name == name)
  }
}
