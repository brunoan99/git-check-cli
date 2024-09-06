package cmd

import (
	"fmt"
	"os"

	process "github.com/brunoan99/git-check-cli/src"
	"github.com/brunoan99/git-check-cli/src/configs"
	"github.com/brunoan99/git-check-cli/src/utils"
	"github.com/spf13/cobra"
)

var setup configs.Setup

var rootCmd = &cobra.Command{
	Use:   "gitcheckcli",
	Short: "TBD",
	Long:  "TBD",
	PreRun: func(cmd *cobra.Command, args []string) {
		verbose, _ := cmd.Flags().GetBool("verbose")
		source, _ := cmd.Flags().GetString("source")

		path := "$HOME/.config/git-check-cli/config.yaml"
		if source != "" {
			path = source
		}

		var err error
		setup, err = configs.GetSetup(path)
		if err != nil {
			panic(err)
		}
		if verbose && !setup.Configs.Verbose {
			setup.Configs.Verbose = verbose
		}

		err = setup.ResolveProjectsPath()
		if err != nil {
			panic(err)
		}
	},
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("Setup In Run: %+v \n\n", setup)

		errors := []process.DisplayErrorInfo{}

		for i := 0; i < len(setup.Projects); i++ {
			fmt.Println("Start Project: ", setup.Projects[i].Name)
			_, err := process.FullProcess(&setup.Projects[i])
			fmt.Println(err)
			if !utils.IsStructEmpty(err) {
				errors = append(errors, err)
			}
		}

		fmt.Println()
		fmt.Println("---")
		fmt.Println()

		for j := 0; j < len(errors); j++ {
			fmt.Println(errors[j].Message)
		}
	},
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version number of GitCheckCli",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("gitcheckcli version 0.1")
	},
}

func init() {
	rootCmd.AddCommand(versionCmd)
	rootCmd.SuggestionsMinimumDistance = 2

	rootCmd.PersistentFlags().BoolP("verbose", "v", false, "verbose output")
	rootCmd.PersistentFlags().StringP("source", "s", "", "Source directory to read setup file")
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		// fmt.Println(err)
		os.Exit(1)
	}
}
