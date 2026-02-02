use {
    crate::LOADER_CONFIG_PATH,
    clap::{Parser, Subcommand},
};

#[derive(Parser)]
#[command(name = "socialblock")]
#[command(about = "Block social media via hosts + systemd", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(
        short,
        long,
        default_value = LOADER_CONFIG_PATH,
        help = "Path to config file"
    )]
    pub config: String,
}

#[derive(Subcommand)]
pub enum Command {
    /// Apply config: block domains + apply schedule
    Apply,

    /// Block immediately
    Block,

    /// Unblock immediately
    Unblock,

    /// Watch config and auto reload
    Watch,
    // /// Generate & install systemd service + timer
    // InstallSystemd,
}
