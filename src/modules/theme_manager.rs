use crate::{args::ThemeManagerArgs, get_image_type};
use colored::Colorize;
use std::env;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn wait() {
    let timeout = time::Duration::from_secs(25);
    let start = time::Instant::now();

    loop {
        if env::var("DISPLAY").is_ok() {
            println!("DISPLAY is set! Running..");
            return; // Success, exit normally
        }

        if start.elapsed() >= timeout {
            eprintln!("Error: DISPLAY is not set after waiting 25 seconds.");
            std::process::exit(1); // Exit with error
        }

        thread::sleep(time::Duration::from_secs(1)); // Check once per second
    }
}

fn set_gnome_theme(args: &ThemeManagerArgs) {
    let image_type = get_image_type();

    if image_type.contains("arcturus") {
        // Set wallpaper for dark theme
        run_command(
            "gsettings",
            format!(
                "set org.gnome.desktop.background picture-uri-dark {}",
                &args.wallpaper
            )
            .as_str(),
            "GNOME wallpaper",
        );

        // Set wallpaper for light theme
        run_command(
            "gsettings",
            format!(
                "set org.gnome.desktop.background picture-uri {}",
                &args.wallpaper
            )
            .as_str(),
            "GNOME wallpaper",
        );

        // Set cursor theme
        run_command(
            "gsettings",
            format!(
                "set org.gnome.desktop.interface cursor-theme {}",
                &args.cursor
            )
            .as_str(),
            "Set Cursor theme",
        );

        // Set icon theme
        run_command(
            "gsettings",
            format!("set org.gnome.desktop.interface icon-theme {}", &args.icons).as_str(),
            "GNOME icons",
        );

        // Set gtk theme
        run_command(
            "gsettings",
            format!("set org.gnome.desktop.interface gtk-theme {}", &args.theme).as_str(),
            "GNOME theme",
        );

        // Apply blur-my-shell settings
        run_command(
            "dconf",
            "load -f /org/gnome/shell/extensions/blur-my-shell/ < /usr/share/horizon/configs/arcturus/blur-my-shell.dconf",
            "Blur my shell settings",
        );

        // Apply blur-my-shell settings
        run_command(
            "dconf",
            "load -f /org/gnome/shell/extensions/dash-to-dock/ < /usr/share/horizon/configs/arcturus/dash-to-dock.dconf",
            "Blur my shell settings",
        );

        // Enable Blur my shell
        run_command(
            "gnome-extensions",
            "enable blur-my-shell@aunetx",
            "Enable blur my shell",
        );

        // Enable dash to dock
        run_command(
            "gnome-extensions",
            "enable dash-to-dock@micxgx.gmail.com",
            "Enable dash to dock",
        );
    }
}

fn set_plasma_theme(args: &ThemeManagerArgs) {
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
    } else {
        println!("{}", "Unsupported desktop environment.".red());
    }
}

fn run_command(command: &str, arg: &str, description: &str) {
    let full_command = format!("{} {}", command, arg);

    let status = Command::new("/bin/sh")
        .args(["-c", full_command.as_str()])
        .status()
        .expect("Failed to run the command");

    if !status.success() {
        eprintln!("{}: Failed to set the {}", "ERR".bold().red(), description);
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
        panic!("{}: No systemd service found!", "ERR".bold().red());
    };

    // Disable the systemd service using systemctl
    let status = Command::new("systemctl")
        .args(["--user", "disable", service_name])
        .status()
        .expect("Failed to run systemctl disable command");

    if status.success() {
        println!(
            "{}: Successfully disabled {}",
            "DONE".bold().green(),
            service_name
        );
    } else {
        eprintln!("{}: Failed to disable {}", "ERR".bold().red(), service_name);
    }
}

pub fn theme_manager(args: &ThemeManagerArgs) {
    let image_type = get_image_type();

    if is_root() {
        println!(
            "{}: Please don't run the theme manager as root.",
            "WARN".yellow()
        );
        return;
    } else {
        wait();

        if image_type.contains("arcturus") {
            set_gnome_theme(&args);
        } else if image_type.contains("aster") {
            set_plasma_theme(&args);
        }

        disable_systemd_service();
    }
}
