# git-check-cli

cli to help check and update git repos

## Installation

Create a config file in

## Use

The path in projects list always needs to be absolute, relative paths will not work correctly

## Uninstall

---

## Idea

---Control different forms of Display
Move Result|Display to outside of git_tracker, maybe some folder that uses both file_tracker and git_tracker, cause it can generate a result based on the file_tracker options

should be able to use cli options in future


---Execution
if no command was provided call a Selec with all options
## Display

**Example**:

```yaml
projects-list:
  - name: project-name1
    path: $HOME/project-dir1
  - name: project-name2
    path: $HOME/project-dir2
  - name: project-name3
    path: $HOME/project-dir3
```

---

**First Example** a short way to display info:

```sh
$ git-check check
```

```
project-name1 [ commits: n ] [ origin -> to-push: n ]
project-name2 [ up to date ]
project-name3 [ commits: n ] [ origin -> to-push: n to-pull: n ] [ origin2 -> to-pull: n ]
```

---

**Second Example** a verbose way to display info:

To get this style use in config file the option verbose or verbose: true, like:

```yaml
config:
  - verbose: true
```

```sh
$ git-check check
```

```
project-name1 - PROJECT/NAME/PATH/1
  Local changes [1]:
    - ?? src/main.rs
  origin changes [4]:
    [To push: 3]
      - ab12cd3 add all
      - ef45gh6 add all
      - ij78kl9 add all
    [To pull: 1]
      - ab12cd3 add all
  origin2 changes [2]:
    [To pull: 2]
      - ef45gh6 add all
      - ij78kl9 add all
project-name2 - PROJECT/NAME/PATH/2
  Local changes: OK
  origin changes: Ok
project-name3 - PROJECT/NAME/PATH/3
  Local changes: OK
  No Remotes
project-name4 - $OTHER/PATH/4
  Error: ...

```
