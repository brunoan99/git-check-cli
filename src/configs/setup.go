package configs

import (
	"github.com/brunoan99/git-check-cli/src/utils"
	"gopkg.in/yaml.v3"
)

type Setup struct {
	Path     string
	Configs  Config    `yaml:"configs"`
	Projects []Project `yaml:"projects"`
}

type Config struct {
	Verbose bool `yaml:"verbose"`
}
type Project struct {
	Name string `yaml:"name"`
	Path string `yaml:"path"`
}

func GetSetup(path string) (Setup, error) {
	resolvedPath, err := utils.ResolvePath(path)
	if err != nil {
		return Setup{}, err
	}

	yamlFileContent, err := utils.LoadFile(resolvedPath)
	if err != nil {
		return Setup{}, err
	}

	var setup Setup
	err = yaml.Unmarshal(yamlFileContent, &setup)
	if err != nil {
		return Setup{}, err
	}
	setup.Path = resolvedPath

	return setup, nil
}

func (p *Project) resolveProjectPath() error {
	newPath, err := utils.ResolvePath(p.Path)
	if err != nil {
		return err
	}
	p.Path = newPath
	return nil
}

func (s *Setup) ResolveProjectsPath() error {
	for i := 0; i < len(s.Projects); i++ {
		err := s.Projects[i].resolveProjectPath()
		if err != nil {
			return err
		}
	}
	return nil
}
