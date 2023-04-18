use crate::{GitFetchingError, Project, RepoHidratateErrors};

use super::{
  repo_info::RepoInfo,
  repo_query::{CommitQuery, RemoteQuery, RemoteQueryData, RepoQuery},
};

#[derive(Debug)]
pub struct RepoResult {
  pub repo: RepoInfo,
  pub commits: CommitTrack,
  pub remotes: RemoteTrack,
}

impl From<RepoQuery> for RepoResult {
  fn from(value: RepoQuery) -> Self {
    Self {
      repo: value.repo,
      commits: CommitTrack::from(value.commits),
      remotes: RemoteTrack::from(value.remotes),
    }
  }
}

#[derive(Debug)]
pub enum CommitTrack {
  Empty,
  UncommitedChanges {
    commits: Vec<FileChange>,
    changes: usize,
  },
}

impl From<CommitQuery> for CommitTrack {
  fn from(value: CommitQuery) -> Self {
    if value.query.is_empty() {
      Self::Empty
    } else {
      Self::UncommitedChanges {
        commits: value
          .query
          .iter()
          .map(String::as_str)
          .map(FileChange::from)
          .collect(),
        changes: value.query.len(),
      }
    }
  }
}

#[derive(Debug)]
pub struct FileChange {
  pub path: String,
  pub tracked: bool,
  pub change: String,
}

impl From<&str> for FileChange {
  fn from(value: &str) -> Self {
    let path = value.split_whitespace().last().unwrap_or("").into();
    let tracked = !(value.starts_with(' ') || value.starts_with('?'));
    let change = value.split_whitespace().next().unwrap_or("").into();
    Self {
      path,
      tracked,
      change,
    }
  }
}

#[derive(Debug)]
pub enum RemoteTrack {
  NoRemote,
  Remotes(Vec<RemoteResult>),
}

impl From<RemoteQuery> for RemoteTrack {
  fn from(value: RemoteQuery) -> Self {
    match value {
      RemoteQuery::NoRemote => Self::NoRemote,
      RemoteQuery::Remotes(remote_queries) => Self::Remotes(
        remote_queries
          .iter()
          .cloned()
          .map(RemoteResult::from)
          .collect(),
      ),
    }
  }
}

#[derive(Debug)]
pub struct RemoteResult {
  pub remote: String,
  pub push: PushChanges,
  pub pull: PullChanges,
}

impl From<RemoteQueryData> for RemoteResult {
  fn from(value: RemoteQueryData) -> Self {
    Self {
      remote: value.remote,
      push: PushChanges::from(value.push_query),
      pull: PullChanges::from(value.pull_query),
    }
  }
}

#[derive(Debug)]
pub enum PushChanges {
  Empty,
  Diff {
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpushed to an remote
    changes: usize, // if in future will contain a total or definitely implement a non verbose version
  },
}

impl From<Vec<String>> for PushChanges {
  fn from(value: Vec<String>) -> Self {
    if value.is_empty() {
      Self::Empty
    } else {
      Self::Diff {
        commits: value.iter().map(String::as_str).map(Commit::from).collect(),
        changes: value.len(),
      }
    }
  }
}

#[derive(Debug)]
pub enum PullChanges {
  Empty,
  Diff {
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpushed to an remote
    changes: usize, // if in future will contain a total or definitely implement a non verbose version
  },
}

impl From<Vec<String>> for PullChanges {
  fn from(value: Vec<String>) -> Self {
    if value.is_empty() {
      Self::Empty
    } else {
      Self::Diff {
        commits: value.iter().map(String::as_str).map(Commit::from).collect(),
        changes: value.len(),
      }
    }
  }
}

#[derive(Debug)]
pub struct Commit {
  pub hash: String,
  pub msg: String,
}

impl From<&str> for Commit {
  fn from(value: &str) -> Self {
    Self {
      hash: (&value[..7]).into(),
      msg: (&value[8..]).into(),
    }
  }
}

pub enum ResultErrors {
  ProjectNotFound,
  GitNotFound,
  GitFetchingError,
}

/// # Errors
///
/// Will return `Err`if any process of info or query return error.
pub fn map_project_to_result(project: &Project) -> Result<RepoResult, ResultErrors> {
  match RepoInfo::try_from(project) {
    Ok(repo_info) => match RepoQuery::try_from(repo_info) {
      Ok(repo_query) => Ok(RepoResult::from(repo_query)),
      Err(GitFetchingError) => Err(ResultErrors::GitFetchingError),
    },
    Err(RepoHidratateErrors::ProjectNotFound) => Err(ResultErrors::ProjectNotFound),
    Err(RepoHidratateErrors::GitNotFound) => Err(ResultErrors::GitNotFound),
  }
}
