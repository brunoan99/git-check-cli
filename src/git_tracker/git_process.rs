use std::path::PathBuf;
use std::process::{Command, Output};
use std::str;

// TODO:
// improve errors

fn remove_break_line(output: &mut Output) {
  if output.stdout.ends_with(&[10]) && !output.stdout.is_empty() {
    output.stdout.remove(output.stdout.len() - 1);
  }
}

fn get_stdout_as_string(output: &Output) -> String {
  let str_stdout = str::from_utf8(&output.stdout).unwrap();
  str_stdout.into()
}

fn _get_stderr_as_string(output: &Output) -> String {
  let str_stderr = str::from_utf8(&output.stderr).unwrap();
  str_stderr.into()
}

pub fn project_exist(path: &str) -> bool {
  let mut project_path: String = path.into();
  if !project_path.ends_with('/') {
    project_path.push('/');
  }
  let project_path = PathBuf::from(project_path);
  project_path.exists()
}

pub fn git_repo_in(path: &str) -> bool {
  let mut git_path: String = path.into();
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
    .filter(|s| !str::is_empty(s))
    .collect();
  remotes
}

pub struct GitFetchingError;

pub fn fetch_repo(path: &str) -> Result<(), GitFetchingError> {
  let output = Command::new("/bin/git")
    .args(["fetch", "--all"])
    .current_dir(path)
    .output()
    .unwrap();
  if output.stderr.is_empty() {
    Ok(())
  } else {
    Err(GitFetchingError)
  }
}

pub fn get_uncommited_changes(path: &str) -> Vec<String> {
  let mut output = Command::new("/bin/git")
    .args(["status", "--short"])
    .current_dir(path)
    .output()
    .unwrap();
  remove_break_line(&mut output);
  get_stdout_as_string(&output)
    .split('\n')
    .map(String::from)
    .filter(|s| !str::is_empty(s))
    .collect()
}

pub fn get_unpushed_commits_by_remote(path: &str, remote: &str, branch: &str) -> Vec<String> {
  let refs = format!("{remote}/{branch}..{branch}");
  let mut output = Command::new("/bin/git")
    .args([
      "log",
      &refs,
      "--decorate-refs-exclude=refs/tags",
      "--pretty=%h %s",
    ])
    .current_dir(path)
    .output()
    .unwrap();
  remove_break_line(&mut output);
  get_stdout_as_string(&output)
    .split('\n')
    .map(String::from)
    .filter(|s| !str::is_empty(s))
    .collect()
}

pub fn get_unpulled_commits_by_remote(path: &str, remote: &str, branch: &str) -> Vec<String> {
  let refs = format!("{branch}..{remote}/{branch}");
  let mut output = Command::new("/bin/git")
    .args([
      "log",
      &refs,
      "--decorate-refs-exclude=refs/tags",
      "--pretty=%h %s",
    ])
    .current_dir(path)
    .output()
    .unwrap();
  remove_break_line(&mut output);
  get_stdout_as_string(&output)
    .split('\n')
    .map(String::from)
    .filter(|s| !str::is_empty(s))
    .collect()
}
