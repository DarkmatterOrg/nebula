use crate::get_image_type;
use crate::running_in_debug;
use colored::Colorize;
use std::process::Command;

pub fn fix_users() {
    if running_in_debug() {
        println!(
            "{}: Do {} run this module on systems other than Horizon,Umbra or Nova.",
            "INFO".bold().blue(),
            "NOT".bold().red()
        );
        std::process::exit(1)
    }

    let image_type = get_image_type();

    match image_type {
        _ if image_type.contains("plasma") => {
            let group_check = Command::new("getent")
                .args(["group", "sddm"])
                .status()
                .expect("Failed to run getent group");

            if !group_check.success() {
                let group_add = Command::new("groupadd")
                    .args(["-r", "sddm"])
                    .status()
                    .expect("Failed to add sddm group");

                if group_add.success() {
                    println!("Created 'sddm' group.");
                } else {
                    eprintln!("Failed to create 'sddm' group.");
                }
            } else {
                println!("{}: 'sddm' group already exists.", "WARN".bold().yellow());
            }

            // Check if 'sddm' user exists, else create it
            let user_check = Command::new("getent")
                .args(["passwd", "sddm"])
                .status()
                .expect("Failed to run getent passwd");

            if !user_check.success() {
                let user_add = Command::new("useradd")
                    .args([
                        "-r", // system account
                        "-g",
                        "sddm", // assign to sddm group
                        "-c",
                        "SDDM Greeter Account", // comment
                        "-d",
                        "/var/lib/sddm", // home directory
                        "-s",
                        "/usr/sbin/nologin", // shell
                        "sddm",              // username
                    ])
                    .status()
                    .expect("Failed to add sddm user");

                if user_add.success() {
                    println!("Created 'sddm' user.");
                } else {
                    eprintln!("{}: Failed to create 'sddm' user.", "ERR".bold().red());
                }
            } else {
                println!("'sddm' user already exists.");
            }
        }

        _ if image_type.contains("gnome") => {
            let group_check = Command::new("getent")
                .args(["group", "gdm"])
                .status()
                .expect("Failed to run getent group");

            if !group_check.success() {
                let group_add = Command::new("groupadd")
                    .args(["-r", "gdm"])
                    .status()
                    .expect("Failed to add sddm group");

                if group_add.success() {
                    println!("Created 'gdm' group.");
                } else {
                    eprintln!("Failed to create 'gdm' group.");
                }
            } else {
                println!("{}: 'gdm' group already exists.", "WARN".bold().yellow());
            }

            // Check if 'sddm' user exists, else create it
            let user_check = Command::new("getent")
                .args(["passwd", "gdm"])
                .status()
                .expect("Failed to run getent passwd");

            if !user_check.success() {
                let user_add = Command::new("useradd")
                    .args([
                        "-r", // system account
                        "-g",
                        "gdm", // assign to sddm group
                        "-c",
                        "GDM Greeter Account", // comment
                        "-d",
                        "/var/lib/gdm", // home directory
                        "-s",
                        "/usr/sbin/nologin", // shell
                        "gdm",               // username
                    ])
                    .status()
                    .expect("Failed to add gdm user");

                if user_add.success() {
                    println!("Created 'gdm' user.");
                } else {
                    eprintln!("{}: Failed to create 'gdm' user.", "ERR".bold().red());
                }
            } else {
                println!("'gdm' user already exists.");
            }
        }

        _ => {}
    }
}
