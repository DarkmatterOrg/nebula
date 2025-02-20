use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Nebula",
    version = "1.0",
    about = "Nebula CLI Tool",
    arg_required_else_help = true
)]
pub struct Cli {
    #[clap(short, long)]
    /// Show version
    pub version: bool,

    /// Show image type
    #[clap(short, long)]
    pub showimage: bool,

    /// Install flatpaks from list
    #[clap(short, long)]
    pub installflatpaks: bool,

    /// Remove flatpaks from list
    #[clap(short, long)]
    pub removeflatpaks: bool,

    /// Run the theme manager
    #[clap(short, long)]
    pub thememanager: bool,

    /// Run the update manager
    #[clap(short, long)]
    pub updatesystem: bool,

    /// Add missing users
    #[clap(short, long)]
    pub fixusers: bool,
}

#[derive(Parser)]
pub struct ThemeManagerCli {
    #[clap(short, long)]
    /// Set Theme
    pub theme: Option<String>,

    #[clap(short, long)]
    /// Set Icon theme
    pub icons: Option<String>,

    #[clap(short, long)]
    /// Set Wallpaper
    pub wallpaper: Option<String>,
}
