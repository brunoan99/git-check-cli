use super::{process, repo_info::RepoInfo};

pub struct RepoQuery {
  pub repo: RepoInfo,
  pub commits: CommitQuery,
  pub remotes: RemoteQuery,
}

pub struct CommitQuery {
  pub query: Vec<String>,
}

pub enum RemoteQuery {
  NoRemote,
  Remotes(Vec<RemoteQueryData>),
}

#[derive(Clone)]
pub struct RemoteQueryData {
  pub remote: String,
  pub push_query: Vec<String>,
  pub pull_query: Vec<String>,
}

pub struct GitFetchingError;

impl TryFrom<RepoInfo> for RepoQuery {
  type Error = GitFetchingError;

  fn try_from(value: RepoInfo) -> Result<Self, Self::Error> {
    let repo = value.clone();
    let path = value.asbolute_path.as_str();
    let branch = value.branch.as_str();
    let remote_names: Vec<&str> = value.remotes.iter().map(String::as_str).collect();

    let commits = CommitQuery {
      query: process::get_uncommited_changes(path),
    };

    if remote_names.is_empty() {
      Ok(Self {
        repo,
        commits,
        remotes: RemoteQuery::NoRemote,
      })
    } else {
      if process::fetch_repo(path).is_err() {
        return Err(GitFetchingError);
      };
      let remotes = RemoteQuery::Remotes(
        remote_names
          .iter()
          .map(|&remote_name| RemoteQueryData {
            remote: remote_name.into(),
            push_query: process::get_unpushed_commits_by_remote(path, remote_name, branch),
            pull_query: process::get_unpulled_commits_by_remote(path, remote_name, branch),
          })
          .collect(),
      );
      Ok(Self {
        repo,
        commits,
        remotes,
      })
    }
  }
}
