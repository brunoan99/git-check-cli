use super::{
  repo_info::RepoInfo,
  repo_query::{CommitQuery, Pulls, Pushs, RepoQuery},
  result::GitTrackerResult,
};

#[derive(Debug)]
pub struct RepoResult {
  pub repo: RepoInfo,
  pub commits: CommitTrack,
  pub push: PushTrack,
  pub pull: PullTrack,
}

impl From<&RepoResult> for GitTrackerResult {
  fn from(value: &RepoResult) -> Self {
    value.repo.name.as_str().into()
  }
}

impl From<RepoQuery> for RepoResult {
  fn from(value: RepoQuery) -> Self {
    Self {
      repo: value.repo,
      commits: CommitTrack::from(value.commits),
      push: PushTrack::from(value.push),
      pull: PullTrack::from(value.pull),
    }
  }
}

#[derive(Debug)]
pub enum CommitTrack {
  Empty,
  UncommitedChanges {
    commits: Vec<FileChange>,
    changes: u32,
  },
}

impl From<CommitQuery> for CommitTrack {
  fn from(value: CommitQuery) -> Self {
    if value.query.is_empty() {
      CommitTrack::Empty
    } else {
      println!("query: {:#?}", value.query);
      CommitTrack::UncommitedChanges {
        commits: value
          .query
          .iter()
          .map(String::as_str)
          .map(FileChange::from)
          .collect(),
        changes: value.query.len() as u32,
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
    println!("file-change: {}", value);
    let tracked = !(value.starts_with(' ') || value.starts_with('?'));
    let change = value.split_whitespace().next().unwrap().into();
    Self {
      path: value.split_whitespace().last().unwrap().into(),
      tracked,
      change,
    }
  }
}

#[derive(Debug)]
pub enum PushTrack {
  NoRemote,
  Track(Vec<PushChanges>),
}

#[derive(Debug)]
pub enum PushChanges {
  Empty {
    remote: String,
  },
  Diff {
    remote: String,
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpushed to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

impl From<Pushs> for PushTrack {
  fn from(value: Pushs) -> Self {
    match value {
      Pushs::NoRemote => PushTrack::NoRemote,
      Pushs::Vec(push_queries) => PushTrack::Track(
        push_queries
          .iter()
          .map(|item| {
            if item.query.is_empty() {
              PushChanges::Empty {
                remote: item.remote.clone(),
              }
            } else {
              PushChanges::Diff {
                remote: item.remote.clone(),
                commits: item
                  .query
                  .iter()
                  .map(String::as_str)
                  .map(Commit::from)
                  .collect(),
                changes: item.query.len() as u32,
              }
            }
          })
          .collect(),
      ),
    }
  }
}

#[derive(Debug)]
pub enum PullTrack {
  NoRemote,
  Track(Vec<PullChanges>),
}

#[derive(Debug)]
pub enum PullChanges {
  Empty {
    remote: String,
  },
  Diff {
    remote: String,
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpushed to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

impl From<Pulls> for PullTrack {
  fn from(value: Pulls) -> Self {
    match value {
      Pulls::NoRemote => PullTrack::NoRemote,
      Pulls::Vec(pull_queries) => PullTrack::Track(
        pull_queries
          .iter()
          .map(|item| {
            if item.query.is_empty() {
              PullChanges::Empty {
                remote: item.remote.clone(),
              }
            } else {
              PullChanges::Diff {
                remote: item.remote.clone(),
                commits: item
                  .query
                  .iter()
                  .map(String::as_str)
                  .map(Commit::from)
                  .collect(),
                changes: item.query.len() as u32,
              }
            }
          })
          .collect(),
      ),
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
