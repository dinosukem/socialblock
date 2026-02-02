pub mod host;

const HOSTS: &str = "/etc/hosts";

pub trait Blocker: Send + Sync {
    fn block(&self, domains: &[String]) -> anyhow::Result<()>;
    fn unblock(&self, domains: &[String]) -> anyhow::Result<()>;
}
