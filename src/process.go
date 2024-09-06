package process

import (
	"fmt"
	"strings"

	setup "github.com/brunoan99/git-check-cli/src/configs"
	"github.com/brunoan99/git-check-cli/src/utils"
)

type DisplaySucessfullInfo struct {
	Local []string
}

type ErrorKing int

const (
	ErrorOnCheckProjectDirectory ErrorKing = iota + 1
	ProjectDirectoryNotExists
	ErrorOnCheckGitRepository
	ProjectIsntGitRepository
)

type DisplayErrorInfo struct {
	Kind    ErrorKing
	Message string
}

func (e *DisplayErrorInfo) Error() string {
	return e.Message
}

func FullProcess(project *setup.Project) (DisplaySucessfullInfo, DisplayErrorInfo) {
	exists, err := CheckIfProjectDirectoryExists(project)
	if err != nil {
		return DisplaySucessfullInfo{}, DisplayErrorInfo{
			Kind:    ErrorOnCheckProjectDirectory,
			Message: err.Error(),
		}
	}
	if !exists {
		return DisplaySucessfullInfo{}, DisplayErrorInfo{
			Kind:    ProjectDirectoryNotExists,
			Message: fmt.Sprintf("cannot find project %s at %s", project.Name, project.Path),
		}
	}

	gitExists, err := CheckIfGitRepositoryExists(project)
	if err != nil {
		return DisplaySucessfullInfo{}, DisplayErrorInfo{
			Kind:    ErrorOnCheckGitRepository,
			Message: err.Error(),
		}
	}
	if !gitExists {
		return DisplaySucessfullInfo{}, DisplayErrorInfo{
			Kind:    ProjectDirectoryNotExists,
			Message: fmt.Sprintf("cannot find git repository at %s", project.Path),
		}
	}

	return DisplaySucessfullInfo{}, DisplayErrorInfo{}
}

func CheckIfProjectDirectoryExists(project *setup.Project) (bool, error) {
	exists, err := utils.CheckDirExists(project.Path)
	if err != nil {
		return false, err
	}
	return exists, nil
}

func CheckIfGitRepositoryExists(project *setup.Project) (bool, error) {
	hasSuffix := strings.HasSuffix(project.Path, "/")
	gitPath := project.Path
	if !hasSuffix {
		gitPath = fmt.Sprint(gitPath, "/")
	}
	gitPath = fmt.Sprint(gitPath, ".git")

	exists, err := utils.CheckDirExists(gitPath)
	if err != nil {
		return false, err
	}
	return exists, nil
}
