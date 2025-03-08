package modules

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/darkmatterorg/orbit/utils"
)

var image_type = utils.Getimagetype()

func check_fedora_remote() bool {
	cmd := exec.Command("flatpak", "remotes")
	var out bytes.Buffer
	cmd.Stdout = &out
	err := cmd.Run()

	if err != nil {
		utils.Error("Failed to run command.")
	}

	output := out.String()
	if strings.Contains(output, "fedora") {
		return true
	}

	return false
}

func remove_fedora_remote() {
	if check_fedora_remote() {
		return
	}

	cmd := exec.Command("flatpak", "remote-delete", "--force", "fedora")
	err := cmd.Run()

	if err != nil {
		utils.Error("Failed to remove the fedora remote.")
	}
}

func remove_flatpaks() {
	var flatpaksToRemove []string

	switch {
	case strings.Contains(image_type, "aster"):
		flatpaksToRemove = append(flatpaksToRemove, filepath.Join("/usr/share/horizon/packages/aster/aster_flatpak_remove"))
	case strings.Contains(image_type, "arcturus"), strings.Contains(image_type, "nova"):
		return
	default:
		utils.Notice("Not implemented yet.")
		return
	}

	for _, flatpakList := range flatpaksToRemove {
		contents, err := os.ReadFile(flatpakList)
		if err != nil {
			var error_message = fmt.Sprintf("Full Error: %v\n", err)

			utils.Error("Failed to read the flatpaks list.\n" + error_message)
			continue
		}

		lines := strings.Split(string(contents), "\n")
		for _, line := range lines {
			line = strings.TrimSpace(line)
			if line != "" {
				cmd := exec.Command("flatpak", "remove", "--system", "-y", line)
				_, err := cmd.CombinedOutput()
				if err != nil {
					utils.Error("Failed to remove flatpaks from list.")
				} else {
					utils.Done("Removed all flatpaks from list.")
				}
			}
		}
	}
}

func run_flatpak_override(args []string) {
	cmd := exec.Command("flatpak", append([]string{"override"}, args...)...)
	err := cmd.Run()

	if err != nil {
		var message = fmt.Sprintf("flatpak override " + strings.Join(args, " "))
		utils.Done(message)
	}
}

func fix_theming() {
	run_flatpak_override([]string{
		"--filesystem=xdg-config/gtk-4.0:ro",
		"--filesystem=xdg-config/gtk-3.0:ro",
		"--filesystem=xdg-data/icons:ro",
	})

	// XInput for Firefox
	run_flatpak_override([]string{"--system", "--env=MOZ_USE_XINPUT2=1", "org.mozilla.firefox"})

	// Fix printing on LibreOffice
	run_flatpak_override([]string{
		"--system",
		"--socket=cups",
		"--socket=session-bus",
		"org.libreoffice.LibreOffice",
	})

	// Allow MangoHUD config for Flatpaks
	run_flatpak_override([]string{
		"--filesystem=xdg-config/MangoHud:ro",
		"--filesystem=xdg-config/vkBasalt:ro",
	})

	// Fix permissions for XIV Launcher
	run_flatpak_override([]string{"--device=dri", "dev.goats.xivlauncher"})

	// Fix permissions for Protontricks
	run_flatpak_override([]string{
		"--filesystem=~/.local/share/Steam",
		"--filesystem=/var/mnt",
		"--filesystem=/run/media",
		"com.github.Matoking.protontricks",
	})

	// Nvidia-specific logic using getImageType()

	if strings.Contains(image_type, "nvidia") {
		lshwOutput, err := exec.Command("lshw", "-C", "display").Output()
		if err != nil {
			utils.Error("Failed to run lshw")
			return
		}

		lshwStdout := string(lshwOutput)
		nvidiaPresent := strings.Contains(lshwStdout, "vendor: NVIDIA Corporation")
		displayCount := strings.Count(lshwStdout, "-display")

		if nvidiaPresent && displayCount <= 1 {
			// Apply Nvidia-specific flatpak override
			run_flatpak_override([]string{
				"--system",
				"--filesystem=host-os",
				"--env=LIBVA_DRIVER_NAME=nvidia",
				"--env=LIBVA_DRIVERS_PATH=/run/host/usr/lib64/dri",
				"--env=LIBVA_MESSAGING_LEVEL=1",
				"--env=MOZ_DISABLE_RDD_SANDBOX=1",
				"--env=NVD_BACKEND=direct",
				"org.mozilla.firefox",
			})
		} else {
			// Undo Nvidia-specific overrides
			run_flatpak_override([]string{
				"--system",
				"--nofilesystem=host-os",
				"--unset-env=LIBVA_DRIVER_NAME",
				"--unset-env=LIBVA_DRIVERS_PATH",
				"--unset-env=LIBVA_MESSAGING_LEVEL",
				"--unset-env=MOZ_DISABLE_RDD_SANDBOX",
				"--unset-env=NVD_BACKEND",
				"org.mozilla.firefox",
			})
		}
	} else {
		// Non-NVIDIA image: Ensure overrides are cleared
		run_flatpak_override([]string{
			"--system",
			"--nofilesystem=host-os",
			"--unset-env=LIBVA_DRIVER_NAME",
			"--unset-env=LIBVA_DRIVERS_PATH",
			"--unset-env=LIBVA_MESSAGING_LEVEL",
			"--unset-env=MOZ_DISABLE_RDD_SANDBOX",
			"--unset-env=NVD_BACKEND",
			"org.mozilla.firefox",
		})
	}
}

func enable_flathub() {
	// Add flathub
	add_flathub := exec.Command("flatpak", "remote-add", "--if-not-exists", "--system", "flathub", "https://dl.flathub.org/repo/flathub.flatpakrepo")
	add_flathub_err := add_flathub.Run()

	if add_flathub_err != nil {
		utils.Error("Failed to add flathub")
	}

	// Enable flathub
	enable_flathub := exec.Command("flatpak", "remote-modify", "--system", "--enable", "flathub")
	enable_flathub_err := enable_flathub.Run()

	if enable_flathub_err != nil {
		utils.Error("Failed to enable flathub.")
	}

}

func install_flatpaks() {
	var flatpaks_to_install []string

	switch {
	case strings.Contains(image_type, "aster"):
		flatpaks_to_install = append(flatpaks_to_install, filepath.Join("/usr/share/horizon/packages/aster/aster_flatpaks"))
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/horizon/packages/shared_flatpaks")

	case strings.Contains(image_type, "arcturus"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/horizon/packages/arcturus_flatpaks")
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/horizon/packages/shared_flatpaks")

	case strings.Contains(image_type, "umbra"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/umbra/packages/flatpaks")

	case strings.Contains(image_type, "nova_gnome"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/gnome/install_flatpaks")

	case strings.Contains(image_type, "nova_gnome_dx"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/gnome/install_flatpaks_dx")

	case strings.Contains(image_type, "nova_gnome_gaming"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/gnome/install_flatpaks_gaming")

	case strings.Contains(image_type, "supernova_gnome"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/gnome/install_flatpaks")
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/gnome/install_flatpaks_dx")
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/gnome/install_flatpaks_gaming")

	case strings.Contains(image_type, "nova_plasma"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/plasma/install_flatpaks")

	case strings.Contains(image_type, "nova_plasma_dx"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/plasma/install_flatpaks_dx")

	case strings.Contains(image_type, "nova_plasma_gaming"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/plasma/install_flatpaks_gaming")

	case strings.Contains(image_type, "supernova_plasma"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/plasma/install_flatpaks")
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/plasma/install_flatpaks_dx")
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/plasma/install_flatpaks_gaming")

	case strings.Contains(image_type, "nova_cosmic_dx"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/cosmic/install_flatpaks_dx")

	case strings.Contains(image_type, "supernova_cosmic"):
		flatpaks_to_install = append(flatpaks_to_install, "/usr/share/nova/packages/cosmic/install_flatpaks_dx")
	default:
		utils.Notice("Not implemented yet.")
		return
	}

	for _, flatpakList := range flatpaks_to_install {
		contents, err := os.ReadFile(flatpakList)
		if err != nil {
			var error_message = fmt.Sprintf("Full Error: %v\n", err)

			utils.Error("Failed to read the flatpaks list.\n" + error_message)
			continue
		}

		lines := strings.Split(string(contents), "\n")
		for _, line := range lines {
			line = strings.TrimSpace(line)
			if line != "" {
				cmd := exec.Command("flatpak", "remove", "--system", "-y", line)
				_, err := cmd.CombinedOutput()
				if err != nil {
					utils.Error("Failed to install flatpaks from list.")
				} else {
					utils.Done("Installed all flatpaks from list.")
				}
			}
		}
	}
}

func Flatpak_manager_remove() {
	remove_flatpaks()

	switch {
	case strings.Contains(image_type, "nova"):
		utils.Disable_systemd_service("nova-flatpak-manager")
	case strings.Contains(image_type, "arcturus"), strings.Contains(image_type, "aster"):
		utils.Disable_systemd_service("horizon-flatpak-manager")
	case strings.Contains(image_type, "umbra"):
		utils.Disable_systemd_service("umbra-flatpak-manager")
	default:
		utils.Error("Image not supported.")
	}
}

func Flatpak_manager() {
	remove_fedora_remote()
	enable_flathub()
	install_flatpaks()
	fix_theming()

	switch {
	case strings.Contains(image_type, "nova"):
		utils.Disable_systemd_service("nova-flatpak-manager")
	case strings.Contains(image_type, "arcturus"), strings.Contains(image_type, "aster"):
		utils.Disable_systemd_service("horizon-flatpak-manager")
	case strings.Contains(image_type, "umbra"):
		utils.Disable_systemd_service("umbra-flatpak-manager")
	default:
		utils.Error("Image not supported.")
	}

}
