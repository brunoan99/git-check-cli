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
    if output.stdout.len() > 0 {
      output.stdout.remove(output.stdout.len() - 1);
    }
  }

  fn get_stdout_as_string(output: &Output) -> String {
    let str_stdout = str::from_utf8(&output.stdout).unwrap();
    String::from(str_stdout)
  }

  pub fn project_exist(path: &str) -> bool {
    let mut project_path = String::from(path);
    if !project_path.ends_with("/") {
      project_path.push_str("/")
    }
    let project_path = PathBuf::from(project_path);
    project_path.exists()
  }

  pub fn git_repo_in(path: &str) -> bool {
    let mut git_path = String::from(path);
    if !git_path.ends_with("/") {
      git_path.push_str("/")
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
    let branch = get_stdout_as_string(&mut output);
    branch
  }

  pub fn get_remotes(path: &str) -> Vec<String> {
    let mut output = Command::new("/bin/git")
      .args(["remote"])
      .current_dir(path)
      .output()
      .unwrap();
    remove_break_line(&mut output);
    let remotes_string = str::from_utf8(&output.stdout).expect("Failed to parse output");
    if remotes_string.contains("\n") {
      let remotes: Vec<String> = remotes_string
        .split("\n")
        .map(|item| String::from(item))
        .collect();
      remotes
    } else {
      vec![String::from(remotes_string)]
    }
  }

  pub fn get_uncommited_changes(path: &str) -> Vec<String> {
    todo!()
  }

  pub fn get_unpushed_commits_by_remote(path: &str, remote: &str) -> Vec<String> {
    todo!()
  }

  pub fn get_upulled_commits_by_remote(path: &str, remote: &str) -> Vec<String> {
    todo!()
  }

  #[cfg(test)]
  mod tests {
    // TODO:
    // Improve testability not using a folder in my setup,
    // Certificate to create and delete a directory to test it
    use super::{get_branch, get_remotes, git_repo_in, project_exist};

    #[test]
    fn test_get_branch() {
      let result = get_branch("/home/snape/test-dir5");
      assert_eq!(result, String::from("another-name"));
    }

    #[test]
    fn test_get_remotes() {
      let result = get_remotes("/home/snape/test-dir3");
      assert_eq!(
        result,
        vec![String::from("origin"), String::from("origin2")]
      );
      let result = get_remotes("/home/snape/test-dir5");
      assert_eq!(result, vec![String::from("")]);
    }

    #[test]
    fn test_git_repo_in() {
      let result = git_repo_in("/home/snape/test-dir4");
      assert_eq!(result, true);
      let result = git_repo_in("/home/snape/test-dir6");
      assert_eq!(result, false);
    }

    #[test]
    fn test_project_exists() {
      let result = project_exist("/home/snape/test-dir4");
      assert_eq!(result, true);
      let result = project_exist("/home/snape/test-dir6");
      assert_eq!(result, false)
    }
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
    Ok(Repo {
      path: value.path,
      name: value.name,
      branch,
      remotes,
    })
  }
}

struct RepoQuery {
  repo: Repo,
  commits: Vec<String>,
  push: Vec<String>,
  pull: Vec<String>,
}

impl TryFrom<Repo> for RepoQuery {
  type Error = ();

  fn try_from(value: Repo) -> Result<Self, Self::Error> {
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
  Empty,
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
