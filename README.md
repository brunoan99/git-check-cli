# git-check-cli
cli to help check and update git repos

## Installation

Create a config file in

## Use

The path in projects list always needs to be absolute, relative paths will not work correctly

## Uninstall

## Future Ideas

**NEVER** switch a $HOME for a plain text in the project file

**Unnecessary Evaluation**
Isnt need to eval all of the paths in the project-list, the reference is part of the path, than is better to show the user with the path and later evaluate it to an absolute path to do checks

Example:
  - $HOME/test-dir
  - $TEST_DIR
  - other/path/

  if first and second options evaluates to the same thing (it's possible) than the user can be confused with what option choose. Like:
  - evaluate/to/it
  - evaluate/to/it
  - other/path/

  an user can simply move the reference and the folder of $TEST_DIR and after it $TEST_DIR and $HOME/test-dir will not be the same path, and removing the unwanted but same path will break this in future.

  than display like the first list is far better than the second.

