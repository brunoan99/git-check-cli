package git

import (
	"os/exec"
	"strings"

	"github.com/brunoan99/git-check-cli/src/utils"
)

type GitRepository struct {
	Path    string
	Branchs []Branch
	Remotes []Remote
}

type Branch struct {
	Name      string
	IsCurrent bool
}

type Remote struct {
	Name         string
	PushAddress  string
	FetchAddress string
}

func GetGitRepository(path string) (GitRepository, error) {
	branchs, err := GetBranchs(path)
	if err != nil {
		return GitRepository{}, err
	}

	remotes, err := GetRemotes(path)
	if err != nil {
		return GitRepository{}, err
	}

	return GitRepository{
		Path:    path,
		Branchs: branchs,
		Remotes: remotes,
	}, nil
}

func GetBranchs(path string) ([]Branch, error) {
	cmd := exec.Command("/usr/bin/git", "-P", "branch")
	cmd.Dir = path
	output, err := cmd.Output()

	if err != nil {
		return []Branch{}, err
	}

	branchStrings := strings.Split(string(output), "\n")
	branchs := []Branch{}

	for _, branchString := range branchStrings {
		cutted, found := strings.CutPrefix(branchString, "*")
		trimed := strings.Trim(cutted, " ")
		if trimed != "" {
			branchs = append(branchs, Branch{
				Name:      trimed,
				IsCurrent: found,
			})
		}
	}

	return branchs, nil
}

func GetRemotes(path string) ([]Remote, error) {
	remotes := []Remote{}

	cmd := exec.Command("/usr/bin/git", "-P", "remote", "-v")
	cmd.Dir = path
	output, err := cmd.Output()

	if err != nil {
		return []Remote{}, err
	}

	remoteStrings := strings.Split(string(output), "\n")

	for _, remoteLine := range remoteStrings {
		if remoteLine == "" {
			continue
		}

		name := utils.GetContentUntil(remoteLine, string([]byte{9}))
		address := utils.GetContentInBetween(remoteLine, string([]byte{9}), " ")
		action := utils.GetContentAfter(remoteLine, " ")
		action = utils.GetContentInBetween(action, "(", ")")

		found := false
		for j, r := range remotes {
			if r.Name == name {
				if action == "push" {
					remotes[j].PushAddress = address
				} else {
					remotes[j].FetchAddress = address
				}
				found = true
				break
			}
		}

		if !found {
			remote := Remote{
				Name:         name,
				PushAddress:  "",
				FetchAddress: "",
			}
			if action == "push" {
				remote.PushAddress = address
			} else {
				remote.FetchAddress = address
			}
			remotes = append(remotes, remote)
		}
	}

	return remotes, nil
}
