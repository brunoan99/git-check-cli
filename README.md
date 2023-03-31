# git-check-cli
cli to help check and update git repos

## Installation

Create a config file in

## Use

The path in projects list always needs to be absolute, relative paths will not work correctly

## Uninstall

## Future Ideas

**NEVER** switch a $HOME for a plain text in the project file

**"Errors"**

```rust
enum UnupdatedEnum {
  Empty,
  Commits(Uncommited),
  Push(Unpublished),
  Pull(Unpulled),
}
// Err -> [UnupdatedEnum] -> [Empty] means updated
//                        -> [Commits, Empty] means only commits to update
//                        -> [Commits, Push, Empty] means commits and push to update
//                        -> [Commits, Push, Pull, Empty] means commits, push and pull to update
// Think about Empty necessities
// Empty can be just []
// but Empty can be usefull if something will not use a vector
// with Empty in Enum not necessarily a vector have to use it to end an array or something like it
```

**Display idea**
- Something like

```sh
$ git-check check
```

Project **project-name** (Uncommited: n) (Unpublished: n) (Unpushed: n)
Project **project-name2** up to date

---

```sh
$ git-check check --verbose
```
Project **project-name**
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

Project **project-name2** up to date

---

**Check git fetch to sync remote repositories**
