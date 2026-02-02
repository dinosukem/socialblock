use {
    super::{Blocker, HOSTS},
    std::fs,
};

pub struct HostsBlocker;

impl HostsBlocker {
    pub fn new() -> Self {
        Self
    }
}

impl Blocker for HostsBlocker {
    fn block(&self, domains: &[String]) -> anyhow::Result<()> {
        let mut content = fs::read_to_string(HOSTS)?;

        for d in domains {
            let line = format!("127.0.0.1 {d}");
            if !content.contains(&line) {
                content.push('\n');
                content.push_str(&line);
            }
        }

        fs::write(HOSTS, content)?;
        Ok(())
    }

    fn unblock(&self, domains: &[String]) -> anyhow::Result<()> {
        let content = fs::read_to_string(HOSTS)?;
        let filtered: String = content
            .lines()
            .filter(|l| !domains.iter().any(|d| l.contains(d)))
            .map(|l| format!("{l}\n"))
            .collect();

        fs::write(HOSTS, filtered)?;
        Ok(())
    }
}
