use super::process;
use crate::tracker;

#[derive(Clone)]
pub struct RepoInfo {
  pub path: String,
  pub name: String,
  pub branch: String,
  pub remotes: Vec<String>,
}

pub enum RepoHidratateErrors {
  ProjectNotFound,
  GitNotFound,
}

impl TryFrom<tracker::Project> for RepoInfo {
  type Error = RepoHidratateErrors;

  fn try_from(value: tracker::Project) -> Result<Self, Self::Error> {
    if !process::project_exist(value.path.as_str()) {
      return Err(RepoHidratateErrors::ProjectNotFound);
    }
    if !process::git_repo_in(value.path.as_str()) {
      return Err(RepoHidratateErrors::GitNotFound);
    }
    let branch = process::get_branch(value.path.as_str());
    let remotes = process::get_remotes(value.path.as_str());
    Ok(Self {
      path: value.path,
      name: value.name,
      branch,
      remotes,
    })
  }
}
