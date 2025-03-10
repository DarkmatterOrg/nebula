/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"os"

	"fmt"
	"strings"

	"github.com/darkmatterorg/nebula/modules"
	"github.com/darkmatterorg/orbit/utils"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
)

var versionFlag bool

const columnWidth = 30

func bold(input string) string {
	return "\033[1m" + input + "\033[0m" // Apply ANSI bold
}

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

			fmt.Println(color.MagentaString("Nebula") + ": 1.5" + color.MagentaString(stars))
		}

	},
}

func formatWithPadding(s string) string {
	return fmt.Sprintf("%-*s", columnWidth, s)
}

func customizeHelp(cmd *cobra.Command, args []string) {
	// Modify the "Usage" section
	usageLine := cmd.UseLine()
	parts := strings.SplitN(usageLine, " ", 2)
	if len(parts) > 1 {
		usageLine = parts[0] + " " + bold(parts[1]) // Bold only arguments
	} else {
		usageLine = bold(usageLine)
	}

	// Print the custom help message
	fmt.Println(bold("Nebula - flexible system manager"))
	fmt.Println("\n" + bold("Usage") + ":")
	fmt.Println("  " + bold(usageLine))
	fmt.Println("\n" + bold("Available Commands"+":"))

	// Print aligned commands with fixed column width
	for _, c := range cmd.Commands() {
		if !c.Hidden {
			fmt.Printf("  %-*s %s\n", columnWidth, bold(c.Name()), c.Short)
		}
	}

	fmt.Println("\nFlags:")

	// Print aligned flags with fixed column width
	cmd.Flags().VisitAll(func(f *pflag.Flag) {
		flagText := "--" + f.Name
		if f.Shorthand != "" {
			flagText = "-" + f.Shorthand + ", " + flagText
		}
		fmt.Printf("  %-*s %s\n", columnWidth, bold(flagText), f.Usage)
	})

	fmt.Println("\nUse " + bold("nebula [command] --help") + " for more information about a command.")
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

	rootCmd.SetHelpFunc(customizeHelp)

}
