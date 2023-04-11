use super::{process, repo_info::RepoInfo};

pub struct RepoQuery {
  pub repo: RepoInfo,
  pub commits: CommitQuery,
  pub push: Vec<PushQuery>,
  pub pull: Vec<PullQuery>,
}

pub struct CommitQuery {
  pub query: Vec<String>,
}

pub struct PushQuery {
  pub remote: String,
  pub query: Vec<String>,
}

pub struct PullQuery {
  pub remote: String,
  pub query: Vec<String>,
}

impl TryFrom<RepoInfo> for RepoQuery {
  type Error = process::GitFetchingError;

  fn try_from(value: RepoInfo) -> Result<Self, Self::Error> {
    let path = value.path.as_str();
    let branch = value.branch.as_str();
    let remotes: Vec<&str> = value.remotes.iter().map(String::as_str).collect();

    process::fetch_repo(path)?;

    Ok(Self {
      repo: value.clone(),
      commits: CommitQuery {
        query: process::get_uncommited_changes(path),
      },
      push: remotes
        .iter()
        .map(|&remote| PushQuery {
          remote: remote.into(),
          query: process::get_unpushed_commits_by_remote(path, remote, branch),
        })
        .collect(),
      pull: remotes
        .iter()
        .map(|&remote| PullQuery {
          remote: remote.into(),
          query: process::get_unpulled_commits_by_remote(path, remote, branch),
        })
        .collect(),
    })
  }
}
