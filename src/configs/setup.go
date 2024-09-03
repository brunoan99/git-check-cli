package configs

import (
	"github.com/brunoan99/git-check-cli/src/utils"
	"gopkg.in/yaml.v3"
)

type Setup struct {
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

func GetSetup() (Setup, error) {
	yamlFileContent, err := utils.LoadFile("$HOME/.config/git-check-cli/config.yaml")
	if err != nil {
		return Setup{}, err
	}

	var setup Setup
	err = yaml.Unmarshal(yamlFileContent, &setup)
	if err != nil {
		return Setup{}, err
	}

	return setup, nil
}
