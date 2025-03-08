package modules

import (
	"fmt"

	"github.com/darkmatterorg/orbit/utils"
	"github.com/fatih/color"
)

func fixUsers() {
	if Debug_mode() {
		boldRed := color.New(color.FgRed, color.Bold).SprintFunc()

		utils.Debug(fmt.Sprintf("Do " + boldRed("NOT") + " run this module on non-Darkmatter systems/images"))
	}
}
