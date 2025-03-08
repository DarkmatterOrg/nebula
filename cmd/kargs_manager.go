package cmd

import (
	"github.com/darkmatterorg/nebula/modules"
	"github.com/darkmatterorg/orbit/utils"
	"github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a kernel argument",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		modules.Add_karg(args[0])
	},
}

var removeCmd = &cobra.Command{
	Use:   "remove",
	Short: "Remove a kernel argument",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		modules.Remove_karg(args[0])
	},
}

var replaceCmd = &cobra.Command{
	Use:   "replace",
	Short: "Replace a kernel argument",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		modules.Replace_karg(args[0], args[1])
	},
}

var kargsCmd = &cobra.Command{
	Use:   "kargs",
	Short: "Manage kernel arguments",
	Run: func(cmd *cobra.Command, args []string) {
		if modules.Debug_mode() {
			utils.Info("You shouldn't run this module on non-Darkmatter systems/images.")
			return
		}
	},
}

func init() {
	kargsCmd.AddCommand(addCmd)
	kargsCmd.AddCommand(removeCmd)
	kargsCmd.AddCommand(replaceCmd)
}
