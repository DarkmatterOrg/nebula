package modules

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/darkmatterorg/nebula/config"
	"github.com/darkmatterorg/orbit/utils"
)

func wait() {
	timeout := 25 * time.Second
	deadline := time.Now().Add(timeout)

	for {
		if _, exists := os.LookupEnv("DISPLAY"); exists {
			utils.Notice("DISPLAY is set.. Running")
			return
		}

		if time.Now().After(deadline) {
			utils.Error("DISPLAY isn't set even after 25 seconds, exiting.")
			os.Exit(1)
		}

		time.Sleep(1 * time.Second) // Check once per second
	}
}

func Setplasmatheme(theme string, icons string, cursor string, wallpaper string) {
	if utils.IsProcessRunning("gnome-shell") {
		if config.Config.Insults {
			utils.Error("Are you dumb? Don't run the Plasma theme functions on non-Plasma environment's")
		} else {
			utils.Error("Don't run Plasma theme functions on non-Plasma environment's")
		}
		return
	}

	wait()

	plasma_colorscheme := exec.Command("plasma-apply-colorscheme", theme)
	plasma_colorscheme_err := plasma_colorscheme.Run()

	if plasma_colorscheme_err != nil {
		utils.Error("Failed to set the Plasma colorscheme")
	}

	plasma_icons := exec.Command("/usr/libexec/plasma-changeicons", icons)
	plasma_icons_err := plasma_icons.Run()

	if plasma_icons_err != nil {
		utils.Error("Failed to set the Icon theme")
	}

	plasma_cursor := exec.Command("plasma-apply-cursortheme", cursor)
	plasma_cursor_err := plasma_cursor.Run()

	if plasma_cursor_err != nil {
		utils.Error("Failed to set the cursor theme")
	}

	plasma_wallpaper := exec.Command("plasma-apply-wallpaperimage", wallpaper)
	plasma_wallpaper_err := plasma_wallpaper.Run()

	if plasma_wallpaper_err != nil {
		utils.Error("Failed to set the wallpaper.")
	}
}

func enableGnomeExtensions(filePath string) {
	// Open the file
	file, err := os.Open(filePath)
	if err != nil {
		message := fmt.Sprintf("Failed to open file: %v\n", err)
		utils.Error(message)
		return
	}
	defer file.Close()

	// Read the file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		extension := strings.TrimSpace(scanner.Text()) // Remove whitespace
		if extension == "" {
			continue // Skip empty lines
		}

		// Enable the extension
		cmd := exec.Command("gnome-extensions", "enable", extension)
		err := cmd.Run()
		if err != nil {

			utils.Error(fmt.Sprintf("Failed to enable extension %s\n", extension))

		} else {
			utils.Done(fmt.Sprintf("Enabled extension: %s\n", extension))
		}
	}

	// Check for errors during scanning
	if err := scanner.Err(); err != nil {
		utils.Error(fmt.Sprintf("Error reading file: %v\n", err))
	}
}

func loadDconfFiles(directory string) error {
	// Read all files in the directory
	files, err := os.ReadDir(directory)
	if err != nil {
		return fmt.Errorf("failed to read directory: %v", err)
	}

	// Loop through the files in the directory
	for _, file := range files {
		// Only process regular files (not directories)
		if !file.IsDir() {
			filePath := filepath.Join(directory, file.Name())
			// Execute the dconf command to load each file
			cmd := exec.Command("dconf", "load", "/org/gnome/", filePath)

			// Run the command and get output
			output, err := cmd.CombinedOutput()
			if err != nil {
				utils.Error(fmt.Sprintf("loading file %s: %v\n", file.Name(), err))
				fmt.Println("Output:", string(output))
				continue
			}

			utils.Done(fmt.Sprintf("Successfully loaded %s\n", file.Name()))
		}
	}

	return nil
}

func Setgnometheme(icons string, wallpaper_dark string, wallpaper_light string, cursor string, theme string) {
	if utils.IsProcessRunning("plasmashell") {
		if config.Config.Insults {
			utils.Error("Are you dumb? Don't run GNOME theme functions on non-GNOME environment's")
		} else {
			utils.Error("Don't run GNOME theme functions on non-GNOME environment's")
		}
		return
	}

	wait()

	// Set wallpaper for dark theme
	wallpaper_darkmode := exec.Command("gsettings", "set", "org.gnome.desktop.background", "picture-uri-dark", wallpaper_dark)

	wallpaper_darkmode_err := wallpaper_darkmode.Run()

	if wallpaper_darkmode_err != nil {
		utils.Error("Failed to set the dark mode wallpaper!")
	}

	// Set wallpaper for light theme
	wallpaper_lightmode := exec.Command("gsettings", "set", "org.gnome.desktop.abckground", "picture-uri", wallpaper_light)

	wallpaper_lightmode_err := wallpaper_lightmode.Run()

	if wallpaper_lightmode_err != nil {
		utils.Error("Failed to set the light mode wallpaper!")
	}

	set_cursor := exec.Command("gsettings", "set", "org.gnome.desktop.interface", "cursor-theme", cursor)

	set_cursor_err := set_cursor.Run()

	if set_cursor_err != nil {
		utils.Error("Failed to set the cursor theme!")
	}

	icon_theme := exec.Command("gsettings", "set", "org.gnome.desktop.interface", "icon-theme", icons)

	icon_theme_err := icon_theme.Run()

	if icon_theme_err != nil {
		utils.Error("Failed to set the icon theme!")
	}

	set_gtk_theme := exec.Command("gsettings", "set", "org.gnome.desktop.interface", "gtk-theme", theme)

	set_gtk_theme_err := set_gtk_theme.Run()

	if set_gtk_theme_err != nil {
		utils.Error("Failed to set the GTK theme!")
	}

	switch {
	case strings.Contains(image_type, "arcturus"):
		enableGnomeExtensions("/usr/share/horizon/configs/arcturus/gnome_extensions.txt")
		loadDconfFiles("/usr/share/horizon/configs/arcturus")
	case strings.Contains(image_type, "umbra"):
		enableGnomeExtensions("/usr/share/umbra/configs/gnome_extensions.txt")
	}
}
