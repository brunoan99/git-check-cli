use crate::{git_tracker, Project, RepoResult};
use colored::Colorize;
use git_tracker::{
  map_project_to_result, CommitTrack, PullChanges, PushChanges, RemoteTrack, ResultErrors,
};

pub struct ShortDisplay(pub String);

impl std::fmt::Display for ShortDisplay {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<&str> for ShortDisplay {
  fn from(value: &str) -> Self {
    Self(value.into())
  }
}

impl From<String> for ShortDisplay {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&RepoResult> for ShortDisplay {
  fn from(value: &RepoResult) -> Self {
    let repo_name = value.repo.name.as_str().yellow().bold();
    let commit_str = match &value.commits {
      CommitTrack::Empty => format!("[ local -> {} ]", "up to date".to_string().green()),
      CommitTrack::UncommitedChanges {
        commits: _,
        changes,
      } => format!("[ commits: {} ]", changes.to_string().yellow()),
    };
    let remotes_str = match &value.remotes {
      RemoteTrack::NoRemote => "".to_owned(),
      RemoteTrack::Remotes(remotes_vec) => {
        let string_vec: Vec<String> = remotes_vec
          .iter()
          .map(|item| {
            let mut text = format!("[ {} ->", item.remote);
            let mut has_changes = false;
            if let PushChanges::Diff {
              commits: _,
              changes,
            } = item.push
            {
              has_changes = true;
              text.push_str(&format!(" to-push: {}", changes.to_string().yellow()));
            }
            if let PullChanges::Diff {
              commits: _,
              changes,
            } = item.pull
            {
              has_changes = true;
              text.push_str(&format!(" to-pull: {}", changes.to_string().yellow()));
            }
            if !has_changes {
              text.push_str(&format!(" {}", "up to date".to_string().green()))
            }
            text.push_str(" ]");
            text
          })
          .collect();
        string_vec.join(" ")
      }
    };
    Self(format!("{repo_name} {commit_str} {remotes_str}"))
  }
}

pub fn map_result_to_short_display(project: &Project) -> ShortDisplay {
  match map_project_to_result(project) {
    Ok(repo_result) => ShortDisplay::from(&repo_result),
    Err(ResultErrors::ProjectNotFound) => format!(
      "{} [ {} ]",
      project.name.yellow().bold(),
      "Folder was found in specified path".red().bold()
    )
    .into(),
    Err(ResultErrors::GitNotFound) => format!(
      "{} [ {} ]",
      project.name.yellow().bold(),
      "Git Repository was found in specified path".red().bold()
    )
    .into(),
    Err(ResultErrors::GitFetchingError) => format!(
      "{} [ {} ]",
      project.name.yellow().bold(),
      "Fetching error into git remotes".red()
    )
    .into(),
  }
}
