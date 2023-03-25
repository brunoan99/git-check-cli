# git-check-cli
cli to help check and update git repos

## Installation

Create a config file in

## Use

The path in projects list always needs to be absolute, relative paths will not work correctly

## Uninstall

## Future Ideas

**NEVER** switch a $HOME for a plain text in the project file

**USE ListOption To Solve this**
remove path should get all paths and names from the project list and give from user suggestions based on
- think about link a path and a id to do the remove and garantee that the path being removed is correct
- paths that evaluate to same thing isn't the same thing
  - example:
    $HOME/test-dir
    $TEST_DIR

    both will be the same if $TEST_DIR evaluates to $HOME/test-dir or if $TEST_DIR evaluates to the same of evaluates of $HOME/test-dir
    than is needed to link the path and the id or something like this
      - check if its possible to pass something like option_text(id, string) and the display property is the string field


after the choose, use the name of the project and check if user confirm the remove
