use std::{fs, process::Command};
use colored::Colorize;
use crate::get_image_type;
use std::path::Path;

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

pub fn set_theme() {

    if is_root() {
        println!("Please don't run the theme manager as root.");
        return
    }

    let image_name = get_image_type();

    let config_file_path: Option<&Path>;

    if Path::new("/usr/share/horizon").exists() {
        config_file_path = Some(Path::new("/usr/share/horizon/configs/kinoite/theme.txt"));
    } else if Path::new("/usr/share/nova").exists() {
        config_file_path = Some(Path::new("/usr/share/nova/configx/plasma/theme.txt"));
    } else if Path::new("/usr/share/umbra").exists() {
        config_file_path = Some(Path::new("/usr/share/umbra/configs/plasma/theme.txt"));
    } else {
        // Return an error if no image type file is found
        panic!("{}: No config file found!", "ERROR".bold().red());
    };
    // Read the content of the file into a string
    let contents = fs::read_to_string(config_file_path.unwrap())
        .expect("Failed to read theme config file");

    // Parse the contents to get theme, icons, and wallpaper values
    let mut theme: Option<String> = None;
    let mut icons: Option<String> = None;
    let mut wallpaper: Option<String> = None;

    for line in contents.lines() {
        if line.starts_with("theme:") {
            theme = Some(line.split(":").nth(1).unwrap().trim().to_string());
        } else if line.starts_with("icons:") {
            icons = Some(line.split(":").nth(1).unwrap().trim().to_string());
        } else if line.starts_with("wallpaper:") {
            wallpaper = Some(line.split(":").nth(1).unwrap().trim().to_string());
        }
    }

    if image_name.contains("plasma") || image_name.contains("kinoite") {
        // Apply the theme if it's specified
        if let Some(theme_name) = theme {
            let plasma_colorscheme = Command::new("plasma-apply-colorscheme")
                .arg(theme_name)
                .status()
                .expect("Failed to run the command");

            if !plasma_colorscheme.success() {
                eprintln!("{}: Failed to set the Plasma colorscheme", "ERROR".bold().red());
            }
        }

        // Apply the icon theme if it's specified
        if let Some(icon_theme) = icons {
            let plasma_cursor_theme = Command::new("plasma-apply-cursortheme")
                .arg(icon_theme)
                .status()
                .expect("Failed to run the command");

            if !plasma_cursor_theme.success() {
                eprintln!("{}: Failed to set the cursor theme", "ERROR".bold().red());
            }
        }

        // Apply the wallpaper if it's specified
        if let Some(wallpaper_path) = wallpaper {
            let plasma_apply_wallpaper = Command::new("plasma-apply-wallpaperimage")
                .arg(wallpaper_path)
                .status()
                .expect("Failed to run the command");

            if !plasma_apply_wallpaper.success() {
                eprintln!("{}: Failed to set the wallpaper", "ERROR".bold().red());
            }
        }
    } else if image_name.contains("gnome") {
        // Handle GNOME or other environments here
    }
}
