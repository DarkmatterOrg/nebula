package modules

import (
	"fmt"
	"github.com/darkmatterorg/orbit/utils"

	"os/exec"
	"strings"
)

func update_distrobox() {
	if utils.IsCmdInstalled("distrobox") {
		utils.Notice("Updating distrobox containers...")

		distrobox_update := exec.Command("distrobox", "upgrade", "-all")

		err := distrobox_update.Run()

		if err != nil {
			utils.Error("Failed to upgrade distrobox containers!")
		}
	} else {
		return
	}
}

func update_flatpaks() {
	if utils.IsCmdInstalled("flatpak") {
		utils.Notice("Updating flatpaks... (It may hang for a few minutes)")

		flatpaks_update := exec.Command("flatpak", "update", "-y")

		err := flatpaks_update.Run()

		if err != nil {
			utils.Error("Failed to update flatpaks!")
		}
	} else {
		return
	}
}

func update_image() {
	if utils.IsCmdInstalled("rpm-ostree") {
		utils.Notice("Updating base image...")

		update_image := exec.Command("bootc", "upgrade")
		err := update_image.Run()

		if err != nil {
			utils.Error("Failed to update base image!")
		}
	} else {
		return
	}
}

func update_python_packages() {
	if utils.IsCmdInstalled("pip3") {
		utils.Notice("Updating python packages...")

		pkg_list := exec.Command("pip3", "freeze", "--user")

		output, err := pkg_list.Output()

		if err != nil {
			utils.Error("Failed to get the installed Python packages!")
			return
		}

		packages := string(output)

		for _, line := range strings.Split(packages, "\n") {
			packageName := strings.Split(line, "=")[0]
			if packageName == "" {
				continue
			}

			upgradeCmd := exec.Command("pip3", "install", "--upgrade", packageName)
			upgradeOutput, err := upgradeCmd.CombinedOutput()

			if err != nil {
				message := fmt.Sprintf("Failed to upgrade packages %s: %v\n", packageName, err)

				utils.Error(message)
				fmt.Println(string(upgradeOutput))
			} else {
				message := fmt.Sprintf("Successfully upgraded %s\n", packageName)

				utils.Done(message)
			}
		}
	}
}

func update_node_packages() {
	node_pkg_managers := [4]string{"pnpm", "yarn", "npm", "bun"}

	for _, pkgManager := range node_pkg_managers {
		if utils.IsCmdInstalled(pkgManager) {
			var cmd *exec.Cmd

			switch pkgManager {
			case "pnpm":
				cmd = exec.Command("pnpm", "update")
			case "yarn":
				cmd = exec.Command("yarn", "upgrade")
			case "npm":
				cmd = exec.Command("npm", "update")
			case "bun":
				cmd = exec.Command("bun", "upgrade")
			default:
				utils.Error("Unsupported package manager")
				return
			}

			err := cmd.Run()

			if err != nil {
				message := fmt.Sprintf("Failed to update node packages using " + pkgManager)
				utils.Error(message)
			}

		}
	}
}

func Update_all() {
	if utils.IsRoot() {
		update_flatpaks()
		update_image()
	} else {
		update_distrobox()
		update_node_packages()
		update_python_packages()
	}
}
