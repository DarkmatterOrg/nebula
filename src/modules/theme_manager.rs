use std::process::Command;
use colored::Colorize;
use crate::{args::ThemeManagerArgs, get_image_type};
use std::path::Path;

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn set_theme(args: &ThemeManagerArgs) {
    if is_root() {
        println!("{}: Please don't run the theme manager as root.", "WARNING".yellow());
        return;
    }

    let image_name = get_image_type();

    if image_name.contains("plasma") || image_name.contains("aster") {
        // Apply the theme
        run_command("plasma-apply-colorscheme", &args.theme, "Plasma colorscheme");

        // Apply the icon theme
        run_command("plasma-apply-cursortheme", &args.icons, "Cursor theme");

        // Apply the wallpaper
        run_command("plasma-apply-wallpaperimage", &args.wallpaper, "Wallpaper");
    } else if image_name.contains("gnome") || image_name.contains("arcturus") {
        // Placeholder for GNOME handling
        println!("{}: GNOME support not implemented yet", "WARNING.".bold().yellow());
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
        eprintln!("{}: Failed to set the {}", "ERROR".bold().red(), description);
    }
}

fn disable_systemd_service() {
    let _systemd_service_path: Option<&Path>;
    let service_name: &str;

    // Determine which systemd service exists
    if Path::new("/usr/share/horizon").exists() {
        _systemd_service_path = Some(Path::new("/usr/lib/systemd/user/horizon-theme-manager.service"));
        service_name = "horizon-theme-manager.service";
    } else if Path::new("/usr/share/nova").exists() {
        _systemd_service_path = Some(Path::new("/usr/lib/systemd/user/nova-theme-manager.service"));
        service_name = "nova-theme-manager.service";
    } else if Path::new("/usr/share/umbra").exists() {
        _systemd_service_path = Some(Path::new("/usr/lib/systemd/user/umbra-theme-manager.service"));
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
        println!("{}: Successfully disabled {}", "SUCCESS".bold().green(), service_name);
    } else {
        eprintln!("{}: Failed to disable {}", "ERROR".bold().red(), service_name);
    }
}

pub fn theme_manager(args: &ThemeManagerArgs) {
    set_theme(&args);
    disable_systemd_service();
}