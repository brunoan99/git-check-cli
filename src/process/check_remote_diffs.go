package process

import (
	"fmt"
	"os/exec"
	"strings"
)

type CommitDiff struct {
	Hash    string
	Message string
}

type DiffsPerRemote struct {
	RemoteName string
	ToPull     []CommitDiff
	ToPush     []CommitDiff
}

type DiffsPerBranch struct {
	BranchName     string
	DiffsPerRemote []DiffsPerRemote
}

type RemoteDiffs struct {
	DiffsPerBranch []DiffsPerBranch
}

func CheckRemoteDiff(repo GitRepository) (RemoteDiffs, error) {
	cmd := exec.Command("/usr/bin/git", "fetch", "--all")
	cmd.Dir = repo.Path
	_, err := cmd.Output()
	if err != nil {
		return RemoteDiffs{}, err
	}

	remoteDiffs, err := checkRemotesPerBranchs(repo.Branchs, repo.Remotes, repo.Path)
	if err != nil {
		return RemoteDiffs{}, nil
	}

	return remoteDiffs, nil
}

func checkRemotesPerBranchs(branchs []Branch, remotes []Remote, path string) (RemoteDiffs, error) {
	remoteDiffs := RemoteDiffs{}

	for _, branch := range branchs {
		diffsPerBranch, err := checkRemotesPerBranch(branch, remotes, path)
		if err != nil {
			return RemoteDiffs{}, err
		}
		remoteDiffs.DiffsPerBranch = append(remoteDiffs.DiffsPerBranch, diffsPerBranch)
	}

	return remoteDiffs, nil
}

func checkRemotesPerBranch(branch Branch, remotes []Remote, path string) (DiffsPerBranch, error) {
	diffsPerBranch := DiffsPerBranch{
		BranchName: branch.Name,
	}

	for _, remote := range remotes {
		diffsPerRemote, err := checkRemotePerBranch(branch, remote, path)
		if err != nil {
			return DiffsPerBranch{}, nil
		}
		diffsPerBranch.DiffsPerRemote = append(diffsPerBranch.DiffsPerRemote, diffsPerRemote)
	}

	return diffsPerBranch, nil
}

func checkRemotePerBranch(branch Branch, remote Remote, path string) (DiffsPerRemote, error) {
	diffsPerRemote := DiffsPerRemote{
		RemoteName: remote.Name,
	}

	cmd := exec.Command("/usr/bin/git", "-P", "log", "--left-right", "--graph", "--cherry-pick", "--oneline", fmt.Sprintf("%s...%s/%s", branch.Name, remote.Name, branch.Name))
	cmd.Dir = path
	output, err := cmd.Output()
	if err != nil {
		return DiffsPerRemote{}, err
	}

	for _, line := range strings.Split(string(output), "\n") {
		if line == "" {
			continue
		}

		diff := line[0:1]
		remaining := strings.Trim(line[1:], " ")
		split := strings.SplitN(remaining, " ", 2)
		hash := split[0]
		message := split[1]

		commitDiff := CommitDiff{
			Hash:    hash,
			Message: message,
		}

		if diff == "<" {
			diffsPerRemote.ToPush = append(diffsPerRemote.ToPush, commitDiff)
		} else if diff == ">" {
			diffsPerRemote.ToPull = append(diffsPerRemote.ToPush, commitDiff)
		} else {
			return DiffsPerRemote{}, fmt.Errorf("cannot parse %s on remote check", diff)
		}
	}
	return diffsPerRemote, nil
}
