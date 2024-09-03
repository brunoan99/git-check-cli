package main

import (
	"fmt"

	"github.com/brunoan99/git-check-cli/cmd"
	"github.com/brunoan99/git-check-cli/src/configs"
)

func main() {
	setup, err := configs.GetSetup()
	if err != nil {
		panic(err)
	}

	fmt.Println(setup)

	cmd.Execute()
}
