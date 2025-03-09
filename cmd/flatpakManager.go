package cmd

import (
	"github.com/darkmatterorg/nebula/modules"
	"github.com/spf13/cobra"
)

var flatpaksCmd = &cobra.Command{
	Use:   "flatpaks",
	Short: "Manage flatpaks",
}

var flatpaksInstallCmd = &cobra.Command{
	Use:   "install",
	Short: "Install flatpaks",
	Run: func(cmd *cobra.Command, args []string) {
		modules.Flatpak_manager()
	},
}

var flatpaksRemoveCmd = &cobra.Command{
	Use:   "remove",
	Short: "Remove flatpaks",
	Run: func(cmd *cobra.Command, args []string) {
		modules.Flatpak_manager_remove()
	},
}

func init() {
	flatpaksCmd.AddCommand(flatpaksInstallCmd)
	flatpaksCmd.AddCommand(flatpaksRemoveCmd)
}
