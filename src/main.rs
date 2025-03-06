mod args;
mod modules;

use args::Commands;
use clap::Parser;
use colored::Colorize;
use std::path::Path;

use crate::modules::{
    debug_mode::running_in_debug,
    fix_users::fix_users,
    flatpak_manager::{flatpak_manager, flatpak_manager_remove},
    image_type::get_image_type,
    kargs_manager::{add_argument, remove_argument, replace_value},
    theme_manager::theme_manager,
    update_manager::update_all,
};

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn main() {
    if running_in_debug() {
        println!("{}: Running in Debug mode.", "INFO".bold().blue());
    }

    let args = args::Cli::parse();

    match args.command {
        Some(Commands::ThemeManager(theme_args)) => {
            // Call theme manager function here
            theme_manager(&theme_args);
        }
        Some(Commands::FixUsers) => {
            // Call the fix_users function directly when the FixUsers subcommand is used
            fix_users();
        }

        Some(Commands::ShowImage) => {
            let image_type = get_image_type();
            println!("{}: {}", "Image Type".bold().purple(), image_type)
        }

        Some(Commands::Kargs(kargs)) => {
            if !is_root() && !running_in_debug() {
                eprintln!("You have to run the kargs manager as root.");
                std::process::exit(1);
            }
            if kargs.add.is_none() && kargs.del.is_none() && kargs.replace.is_none() {
                eprintln!("{}: No arguments provided.", "WARN".bold().yellow());
                std::process::exit(1);
            }

            if !Path::new("/usr/bin/rpm-ostree").exists() && !running_in_debug() {
                eprintln!("{}: rpm-ostree is missing.", "ERR".bold().red());
                std::process::exit(1);
            }

            let new_value = kargs.new_value.as_deref().unwrap_or("none");

            if let Some(value) = kargs.add {
                add_argument(value);
            }
            if let Some(value) = kargs.del {
                remove_argument(value);
            }
            if let Some(value) = kargs.replace {
                replace_value(value, new_value.to_string());
            }
        }

        Some(Commands::FlatpaksRemove) => {
            flatpak_manager_remove();
        }

        Some(Commands::FlatpaksInstall) => {
            flatpak_manager();
        }

        Some(Commands::UpdateSystem) => {
            update_all();
        }

        None => {
            return;
        }
    }
}
