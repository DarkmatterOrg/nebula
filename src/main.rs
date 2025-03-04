mod args;
mod modules;

use args::Commands;
use clap::Parser;
use colored::Colorize;

use crate::modules::{
    fix_users::fix_users,
    flatpak_manager::{flatpak_manager, flatpak_manager_remove},
    image_type::get_image_type,
    theme_manager::theme_manager,
    update_manager::update_all,
};

fn main() {
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
