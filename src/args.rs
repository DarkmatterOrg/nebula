use crate::modules::funny::print_nebula;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;

const VERSION: &str = clap::crate_version!();

fn nebula_version() -> String {
    let stars = print_nebula();

    let version_message = format!("{}: v{}\n {}", "Nebula".bold().purple(), VERSION, stars);

    return version_message;
}

#[derive(Parser)]
#[command(
    name = "Nebula",
    version = VERSION,
    about = "Nebula CLI Tool",
    arg_required_else_help = true,
    version = nebula_version(),
)]
pub struct Cli {
    /// Run the theme manager
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage themes
    ThemeManager(ThemeManagerArgs),

    /// Manage kernel arguments
    Kargs(KargsManagerArgs),

    /// Fix Users
    FixUsers,

    /// Show Image type
    ShowImage,

    /// Remove flatpaks from a list
    FlatpaksRemove,

    /// Install flatpaks from a list
    FlatpaksInstall,

    /// Update system
    UpdateSystem,
}

#[derive(Args)]
pub struct KargsManagerArgs {
    #[arg(short, long)]
    pub del: Option<String>,

    #[arg(short, long)]
    pub add: Option<String>,

    #[arg(short, long)]
    pub replace: Option<String>,

    #[arg(short, long)]
    pub new_value: Option<String>,
}

#[derive(Args)]
pub struct ThemeManagerArgs {
    /// Set Theme
    #[arg(short, long)]
    pub theme: String,

    /// Set Cursor theme
    #[arg(short, long)]
    pub cursor: String,

    /// Set Icon theme
    #[arg(short, long)]
    pub icons: String,

    /// Set Wallpaper
    #[arg(short, long)]
    pub wallpaper: String,
}
