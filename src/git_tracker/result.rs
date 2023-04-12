use crate::{Project, RepoHidratateErrors, RepoInfo, RepoQuery, RepoResult};

pub struct GitTrackerResult(pub String);

impl From<&str> for GitTrackerResult {
  fn from(value: &str) -> Self {
    GitTrackerResult(value.into())
  }
}

pub fn map_project_to_result(project: &Project) -> GitTrackerResult {
  match RepoInfo::try_from(project) {
    Ok(repo_info) => match RepoQuery::try_from(repo_info) {
      Ok(repo_query) => {
        let repo_result = RepoResult::from(repo_query);
        println!("{:#?}", repo_result);
        GitTrackerResult::from(&repo_result)
      }
      Err(_) => "Fetching error into git remotes".into(),
    },
    Err(RepoHidratateErrors::PathNotEvaluated) => "Path cannot be evaluated".into(),
    Err(RepoHidratateErrors::ProjectNotFound) => "No folder was found in specified path".into(),
    Err(RepoHidratateErrors::GitNotFound) => "No git repository was found in specified path".into(),
  }
}
