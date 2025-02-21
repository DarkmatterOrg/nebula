use colored::Colorize;
use std::process::Command;
use std::env;
use std::path::Path;
use libc;

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

pub fn update_python_packages() {

    println!("\n{}", "Updating python packages...".bold());

    // Run the pip3 freeze command and capture the output
    let output = Command::new("pip3")
        .arg("freeze")
        .output()
        .expect("Failed to run pip3 freeze");

    if !output.status.success() {
        eprintln!("Failed to list installed Python packages with pip3");
        return;
    }

    // Capture the list of installed packages
    let packages = String::from_utf8_lossy(&output.stdout);

    // Loop over each package name and upgrade it
    for line in packages.lines() {
        let package = line.split('=').next().unwrap(); // Get the package name before the version part

        let upgrade_output = Command::new("pip3")
            .arg("install")
            .arg("--upgrade")
            .arg(package)
            .output()
            .expect(&format!("Failed to upgrade package {}", package));

        if upgrade_output.status.success() {
            println!("Successfully upgraded {}", package);
        } else {
            eprintln!("Failed to upgrade {}", package);
        }
    }
}

pub fn update_node_packages() {
    let node_pkg_managers = vec!["pnpm", "yarn", "npm", "bun"];

    let path_var = env::var("PATH").expect("Failed to read PATH");
    let paths: Vec<_> = env::split_paths(&path_var).collect(); // Collect paths into a Vec

    println!("\n{}", "Updating node packages...".bold());

    // Find the first package manager that exists in the PATH
    if let Some(pkg_manager) = node_pkg_managers.iter().find(|&&pkg| {
        paths.iter().any(|dir| Path::new(&dir).join(pkg).exists())
    }) {
        // Use the found package manager to update the packages
        let output = Command::new(pkg_manager)
            .arg("update")
            .output()
            .expect(&format!("Failed to run {}", pkg_manager));

        if output.status.success() {
            println!("Packages updated successfully using {}", pkg_manager);
        } else {
            eprintln!("{}: Failed to update packages using {}", "ERROR".bold().red(), pkg_manager);
        }
    } else {
        eprintln!("{}: No known node package manager found in the system PATH", "ERROR".bold().red());
    }
}

pub fn update_distrobox() {

    if Path::new("/usr/bin/distrobox").exists() {
        println!("\n{}", "Updating distrobox containers...".bold());

        let distrobox_update = Command::new("distrobox").args(["upgrade", "--all"]).status().expect("Failed to upgrade all distroboc containers!");
    
        if !distrobox_update.success() {
            eprintln!("{}: Failed to upgrade distrobox containers!", "ERROR".bold().red())
        }
    } else {
        return
    }

}

pub fn update_flatpaks() {
    if Path::new("/usr/bin/flatpak").exists() {
        println!("\n{}", "Updating flatpaks... (It may hang for a few minutes)".bold());

        let flatpak_update = Command::new("flatpak").args(["update", "-y"]).status().expect("Failed to run command");
    
        if !flatpak_update.success() {
            eprintln!("{}: Failed to update flatpaks!", "ERROR".bold().red())
        }
    } else {
        return
    }
}

pub fn update_snaps() {
    if Path::new("/usr/bin/snap").exists() {
        println!("\n{}", "Updating snaps...".bold());

        let snap_update = Command::new("snap").arg("update").status().expect("Failed to run command");

        if !snap_update.success() {
            eprintln!("{}: Failed to update snaps!", "ERROR".bold().red())
        }
    } else {
        return
    }
}

pub fn update_image() {
    if Path::new("/usr/bin/bootc").exists() {
        println!("\n{}", "Updating base image...".bold());
        let image_update = Command::new("bootc").arg("upgrade").status().expect("Failed to run command");
    
        if !image_update.success() {
            eprintln!("{}: Failed to update the base image!!", "ERROR".bold().red())
        }
    } else {
        return
    }
}

pub fn update_all() {

    if is_root() {
        update_flatpaks();
        update_snaps();
        update_image();
    } else {
        update_distrobox();
        update_node_packages();
        update_python_packages();
    }

}