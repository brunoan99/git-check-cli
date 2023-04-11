use super::{
  repo_info::RepoInfo,
  repo_query::{CommitQuery, PullQuery, PushQuery, RepoQuery},
};

pub struct RepoResult {
  pub repo: RepoInfo,
  pub commits: CommitTrack,
  pub push: Vec<PushTrack>,
  pub pull: Vec<PullTrack>,
}

impl From<RepoQuery> for RepoResult {
  fn from(value: RepoQuery) -> Self {
    Self {
      repo: value.repo,
      commits: CommitTrack::from(value.commits),
      push: vec![PushTrack::Empty { remote: "".into() }],
      pull: vec![PullTrack::Empty { remote: "".into() }],
    }
  }
}

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

pub struct FileChange {
  pub path: String,
  pub tracked: bool,
  pub change: String,
}

impl From<&str> for FileChange {
  fn from(value: &str) -> Self {
    let tracked = !(value.starts_with(' ') || value.starts_with('?'));
    let change = value.split_whitespace().next().unwrap().into();
    Self {
      path: value.split_whitespace().last().unwrap().into(),
      tracked,
      change,
    }
  }
}

pub enum PushTrack {
  Empty {
    remote: String,
  },
  UnpushedChanges {
    remote: String,
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpushed to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

impl From<PushQuery> for PushTrack {
  fn from(value: PushQuery) -> Self {
    if value.query.is_empty() {
      Self::Empty {
        remote: value.remote,
      }
    } else {
      Self::UnpushedChanges {
        remote: value.remote,
        commits: value
          .query
          .iter()
          .map(String::as_str)
          .map(Commit::from)
          .collect(),
        changes: value.query.len() as u32,
      }
    }
  }
}

pub enum PullTrack {
  Empty {
    remote: String,
  },
  UnpulledChanges {
    remote: String,
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpulled to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

impl From<PullQuery> for PullTrack {
  fn from(value: PullQuery) -> Self {
    if value.query.is_empty() {
      PullTrack::Empty {
        remote: value.remote,
      }
    } else {
      PullTrack::UnpulledChanges {
        remote: value.remote,
        commits: value
          .query
          .iter()
          .map(String::as_str)
          .map(Commit::from)
          .collect(),
        changes: value.query.len() as u32,
      }
    }
  }
}

pub struct Commit {
  pub hash: String,
  pub msg: String,
}

impl From<&str> for Commit {
  fn from(value: &str) -> Self {
    let (hash, msg) = value.split_once(' ').unwrap();
    Self {
      hash: hash.into(),
      msg: msg.into(),
    }
  }
}
