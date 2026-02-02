use {
    clap::Parser,
    socialblock::{
        blocker::host::HostsBlocker,
        cli::{Cli, Command},
        config::loader::ConfigLoader,
        *,
    },
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let loader = ConfigLoader::new(&cli.config);

    let app = std::sync::Arc::new(SocialBlockApp::new(
        HostsBlocker::new(),
        loader,
        // SystemdScheduler::new(std::env::current_exe()?.display().to_string()),
    ));

    match cli.command {
        Command::Apply => app.apply(),
        Command::Block => app.block_only(),
        Command::Unblock => app.unblock_only(),
        Command::Watch => std::sync::Arc::clone(&app).watch(),
        // Command::InstallSystemd => app.install_systemd(),
    }
}
