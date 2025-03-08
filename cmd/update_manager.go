package cmd

import (
	"github.com/darkmatterorg/nebula/modules"
	"github.com/spf13/cobra"
)

var updateCmd = &cobra.Command{
	Use:   "update-system",
	Short: "Update the whole system",
	Run: func(cmd *cobra.Command, args []string) {
		modules.Update_all()
	},
}
