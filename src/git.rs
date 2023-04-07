#![allow(dead_code)]
#![allow(unused_variables)]

/*
Config -> RepoInfo -> RepoTrack -> RepoResult
        | Path | name ...
                   | the result of the querys
                                | the result of the process of the querys
 */

use crate::config;

mod process {
  use std::path::PathBuf;
  use std::process::{Command, Output};
  use std::str;

  // TODO:
  // improve errors

  fn remove_break_line(output: &mut Output) {
    if output.stdout.is_empty() {
      output.stdout.remove(output.stdout.len() - 1);
    }
  }

  fn get_stdout_as_string(output: &Output) -> String {
    let str_stdout = str::from_utf8(&output.stdout).unwrap();
    String::from(str_stdout)
  }

  fn get_stderr_as_string(output: &Output) -> String {
    let str_stderr = str::from_utf8(&output.stderr).unwrap();
    String::from(str_stderr)
  }

  pub fn project_exist(path: &str) -> bool {
    let mut project_path = String::from(path);
    if !project_path.ends_with('/') {
      project_path.push('/');
    }
    let project_path = PathBuf::from(project_path);
    project_path.exists()
  }

  pub fn git_repo_in(path: &str) -> bool {
    let mut git_path = String::from(path);
    if !git_path.ends_with('/') {
      git_path.push('/');
    }
    git_path.push_str(".git/");
    let git_path = PathBuf::from(git_path);
    git_path.exists()
  }

  pub fn get_branch(path: &str) -> String {
    let mut output = Command::new("/bin/git")
      .args(["branch", "--show-current"])
      .current_dir(path)
      .output()
      .unwrap();
    remove_break_line(&mut output);
    get_stdout_as_string(&output)
  }

  pub fn get_remotes(path: &str) -> Vec<String> {
    let mut output = Command::new("/bin/git")
      .args(["remote"])
      .current_dir(path)
      .output()
      .unwrap();
    remove_break_line(&mut output);
    let remotes: Vec<String> = get_stdout_as_string(&output)
      .split('\n')
      .map(String::from)
      .collect();
    remotes
  }

  pub fn get_uncommited_changes(path: &str) -> Vec<String> {
    let mut output = Command::new("/bin/git")
      .args(["status", "--short"])
      .current_dir(path)
      .output()
      .unwrap();
    remove_break_line(&mut output);
    let uncommited_changes: Vec<String> = get_stdout_as_string(&output)
      .split('\n')
      .map(String::from)
      .collect();
    uncommited_changes
  }

  pub struct GitFetchingError;

  pub fn fetch_repo(path: &str) -> Result<(), GitFetchingError> {
    let output = Command::new("/bin/git")
      .args(["fetch", "--all"])
      .output()
      .unwrap();
    if output.stderr.is_empty() {
      Ok(())
    } else {
      Err(GitFetchingError)
    }
  }

  pub fn get_unpushed_commits_by_remote(path: &str, remote: &str, branch: &str) -> Vec<String> {
    let refs = format!("{remote}/{branch}..{branch}");
    let mut output = Command::new("/bin/git")
      .args(["log", "--oneline", &refs])
      .output()
      .unwrap();
    remove_break_line(&mut output);
    let unpushed_commits: Vec<String> = get_stdout_as_string(&output)
      .split('\n')
      .map(String::from)
      .collect();
    unpushed_commits
  }

  pub fn get_unpulled_commits_by_remote(path: &str, remote: &str, branch: &str) -> Vec<String> {
    let refs = format!("{branch}..{remote}/{branch}");
    let mut output = Command::new("/bin/git")
      .args(["log", "--oneline", &refs])
      .output()
      .unwrap();
    remove_break_line(&mut output);
    let unpulled_commits: Vec<String> = get_stdout_as_string(&output)
      .split('\n')
      .map(String::from)
      .collect();
    unpulled_commits
  }
}

struct Repo {
  path: String,
  name: String,
  branch: String,
  remotes: Vec<String>,
}

enum RepoHidratateErrors {
  ProjectNotFound,
  GitNotFound,
}

impl TryFrom<config::Project> for Repo {
  type Error = RepoHidratateErrors;

  fn try_from(value: config::Project) -> Result<Self, Self::Error> {
    if !process::project_exist(value.path.as_str()) {
      return Err(RepoHidratateErrors::ProjectNotFound);
    }
    if !process::git_repo_in(value.path.as_str()) {
      return Err(RepoHidratateErrors::GitNotFound);
    }
    let branch = process::get_branch(value.path.as_str());
    let remotes = process::get_remotes(value.path.as_str());
    Ok(Self {
      path: value.path,
      name: value.name,
      branch,
      remotes,
    })
  }
}

struct RepoQuery {
  repo: Repo,
  commits: CommitQuery,
  push: Vec<PushQuery>,
  pull: Vec<PullQuery>,
}

struct CommitQuery;

struct PushQuery;

struct PullQuery;

impl From<Repo> for RepoQuery {
  fn from(value: Repo) -> Self {
    todo!()
  }
}

struct RepoResult {
  repo: Repo,
  commits: CommitTrack,
  push: Vec<PushTrack>,
  pull: Vec<PullTrack>,
}

impl From<RepoQuery> for RepoResult {
  fn from(value: RepoQuery) -> Self {
    todo!()
  }
}

struct Commit {
  hash: String,
  branch: String,
  msg: String,
}

impl TryFrom<&str> for Commit {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    todo!()
  }
}

enum CommitTrack {
  Empty,
  UncommitedChanges { commits: Vec<Commit>, changes: u32 },
}

impl From<Vec<String>> for CommitTrack {
  fn from(value: Vec<String>) -> Self {
    todo!()
  }
}

enum PushTrack {
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

enum PullTrack {
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
