use crate::{OptionSet, RepoResult};

pub struct DisplayResult(pub String);

impl std::fmt::Display for DisplayResult {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<&str> for DisplayResult {
  fn from(value: &str) -> Self {
    Self(value.into())
  }
}

impl From<String> for DisplayResult {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl DisplayResult {
  fn from(repo_result: &RepoResult, options: &OptionSet) -> Self {
    todo!("implement solution for both verbose and short here, use options to check what should be displayed")
  }
}
