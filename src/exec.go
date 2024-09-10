package exec

import (
	"fmt"

	"github.com/brunoan99/git-check-cli/src/configs"
	"github.com/brunoan99/git-check-cli/src/process"
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

func FullProcess(project *configs.Project) (DisplaySucessfullInfo, DisplayErrorInfo) {
	exists, err := process.CheckIfProjectDirectoryExists(project)
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

	gitExists, err := process.CheckIfGitRepositoryExists(project)
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
