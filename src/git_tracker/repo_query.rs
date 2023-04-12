use super::{process, repo_info::RepoInfo};

pub struct RepoQuery {
  pub repo: RepoInfo,
  pub commits: CommitQuery,
  pub push: Pushs,
  pub pull: Pulls,
}

pub struct CommitQuery {
  pub query: Vec<String>,
}

pub enum Pushs {
  NoRemote,
  Vec(Vec<PushQuery>),
}

pub struct PushQuery {
  pub remote: String,
  pub query: Vec<String>,
}

pub enum Pulls {
  NoRemote,
  Vec(Vec<PullQuery>),
}

pub struct PullQuery {
  pub remote: String,
  pub query: Vec<String>,
}

pub struct GitFetchingError;

impl TryFrom<RepoInfo> for RepoQuery {
  type Error = GitFetchingError;

  fn try_from(value: RepoInfo) -> Result<Self, Self::Error> {
    let repo = value.clone();
    let path = value.asbolute_path.as_str();
    let branch = value.branch.as_str();
    let remotes: Vec<&str> = value.remotes.iter().map(String::as_str).collect();

    let commits = CommitQuery {
      query: process::get_uncommited_changes(path),
    };

    if !remotes.is_empty() {
      if let Err(_) = process::fetch_repo(path) {
        return Err(GitFetchingError);
      };
      Ok(Self {
        repo,
        commits,
        push: Pushs::Vec(
          remotes
            .iter()
            .map(|&remote| PushQuery {
              remote: remote.into(),
              query: process::get_unpushed_commits_by_remote(path, remote, branch),
            })
            .collect(),
        ),
        pull: Pulls::Vec(
          remotes
            .iter()
            .map(|&remote| PullQuery {
              remote: remote.into(),
              query: process::get_unpulled_commits_by_remote(path, remote, branch),
            })
            .collect(),
        ),
      })
    } else {
      Ok(Self {
        repo,
        commits,
        push: Pushs::NoRemote,
        pull: Pulls::NoRemote,
      })
    }
  }
}
