pub mod blocker;
pub mod cli;
pub mod config;
pub mod domain;
pub mod notify;
pub mod scheduler;

pub const LOADER_CONFIG_PATH: &str = "./socialblock.toml";
// pub const LOADER_CONFIG_PATH: &str = "/etc/socialblock.toml";

use crate::{
    blocker::Blocker,
    config::{Config, loader::ConfigLoader},
    domain::DomainExpander,
    notify::ConsoleNotifier,
    // scheduler::Scheduler,
};

pub struct SocialBlockApp<B: Blocker> {
    blocker: B,
    // scheduler: S,
    loader: ConfigLoader,
    expander: DomainExpander,
    notifier: ConsoleNotifier,
}

impl<B: Blocker> SocialBlockApp<B> {
    pub fn new(blocker: B, loader: ConfigLoader) -> Self {
        Self {
            blocker,
            // scheduler,
            loader,
            expander: DomainExpander::new(),
            notifier: ConsoleNotifier::new(),
        }
    }

    pub fn apply(&self) -> anyhow::Result<()> {
        let cfg = self.loader.load()?;
        let domains = self.collect_domains(&cfg);

        self.blocker.block(&domains)?;

        // if let Some(s) = &cfg.schedule {
        //     self.scheduler.apply(s)?;
        // }

        self.notifier.info("Config applied successfully");
        Ok(())
    }

    fn collect_domains(&self, cfg: &Config) -> Vec<String> {
        let mut out = vec![];

        if let Some(meta) = &cfg.meta {
            for group in [&meta.facebook, &meta.instagram, &meta.whatsapp]
                .into_iter()
                .flatten()
            {
                for d in &group.domains {
                    out.extend(self.expander.expand(d));
                }
            }
        }

        if let Some(t) = &cfg.tiktok {
            for d in &t.domains {
                out.extend(self.expander.expand(d));
            }
        }

        out
    }

    pub fn block_only(&self) -> anyhow::Result<()> {
        let cfg = self.loader.load()?;
        let domains = self.collect_domains(&cfg);
        self.blocker.block(&domains)?;
        self.notifier.info("Blocked domains");
        Ok(())
    }

    pub fn unblock_only(&self) -> anyhow::Result<()> {
        self.blocker.unblock()?;
        self.notifier.info("Unblocked domains");
        Ok(())
    }

    // pub fn install_systemd(&self) -> anyhow::Result<()> {
    //     let cfg = self.loader.load()?;
    //     if let Some(s) = &cfg.schedule {
    //         self.scheduler.apply(s)?;
    //         self.notifier.info("Systemd timer installed");
    //     }
    //     Ok(())
    // }

    pub fn watch(&self) -> anyhow::Result<()> {
        let path = self.loader.path().to_string();

        self.notifier.info("Watching config changes...");

        crate::config::watcher::ConfigWatcher::watch(&path, || {
            println!("Config changed → re-applying...");
        })?;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    }
}
