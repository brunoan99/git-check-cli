# git-check-cli
cli to help check and update git repos

## Installation

Create a config file in

## Use

The path in projects list always needs to be absolute, relative paths will not work correctly

## Uninstall

## Future Ideas

**NEVER** switch a $HOME for a plain text in the project file

**Pull and Push**
Needs to check Pull and Push for each remote.
Push Example:
  project have to remotes
    - origin1
    - origin2
  project have 5 commits
    - commit 1
    - commit 2
    - commit 3 <- origin2
    - commit 4
    - commit 5 <- origin1

In this scenario the remote origin1 is up to date, but has commits to push to origin2.

Push Example:

  project have to remotes
    - origin1
    - origin2
  project have 3 commits
    - commit 1
    - commit 2
    - commit 3

  remote origin1 has 3 commits
    - commit 1
    - commit 2
    - commit 3

  remote origin2 has 5 commits
    - commit 1
    - commit 2
    - commit 3
    - commit 4
    - commit 5

In this scenario the local branch is up to date in relation to origin1, but has commits to push from origin2.

**"Log"**

```rust

type get_repo_info = fn(Project) -> Repo

struct Repo {
  path: String, // maybe this had to become a Path | PathBuff
  name: String,
  branch: String,
  remotes: String[],
}

type checkrepo = fn(Repo) -> RepoTrack

struct RepoTrack {
  repo: Repo,
  commits: CommitTrack,
  push: [PushTrack],
  pull: [PullTrack],
}

enum CommitTrack {
  Empty,
  UncommitedChanges {
    fileChanges: [String], // Array or a Vec containing each line of changes in local repository
    changes: u32,
  },
}

struct Commit{
  hash: String,
  branch: String,
  msg: String,
}

enum PushTrack {
  Empty,
  UnpushedChanges {
    remote: String,
    commits: [String], // Array or a Vec containing each line of log unpushed to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

enum PullTrack {
  Empty,
  UnpulledChanges {
    remote: String,
    commits: [String], // Array or a Vec containing each line of log unpushed to an remote
    changes: u32, // if in future will contain a total or definitely implement a non verbose version
  },
}

// than an repo updated will be described like
let up = RepoTrack{
  commits: CommitTrak::Empty,
  push: PushTrack::Empty,
  pull: PullTrack::Empty,
}
```

**Display idea**
- Think about default, maybe verbose is better and the other as an short option

```sh
$ git-check check
```

Project (project-name) (Uncommited: n) (Unpublished: n) (Unpushed: n)
Project (project-name2) up to date

---

```sh
$ git-check check --verbose
```
Project (project-name)
  - Uncommited Changes:
     M file A
    ?? file B
     D file C
     A file D
  - Unpublished Changes:
    hhhhhhh (HEAD -> branch) commit-msg
    hhhhhhh (HEAD -> branch) commit-msg
  - Unpulled Changes:
    hhhhhhh (remote -> branch) commit-msg

Project (project-name2) up to date

---

**Check git fetch to sync remote repositories**
