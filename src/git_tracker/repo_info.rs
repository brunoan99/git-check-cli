use crate::{file_tracker, git_process, process};

#[derive(Clone, Debug)]
pub struct RepoInfo {
  pub path: String,
  pub asbolute_path: String,
  pub name: String,
  pub branch: String,
  pub remotes: Vec<String>,
}

pub enum RepoHidratateErrors {
  ProjectNotFound,
  GitNotFound,
}

impl TryFrom<&file_tracker::Project> for RepoInfo {
  type Error = RepoHidratateErrors;

  fn try_from(value: &file_tracker::Project) -> Result<Self, Self::Error> {
    let path = value.path.as_str();
    let absolute_path = process::eval_to_absolute_path(path);
    let path_to_check = absolute_path.as_str();

    if !git_process::project_exist(path_to_check) {
      return Err(RepoHidratateErrors::ProjectNotFound);
    }
    if !git_process::git_repo_in(path_to_check) {
      return Err(RepoHidratateErrors::GitNotFound);
    }
    Ok(Self {
      path: value.path.clone(),
      asbolute_path: absolute_path.clone(),
      name: value.name.clone(),
      branch: git_process::get_branch(path_to_check),
      remotes: git_process::get_remotes(path_to_check),
    })
  }
}
