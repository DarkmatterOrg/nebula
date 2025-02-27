use crate::{args::ThemeManagerArgs, get_image_type};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn set_theme(args: &ThemeManagerArgs) {
    let image_name = get_image_type();

    if image_name.contains("plasma") || image_name.contains("aster") {
        // Apply the theme
        run_command(
            "plasma-apply-colorscheme",
            &args.theme,
            "Plasma colorscheme",
        );

        // Apply the cursor theme
        run_command("plasma-apply-cursortheme", &args.cursor, "Cursor theme");

        // Apply the icon theme
        run_command("/usr/libexec/plasma-changeicons", &args.icons, "Icon theme");

        // Apply the wallpaper
        run_command("plasma-apply-wallpaperimage", &args.wallpaper, "Wallpaper");
    } else if image_name.contains("gnome") || image_name.contains("arcturus") {
        // Set wallpaper for dark theme
        run_command(
            "gsettings set org.gnome.desktop.background picture-uri-dark",
            &args.wallpaper,
            "GNOME wallpaper",
        );

        // Set wallpaper for light theme
        run_command(
            "gsettings set org.gnome.desktop.background picture-uri",
            &args.wallpaper,
            "GNOME wallpaper",
        );

        // Set icon theme
        run_command(
            "gsettings set org.gnome.desktop.interface icon-theme",
            &args.icons,
            "GNOME icons",
        );

        // Set icon theme
        run_command(
            "gsettings set org.gnome.desktop.interface gtk-theme",
            &args.theme,
            "GNOME theme",
        );

        // Apply blur-my-shell settings
        run_command(
            "dconf -f load /org/gnome/shell/extensions/blur-my-shell/ <",
            "/usr/share/horizon/configs/arcturus/blur-my-shell.dconf",
            "Blur my shell settings",
        );

        // Apply blur-my-shell settings
        run_command(
            "dconf -f load /org/gnome/shell/extensions/dash-to-dock/ <",
            "/usr/share/horizon/configs/arcturus/dash-to-dock.dconf",
            "Blur my shell settings",
        );

    } else {
        println!("{}", "Unsupported desktop environment.".red());
    }
}

fn run_command(command: &str, arg: &str, description: &str) {
    let status = Command::new(command)
        .arg(arg)
        .status()
        .expect("Failed to run the command");

    if !status.success() {
        eprintln!(
            "{}: Failed to set the {}",
            "ERROR".bold().red(),
            description
        );
    }
}

fn disable_systemd_service() {
    let _systemd_service_path: Option<&Path>;
    let service_name: &str;

    // Determine which systemd service exists
    if Path::new("/usr/share/horizon").exists() {
        _systemd_service_path = Some(Path::new(
            "/usr/lib/systemd/user/horizon-theme-manager.service",
        ));
        service_name = "horizon-theme-manager.service";
    } else if Path::new("/usr/share/nova").exists() {
        _systemd_service_path = Some(Path::new(
            "/usr/lib/systemd/user/nova-theme-manager.service",
        ));
        service_name = "nova-theme-manager.service";
    } else if Path::new("/usr/share/umbra").exists() {
        _systemd_service_path = Some(Path::new(
            "/usr/lib/systemd/user/umbra-theme-manager.service",
        ));
        service_name = "umbra-theme-manager.service";
    } else {
        panic!("{}: No systemd service found!", "ERROR".bold().red());
    };

    // Disable the systemd service using systemctl
    let status = Command::new("systemctl")
        .args(["--user", "disable", service_name])
        .status()
        .expect("Failed to run systemctl disable command");

    if status.success() {
        println!(
            "{}: Successfully disabled {}",
            "SUCCESS".bold().green(),
            service_name
        );
    } else {
        eprintln!(
            "{}: Failed to disable {}",
            "ERROR".bold().red(),
            service_name
        );
    }
}

pub fn theme_manager(args: &ThemeManagerArgs) {
    if is_root() {
        println!(
            "{}: Please don't run the theme manager as root.",
            "WARNING".yellow()
        );
        return;
    } else {
        set_theme(&args);
        disable_systemd_service();
    }
}
