use super::{repo_info::RepoInfo, repo_query::RepoQuery};

pub struct RepoResult {
  pub repo: RepoInfo,
  pub commits: CommitTrack,
  pub push: Vec<PushTrack>,
  pub pull: Vec<PullTrack>,
}

impl From<RepoQuery> for RepoResult {
  fn from(value: RepoQuery) -> Self {
    todo!()
  }
}

pub struct Commit {
  pub hash: String,
  pub branch: String,
  pub msg: String,
}

impl TryFrom<&str> for Commit {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    todo!()
  }
}

pub enum CommitTrack {
  Empty,
  UncommitedChanges { commits: Vec<Commit>, changes: u32 },
}

impl From<Vec<String>> for CommitTrack {
  fn from(value: Vec<String>) -> Self {
    todo!()
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

impl PushTrack {
  fn from(query: Vec<String>, remote: &str) -> Self {
    todo!()
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

impl PullTrack {
  fn from(query: Vec<String>, remote: &str) -> Self {
    todo!()
  }
}
