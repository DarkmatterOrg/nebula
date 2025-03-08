package modules

import (
	"os/exec"

	"github.com/darkmatterorg/nebula/config"
	"github.com/darkmatterorg/orbit/utils"
)

func Add_karg(karg string) {
	if karg == "" && config.Config.Insults {
		utils.Error("Maybe add the kernel argument idiot?")
	} else if karg == "" {
		utils.Error("Please add the kernel argument")
	}

	add_argument := exec.Command("rpm-ostree", "kargs", "--append=", karg)

	err := add_argument.Run()

	if err != nil {
		utils.Error("Failed to add the kernel argument!")
	}
}

func Remove_karg(karg string) {
	if karg == "" && config.Config.Insults {
		utils.Error("Maybe add the kernel argument idiot?")
	} else if karg == "" {
		utils.Error("Please add the kernel argument")
	}

	remove_argument := exec.Command("rpm-ostree", "kargs", "--delete=", karg)

	err := remove_argument.Run()

	if err != nil {
		utils.Error("Failed to delete the kernel argument!")
	}
}

func Replace_karg(karg string, new_value string) {
	if karg == "" && config.Config.Insults {
		utils.Error("Maybe add the kernel argument idiot?")
	} else if karg == "" {
		utils.Error("Please add the kernel argument")
	}

	replace_argument := exec.Command("rpm-ostree", "kargs", "--replace=", karg, "=", new_value)

	err := replace_argument.Run()

	if err != nil {
		utils.Error("Failed to replace the kernel argument!")
	}
}
