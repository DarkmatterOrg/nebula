use crate::get_image_type;
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn remove_fedora_remote() {
    let mut remove_command = Command::new("flatpak")
        .args(["remote-delete", "--force", "fedora"])
        .spawn()
        .expect("Failed to start flatpak remote-delete");

    let status = remove_command.wait().expect("Failed to remove the remote");

    if !status.success() {
        eprintln!(
            "{}: Failed to remove the Fedora remote!",
            "ERR".bold().red()
        )
    }
}

fn remove_flatpaks() {
    let mut flatpaks_to_remove: Vec<PathBuf> = Vec::new();
    let image_type = get_image_type();

    match image_type {
        _ if image_type.contains("aster") => {
            flatpaks_to_remove.push(PathBuf::from(
                "/usr/share/horizon/packages/aster/aster_flatpak_remove",
            ));
        }

        _ if image_type.contains("arcturus") => return,

        _ if image_type.contains("nova") => return,

        _ => {
            eprintln!("No matching image type for flatpak installation");
            return;
        }
    }

    for flatpak_list in flatpaks_to_remove {
        let contents = fs::read_to_string(flatpak_list).expect("Failed to read flatpaks list");

        for line in contents.lines() {
            if !line.trim().is_empty() {
                let status = Command::new("flatpak")
                    .args(["remove", "--system", "-y", line.trim()])
                    .status()
                    .expect("Failed to run flatpak remove");

                if !status.success() {
                    eprintln!("Failed to remove {line}");
                }
            }
        }
    }
}

pub fn fix_theming() {
    // Helper function to run flatpak override commands
    fn run_flatpak_override(args: &[&str]) {
        let status = Command::new("flatpak")
            .arg("override")
            .args(args)
            .status()
            .expect("Failed to run flatpak override");

        if status.success() {
            println!("{}: flatpak override {:?}", "DONE".bold().green(), args);
        } else {
            eprintln!("{}: flatpak override {:?} failed", "ERR".bold().red(), args);
        }
    }

    // Flatpak theming support
    run_flatpak_override(&[
        "--filesystem=xdg-config/gtk-4.0:ro",
        "--filesystem=xdg-config/gtk-3.0:ro",
        "--filesystem=xdg-data/icons:ro",
    ]);

    // XInput for Firefox
    run_flatpak_override(&["--system", "--env=MOZ_USE_XINPUT2=1", "org.mozilla.firefox"]);

    // Fix printing on LibreOffice
    run_flatpak_override(&[
        "--system",
        "--socket=cups",
        "--socket=session-bus",
        "org.libreoffice.LibreOffice",
    ]);

    // Allow MangoHUD config for Flatpaks
    run_flatpak_override(&[
        "--filesystem=xdg-config/MangoHud:ro",
        "--filesystem=xdg-config/vkBasalt:ro",
    ]);

    // Fix permissions for XIV Launcher
    run_flatpak_override(&["--device=dri", "dev.goats.xivlauncher"]);

    // Fix permissions for Protontricks
    run_flatpak_override(&[
        "--filesystem=~/.local/share/Steam",
        "--filesystem=/var/mnt",
        "--filesystem=/run/media",
        "com.github.Matoking.protontricks",
    ]);

    // Nvidia-specific logic using get_image_type()
    let image_type = get_image_type(); // Uses your existing logic

    if image_type.contains("nvidia") {
        let lshw_output = Command::new("lshw")
            .arg("-C")
            .arg("display")
            .output()
            .expect("Failed to run lshw");

        let lshw_stdout = String::from_utf8_lossy(&lshw_output.stdout);

        let nvidia_present = lshw_stdout.contains("vendor: NVIDIA Corporation");
        let display_count = lshw_stdout.matches("-display").count();

        if nvidia_present && display_count <= 1 {
            // Apply Nvidia-specific flatpak override
            run_flatpak_override(&[
                "--system",
                "--filesystem=host-os",
                "--env=LIBVA_DRIVER_NAME=nvidia",
                "--env=LIBVA_DRIVERS_PATH=/run/host/usr/lib64/dri",
                "--env=LIBVA_MESSAGING_LEVEL=1",
                "--env=MOZ_DISABLE_RDD_SANDBOX=1",
                "--env=NVD_BACKEND=direct",
                "org.mozilla.firefox",
            ]);
        } else {
            // Undo Nvidia-specific overrides
            run_flatpak_override(&[
                "--system",
                "--nofilesystem=host-os",
                "--unset-env=LIBVA_DRIVER_NAME",
                "--unset-env=LIBVA_DRIVERS_PATH",
                "--unset-env=LIBVA_MESSAGING_LEVEL",
                "--unset-env=MOZ_DISABLE_RDD_SANDBOX",
                "--unset-env=NVD_BACKEND",
                "org.mozilla.firefox",
            ]);
        }
    } else {
        // Non-NVIDIA image: Ensure overrides are cleared
        run_flatpak_override(&[
            "--system",
            "--nofilesystem=host-os",
            "--unset-env=LIBVA_DRIVER_NAME",
            "--unset-env=LIBVA_DRIVERS_PATH",
            "--unset-env=LIBVA_MESSAGING_LEVEL",
            "--unset-env=MOZ_DISABLE_RDD_SANDBOX",
            "--unset-env=NVD_BACKEND",
            "org.mozilla.firefox",
        ]);
    }
}

fn enable_flathub() {
    println!("");

    let mut add_command = Command::new("flatpak")
        .args([
            "remote-add",
            "--if-not-exists",
            "--system",
            "flathub",
            "https://dl.flathub.org/repo/flathub.flatpakrepo",
        ])
        .spawn()
        .expect("Failed to start flatpak remote-add");

    let status = add_command.wait().expect("Failed to add flathub");

    if !status.success() {
        eprintln!("{}: Failed to add flathub!", "ERR".bold().red())
    }

    let mut enable_command = Command::new("flatpak")
        .args(["remote-modify", "--system", "--enable", "flathub"])
        .spawn()
        .expect("Failed to start flatpak remote-modify");

    let status = enable_command.wait().expect("Failed to modify flathub");

    if !status.success() {
        eprintln!("{}: Failed to modify flathub!", "ERR".bold().red())
    }
}

fn install_flatpaks() {
    let mut flatpaks_to_install: Vec<PathBuf> = Vec::new();
    let image_type = get_image_type();

    match image_type {
        _ if image_type.contains("aster") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/horizon/packages/aster/aster_flatpaks",
            ));

            flatpaks_to_install.push(PathBuf::from("/usr/share/horizon/packages/shared_flatpaks"));
        }

        // Annoying thing
        _ if image_type.contains("arcturus") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/horizon/packages/arcturus_flatpaks",
            ));

            flatpaks_to_install.push(PathBuf::from("/usr/share/horizon/packages/shared_flatpaks"));
        }

        _ if image_type.contains("umbra") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/umbra/packages/flatpaks"));
        }
        _ if image_type.contains("nova_gnome") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/gnome/install_flatpaks",
            ));
        }
        _ if image_type.contains("nova_gnome_dx") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/gnome/install_flatpaks_dx",
            ));
        }
        _ if image_type.contains("nova_gnome_gaming") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/gnome/install_flatpaks_gaming",
            ));
        }
        _ if image_type.contains("supernova_gnome") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/gnome/install_flatpaks",
            ));
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/gnome/install_flatpaks_dx",
            ));
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/gnome/install_flatpaks_gaming",
            ));
        }
        _ if image_type.contains("nova_plasma") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/plasma/install_flatpaks",
            ));
        }
        _ if image_type.contains("nova_plasma_dx") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/plasma/install_flatpaks_dx",
            ));
        }
        _ if image_type.contains("nova_plasma_gaming") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/plasma/install_flatpaks_gaming",
            ));
        }
        _ if image_type.contains("supernova_plasma") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/plasma/install_flatpaks",
            ));
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/plasma/install_flatpaks_dx",
            ));
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/plasma/install_flatpaks_gaming",
            ));
        }
        _ if image_type.contains("nova_cosmic_dx") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/cosmic/install_flatpaks_dx",
            ));
        }
        _ if image_type.contains("supernova_cosmic") => {
            flatpaks_to_install.push(PathBuf::from(
                "/usr/share/nova/packages/cosmic/install_flatpaks_dx",
            ));
        }

        _ => {
            eprintln!("No matching image type for flatpak installation");
            return;
        }
    }

    for flatpak_list in flatpaks_to_install {
        let contents = fs::read_to_string(flatpak_list).expect("Failed to read flatpaks list");

        for line in contents.lines() {
            if !line.trim().is_empty() {
                let status = Command::new("flatpak")
                    .args(["install", "--system", "-y", line.trim()])
                    .status()
                    .expect("Failed to run flatpak install");

                if !status.success() {
                    eprintln!("Failed to install {line}");
                }
            }
        }
    }
}

fn disable_systemd_service() {
    let _systemd_service_path: Option<&Path>;
    let service_name: &str;

    // Determine which systemd service exists
    if Path::new("/usr/share/horizon").exists() {
        _systemd_service_path = Some(Path::new(
            "/usr/lib/systemd/system/horizon-flatpak-manager.service",
        ));
        service_name = "horizon-flatpak-manager.service";
    } else if Path::new("/usr/share/nova").exists() {
        _systemd_service_path = Some(Path::new(
            "/usr/lib/systemd/system/nova-flatpak-manager.service",
        ));
        service_name = "nova-flatpak-manager.service";
    } else if Path::new("/usr/share/umbra").exists() {
        _systemd_service_path = Some(Path::new(
            "/usr/lib/systemd/system/umbra-flatpak-manager.service",
        ));
        service_name = "umbra-flatpak-manager.service";
    } else {
        panic!("{}: No systemd service found!", "ERR".bold().red());
    };

    // Disable the systemd service using systemctl
    let status = Command::new("systemctl")
        .args(["disable", "--now", service_name])
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

pub fn flatpak_manager_remove() {
    remove_flatpaks();
}

pub fn flatpak_manager() {
    remove_fedora_remote();
    enable_flathub();
    install_flatpaks();
    fix_theming();
    disable_systemd_service();
}
