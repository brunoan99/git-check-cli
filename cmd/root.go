package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "git check cli",
	Short: "TBD",
	Long:  "TBD",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Tests")
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
