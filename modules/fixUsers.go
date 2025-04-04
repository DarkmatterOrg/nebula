package modules

import (
	"fmt"

	"github.com/darkmatterorg/nebula/config"
	"github.com/darkmatterorg/orbit/utils"
	"github.com/fatih/color"
)

func fixUsers() {
	if config.Config.Mode == "debug" {
		boldRed := color.New(color.FgRed, color.Bold).SprintFunc()

		utils.Debug(fmt.Sprintf("Do " + boldRed("NOT") + " run this module on non-Darkmatter systems/images"))
	}
}
