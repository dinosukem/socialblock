pub mod host;

const HOSTS_FILE: &str = "/etc/hosts";
const MARKER: &str = "# socialblock";

pub trait Blocker: Send + Sync {
    fn block(&self, domains: &[String]) -> anyhow::Result<()>;
    fn unblock(&self) -> anyhow::Result<()>;
}
