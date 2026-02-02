use {
    crate::config::Schedule,
    std::{fs, process::Command},
};

pub mod systemd;

pub const SYSTEMD_SERVICE_PATH: &str = "/etc/systemd/system/socialblock.service";
pub const SYSTEMD_TIMER_PATH: &str = "/etc/systemd/system/socialblock.timer";

pub trait Scheduler: Send + Sync {
    fn apply(&self, schedule: &Schedule) -> anyhow::Result<()>;
    fn cmd(&self, c: &str) -> anyhow::Result<()>;
}
