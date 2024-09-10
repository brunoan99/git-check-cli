package process

import (
	"fmt"
	"os/exec"
	"strings"
)

type UncomittedStatus string

// check: https://git-scm.com/docs/git-status
const (
	Modified   UncomittedStatus = "M"
	TypeChange UncomittedStatus = "T"
	Added      UncomittedStatus = "A"
	Deleted    UncomittedStatus = "D"
	Renamed    UncomittedStatus = "R"
	Copied     UncomittedStatus = "C"
	Untracked  UncomittedStatus = "?" // this will only appear duplicated
	Ignored    UncomittedStatus = "!" // this will only appear duplicated
	None       UncomittedStatus = " "
)

type FileLocalDiff struct {
	Staged    UncomittedStatus
	NotStaged UncomittedStatus
	FileName  string
}

type LocalDiffs struct {
	Diffs []FileLocalDiff
}

func stringToUncomittedStatus(s string) (UncomittedStatus, error) {
	if len(s) > 1 {
		return None, fmt.Errorf("cannot parse %s as an uncomitted status", s)
	}

	var uncomittedStatus UncomittedStatus
	switch s {
	case " ":
		uncomittedStatus = None
	case "?":
		uncomittedStatus = Untracked
	case "M":
		uncomittedStatus = Modified
	case "A":
		uncomittedStatus = Added
	case "D":
		uncomittedStatus = Deleted
	case "R":
		uncomittedStatus = Renamed
	case "C":
		uncomittedStatus = Copied
	case "!":
		uncomittedStatus = Ignored
	case "T":
		uncomittedStatus = TypeChange
	default:
		return None, fmt.Errorf("cannot parse %s as an uncomitted status", s)
	}

	return uncomittedStatus, nil
}

func CheckLocalDiff(repo GitRepository) (LocalDiffs, error) {
	localDiffs := LocalDiffs{}

	cmd := exec.Command("/usr/bin/git", "-P", "status", "--porcelain=v1")
	cmd.Dir = repo.Path
	output, err := cmd.Output()
	if err != nil {
		return localDiffs, err
	}

	lines := strings.Split(string(output), "\n")

	for _, line := range lines {
		if line == "" {
			continue
		}

		stagedString := line[:1]
		notStagedString := line[1:2]
		path := line[3:]

		staged, err := stringToUncomittedStatus(stagedString)
		if err != nil {
			return localDiffs, err
		}
		notStaged, err := stringToUncomittedStatus(notStagedString)
		if err != nil {
			return localDiffs, err
		}

		FileLocalDiff := FileLocalDiff{
			FileName:  path,
			Staged:    staged,
			NotStaged: notStaged,
		}

		localDiffs.Diffs = append(localDiffs.Diffs, FileLocalDiff)

	}

	return localDiffs, nil
}
