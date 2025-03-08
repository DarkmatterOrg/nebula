package cmd

import (
	"github.com/darkmatterorg/nebula/config"
	"github.com/spf13/cobra"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Show the location of the config",
	Run: func(cmd *cobra.Command, args []string) {
		config.FindConfig()
	},
}
