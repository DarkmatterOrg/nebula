package cmd

import (
	"fmt"
	"github.com/darkmatterorg/nebula/modules"
	"github.com/darkmatterorg/orbit/utils"
	"github.com/spf13/cobra"
)

var thememanagerCmd = &cobra.Command{
	Use:   "themes",
	Short: "Run the theme manager",
}

var setGnomeCmd = &cobra.Command{
	Use:   "set-gnome",
	Short: "Set the GNOME theme",
	Args: func(cmd *cobra.Command, args []string) error {

		if len(args) < 4 {
			utils.Error("You need to specify icons, wallpaper-dark, wallpaper-light, cursor, theme")

			return fmt.Errorf("")
		}

		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		modules.Setgnometheme(args[0], args[1], args[2], args[3], args[4])
	},
}

var setPlasmaCmd = &cobra.Command{
	Use:   "set-plasma",
	Short: "Set the Plasma theme",
	Args: func(cmd *cobra.Command, args []string) error {

		if len(args) < 4 {
			utils.Error("You need to specify theme,icons,cursor and wallpaper")

			return fmt.Errorf("")
		}

		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		modules.Setplasmatheme(args[0], args[1], args[2], args[3])
	},
}

func init() {
	thememanagerCmd.AddCommand(setGnomeCmd)
	thememanagerCmd.AddCommand(setPlasmaCmd)

	setGnomeCmd.SilenceErrors = true
	setPlasmaCmd.SilenceErrors = true
}
