mod modules;
mod args;

use clap::Parser;
use colored::Colorize;

use crate::modules::{
    image_type::get_image_type, flatpak_manager::{remove_fedora_remote, remove_flatpaks, install_flatpaks}, theme_manager::set_theme, update_manager::update_all,
    fix_users::fix_users, funny::print_nebula
};

const VERSION: &str = clap::crate_version!();
fn main() {
    let args = args::Cli::parse();


    if args.version == true {
        println!("{}: v{}", "Nebula".bold().purple(), VERSION);
        print_nebula();
    }

    if args.installflatpaks == true {
        remove_fedora_remote();
        install_flatpaks();
    }

    if args.removeflatpaks == true {
        remove_flatpaks();
    }

    if args.showimage == true {
        let image_type = get_image_type();
        println!("{}: {}", "Image Type".bold().purple(), image_type)
    }

    if args.updatesystem == true {
        update_all();
    }

    if args.thememanager == true {
        set_theme();
    }

    if args.fixusers == true {
        fix_users();
    }    
}
