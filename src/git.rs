#![allow(dead_code)]
#![allow(unused_variables)]

mod process {
  use std::path::PathBuf;
  use std::process::{Command, Output};
  use std::str;

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

impl Repo {
  const fn new(path: String, name: String, branch: String, remotes: Vec<String>) -> Self {
    Self {
      path,
      name,
      branch,
      remotes,
    }
  }

  pub fn from_path(path: String, name: String) -> Result<Self, RepoHidratateErrors> {
    if !process::project_exist(path.as_str()) {
      return Err(RepoHidratateErrors::ProjectNotFound);
    }
    if !process::git_repo_in(path.as_str()) {
      return Err(RepoHidratateErrors::GitNotFound);
    }
    let branch = process::get_branch(path.as_str());
    let remotes = process::get_remotes(path.as_str());
    Ok(Repo::new(path, name, branch, remotes))
  }
}

struct RepoTrack {
  repo: Repo,
  commits: CommitTrack,
  push: Vec<PushTrack>,
  pull: Vec<PullTrack>,
}

struct Commit {
  hash: String,
  branch: String,
  msg: String,
}

enum CommitTrack {
  Empty,
  UncommitedChanges {
    file_changes: Vec<Commit>,
    changes: u32,
  },
}

enum PushTrack {
  Empty,
  UnpushedChanges {
    remote: String,
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpushed to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

enum PullTrack {
  Empty,
  UnpulledChanges {
    remote: String,
    commits: Vec<Commit>, // Array or a Vec containing each line of log unpulled to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}
