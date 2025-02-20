use std::fs;
use colored::Colorize;
use std::path::PathBuf;
use crate::get_image_type;
use std::process::Command;

pub fn remove_fedora_remote() {
    let mut remove_command = Command::new("flatpak")
        .args(["remote-delete", "--force" ,"fedora"])
        .spawn()
        .expect("Failed to start flatpak remote-delete");


    let status = remove_command.wait().expect("Failed to remove the remote");

    if !status.success() {
        eprintln!("{}: Failed to remove the Fedora remote!", "ERROR".bold().red())
    }
}

pub fn remove_flatpaks() {
    let mut flatpaks_to_remove: Vec<PathBuf> = Vec::new();
    let image_type = get_image_type();

    match image_type {
        _ if image_type.contains("kinoite") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/horizon/packages/kinoite_flatpak_remove"));
        },
        _ if image_type == "plasma" => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/umbra/plasma/flatpaks_remove"));
        },
        /*_ if image_type.contains("nova_gnome") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks"));
        },
        _ if image_type.contains("nova_gnome_dx") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_dx"));
        },
        _ if image_type.contains("nova_gnome_gaming") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_gaming"));
        },
        _ if image_type.contains("supernova_gnome") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks"));
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_dx"));
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_gaming"));
        },
        _ if image_type.contains("nova_plasma") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks"));
        },
        _ if image_type.contains("nova_plasma_dx") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_dx"));
        },
        _ if image_type.contains("nova_plasma_gaming") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_gaming"));
        },
        _ if image_type.contains("supernova_plasma") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks"));
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_dx"));
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_gaming"));
        },
        _ if image_type.contains("nova_cosmic_dx") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/cosmic/install_flatpaks_dx"));
        },
        _ if image_type.contains("supernova_cosmic") => {
            flatpaks_to_remove.push(PathBuf::from("/usr/share/nova/packages/cosmic/install_flatpaks_dx"));
        }, */

        _ => {
            eprintln!("No matching image type for flatpak installation");
            return;
        },
    }

    for flatpak_list in flatpaks_to_remove {
        let contents = fs::read_to_string(flatpak_list)
            .expect("Failed to read flatpaks list");

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

pub fn install_flatpaks() {
    let mut flatpaks_to_install: Vec<PathBuf> = Vec::new();
    let image_type = get_image_type();

    match image_type {
        _ if image_type.contains("kinoite") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/horizon/packages/kinoite_flatpaks"));
        },
        _ if image_type == "plasma" => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/umbra/packages/flatpaks"));
        },
        _ if image_type.contains("nova_gnome") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks"));
        },
        _ if image_type.contains("nova_gnome_dx") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_dx"));
        },
        _ if image_type.contains("nova_gnome_gaming") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_gaming"));
        },
        _ if image_type.contains("supernova_gnome") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks"));
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_dx"));
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/gnome/install_flatpaks_gaming"));
        },
        _ if image_type.contains("nova_plasma") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks"));
        },
        _ if image_type.contains("nova_plasma_dx") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_dx"));
        },
        _ if image_type.contains("nova_plasma_gaming") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_gaming"));
        },
        _ if image_type.contains("supernova_plasma") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks"));
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_dx"));
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/plasma/install_flatpaks_gaming"));
        },
        _ if image_type.contains("nova_cosmic_dx") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/cosmic/install_flatpaks_dx"));
        },
        _ if image_type.contains("supernova_cosmic") => {
            flatpaks_to_install.push(PathBuf::from("/usr/share/nova/packages/cosmic/install_flatpaks_dx"));
        },

        _ => {
            eprintln!("No matching image type for flatpak installation");
            return;
        },
    }

    for flatpak_list in flatpaks_to_install {
        let contents = fs::read_to_string(flatpak_list)
            .expect("Failed to read flatpaks list");

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