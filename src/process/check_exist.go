package process

import (
	"fmt"
	"strings"

	"github.com/brunoan99/git-check-cli/src/configs"
	"github.com/brunoan99/git-check-cli/src/utils"
)

func CheckIfProjectDirectoryExists(project *configs.Project) (bool, error) {
	exists, err := utils.CheckDirExists(project.Path)
	if err != nil {
		return false, err
	}
	return exists, nil
}

func CheckIfGitRepositoryExists(project *configs.Project) (bool, error) {
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
