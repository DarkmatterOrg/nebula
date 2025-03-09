/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"os"

	"fmt"

	"github.com/darkmatterorg/nebula/modules"
	"github.com/darkmatterorg/orbit/utils"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var versionFlag bool

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "nebula",
	Short: "Nebula - flexible system manager",
	// Uncomment the following line if your bare application
	// has an action associated with it:
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 && cmd.Flags().NFlag() == 0 {
			_ = cmd.Help() // Show help message
			return         // Exit after showing help
		}

		if versionFlag {
			stars := modules.PrintNebula()

			fmt.Println(color.MagentaString("Nebula") + ": 1.4" + color.MagentaString(stars))
		}

	},
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if modules.Debug_mode() {
		utils.Info("Running in debug mode, expect breakage.")
	} else if utils.Getimagetype() == "" {
		utils.Error("Nebula can only run on Darkmatter images.")
		return
	}

	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Root().CompletionOptions.DisableDefaultCmd = true

	rootCmd.AddCommand(configCmd)
	rootCmd.AddCommand(flatpaksCmd)
	rootCmd.AddCommand(updateCmd)
	rootCmd.AddCommand(kargsCmd)
	rootCmd.AddCommand(thememanagerCmd)
	rootCmd.Flags().BoolVarP(&versionFlag, "version", "v", false, "Show version")
}
