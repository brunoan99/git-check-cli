use colored::Colorize;

use crate::{
  map_project_to_result, process, CommitTrack, OptionSet, Project, PullChanges, PushChanges,
  RemoteTrack, RepoResult, ResultErrors,
};

pub struct Display(pub String);

impl std::fmt::Display for Display {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<&str> for Display {
  fn from(value: &str) -> Self {
    Self(value.into())
  }
}

impl From<String> for Display {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl Display {
  fn short_from_result(repo_result: &RepoResult) -> Self {
    let repo_name = repo_result.repo.name.as_str().yellow().bold();
    let commit_str = match &repo_result.commits {
      CommitTrack::Empty => format!("[ local -> {} ]", "up to date".to_string().green()),
      CommitTrack::UncommitedChanges {
        commits: _,
        changes,
      } => format!("[ commits: {} ]", changes.to_string().yellow()),
    };
    let remotes_str = match &repo_result.remotes {
      RemoteTrack::NoRemote => String::new(),
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
              text.push_str(&format!(" {}", "up to date".to_string().green()));
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

  fn short_error(name: &str, error: &str) -> Self {
    Self(format!(
      "{} [ {} ]",
      name.yellow().bold(),
      error.red().bold()
    ))
  }

  fn short_from(project: &Project) -> Self {
    match map_project_to_result(project) {
      Ok(repo_result) => Self::short_from_result(&repo_result), // &repo_result,
      Err(ResultErrors::ProjectNotFound) => {
        Self::short_error(&project.name, "Folder was found in specified path")
      }
      Err(ResultErrors::GitNotFound) => {
        Self::short_error(&project.name, "Git Repository was found in specified path")
      }
      Err(ResultErrors::GitFetchingError) => {
        Self::short_error(&project.name, "Fetching error into git remotes")
      }
    }
  }

  fn verbose_from_result(repo_result: &RepoResult) -> Self {
    let repo_name = repo_result.repo.name.as_str().yellow().bold();
    let repo_path = repo_result.repo.path.as_str().bold();
    let commit_str = match &repo_result.commits {
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

    let remotes_str = match &repo_result.remotes {
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
                ));
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
                ));
              }
            }
            if !has_changes {
              text.push_str(&format!(" {} ", "OK".green().bold()));
            }
            text
          })
          .collect();
        string_vec.join("\n")
      }
    };

    Self(format!(
      "{repo_name} - {repo_path}\n{commit_str}\n{remotes_str}\n"
    ))
  }

  fn verbose_error(name: &str, path: &str, error: &str) -> Self {
    Self(format!(
      "{} - {}\n  {}: {}\n",
      name.yellow().bold(),
      process::get_absolute_path(path).bold(),
      "Error".bold(),
      error.red().bold(),
    ))
  }

  fn verbose_from(project: &Project) -> Self {
    match map_project_to_result(project) {
      Ok(repo_result) => Self::verbose_from_result(&repo_result), // &repo_result,
      Err(ResultErrors::ProjectNotFound) => Self::verbose_error(
        &project.name,
        &project.path,
        "Folder was found in specified path",
      ),
      Err(ResultErrors::GitNotFound) => Self::verbose_error(
        &project.name,
        &project.path,
        "Git Repository was found in specified path",
      ),
      Err(ResultErrors::GitFetchingError) => Self::verbose_error(
        &project.name,
        &project.path,
        "Fetching error into git remotes",
      ),
    }
  }

  #[must_use]
  pub fn from(project: &Project, options: &OptionSet) -> Self {
    if options.verbose {
      Self::verbose_from(project)
    } else {
      Self::short_from(project)
    }
  }
}
