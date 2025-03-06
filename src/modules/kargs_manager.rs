use colored::Colorize;
use std::process::Command;

fn run_command(command: &str, arg: &str, description: &str) {
    let full_command = format!("{} {}", command, arg);

    let status = Command::new("/bin/sh")
        .args(["-c", full_command.as_str()])
        .status()
        .expect("Failed to run the command");

    if !status.success() {
        eprintln!("{}: Failed to {}", "ERR".bold().red(), description);
    }
}

pub fn remove_argument(karg: String) {
    run_command(
        "rpm-ostree",
        format!("kargs --delete={}", karg.as_str()).as_str(),
        "remove a kernel argument",
    )
}

pub fn replace_value(karg: String, new_value: String) {
    run_command(
        "rpm-ostree",
        format!("kargs replace={}={}", karg.as_str(), new_value.as_str()).as_str(),
        "replace the kernel argument",
    );
}

pub fn add_argument(karg: String) {
    run_command(
        "rpm-ostree",
        format!("kargs --append={}", karg.as_str()).as_str(),
        "add a kernel argument",
    );
}
