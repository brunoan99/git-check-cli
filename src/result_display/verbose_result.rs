use colored::Colorize;

use crate::{
  map_project_to_result, process, CommitTrack, Project, PullChanges, PushChanges, RemoteTrack,
  RepoResult, ResultErrors,
};

pub struct VerboseDisplay(pub String);

impl std::fmt::Display for VerboseDisplay {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<&str> for VerboseDisplay {
  fn from(value: &str) -> Self {
    Self(value.into())
  }
}

impl From<String> for VerboseDisplay {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&RepoResult> for VerboseDisplay {
  fn from(value: &RepoResult) -> Self {
    let repo_name = value.repo.name.as_str().yellow().bold();
    let repo_path = value.repo.path.as_str().bold();
    let commit_str = match &value.commits {
      CommitTrack::Empty => format!("  Local changes: {}", "OK".green().bold()),
      CommitTrack::UncommitedChanges { commits, changes } => {
        let mut text = format!("  Local changes: {}\n", changes.to_string().yellow().bold());
        let commits_str: Vec<String> = commits
          .iter()
          .map(|item| {
            let line_start = "    - ";
            let changed = if item.tracked {
              item.change.green()
            } else {
              item.change.red()
            };
            format!("{line_start}{} {}", changed, item.path)
          })
          .collect();
        text.push_str(commits_str.join("\n").as_str());
        text
      }
    };

    let remotes_str = match &value.remotes {
      RemoteTrack::NoRemote => "  No Remotes".to_owned(),
      RemoteTrack::Remotes(remotes_vec) => {
        let string_vec: Vec<String> = remotes_vec
          .iter()
          .map(|item| {
            let line_title_start = "\n    ";
            let line_commit_start = "\n      - ";
            let mut text = format!("  {} changes:", item.remote);
            let mut has_changes = false;
            if let PushChanges::Diff { commits, changes } = &item.push {
              has_changes = true;
              text.push_str(&format!(
                "{line_title_start}[ To push: {} ]",
                changes.to_string().bold()
              ));
              for commit in commits.iter() {
                text.push_str(&format!(
                  "{line_commit_start}{} {}",
                  commit.hash, commit.msg
                ))
              }
            }
            if let PullChanges::Diff { commits, changes } = &item.pull {
              has_changes = true;
              text.push_str(&format!(
                "{line_title_start}[ To pull {} ]",
                changes.to_string().bold()
              ));
              for commit in commits.iter() {
                text.push_str(&format!(
                  "{line_commit_start}{} {}",
                  commit.hash, commit.msg
                ))
              }
            }
            if !has_changes {
              text.push_str(&format!(" {} ", "OK".green().bold()))
            }
            text
          })
          .collect();
        string_vec.join("\n")
      }
    };

    Self(format!(
      "{} - {}\n{}\n{}\n",
      repo_name, repo_path, commit_str, remotes_str
    ))
  }
}

pub fn map_result_to_verbose_display(project: &Project) -> VerboseDisplay {
  match map_project_to_result(project) {
    Ok(repo_result) => VerboseDisplay::from(&repo_result),
    Err(ResultErrors::ProjectNotFound) => format!(
      "{} - {}\n  {}: {}\n",
      project.name.yellow().bold(),
      process::get_absolute_path(&project.path).bold(),
      "Error".bold(),
      "Fetching error into git remotes".red()
    )
    .into(),
    Err(ResultErrors::GitNotFound) => format!(
      "{} - {}\n  {}: {}\n",
      project.name.yellow().bold(),
      process::get_absolute_path(&project.path).bold(),
      "Error".bold(),
      "Git Repository was found in specified path".red()
    )
    .into(),
    Err(ResultErrors::GitFetchingError) => format!(
      "{} - {}\n  {}: {}\n",
      project.name.yellow().bold(),
      process::get_absolute_path(&project.path).bold(),
      "Error".bold(),
      "Fetching error into git remotes".red()
    )
    .into(),
  }
}
