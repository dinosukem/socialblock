use {
    super::{Blocker, HOSTS_FILE, MARKER},
    std::fs,
};

pub struct HostsBlocker;

impl HostsBlocker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HostsBlocker {
    fn default() -> Self {
        Self::new()
    }
}

impl Blocker for HostsBlocker {
    fn block(&self, domains: &[String]) -> anyhow::Result<()> {
        let mut content = fs::read_to_string(HOSTS_FILE)?;

        for d in domains {
            let line = format!("127.0.0.1 {d} {MARKER}");
            if !content.lines().any(|l| l.trim() == line) {
                content.push('\n');
                content.push_str(&line);
            }
        }

        fs::write(HOSTS_FILE, content)?;
        Ok(())
    }

    fn unblock(&self) -> anyhow::Result<()> {
        let content = fs::read_to_string(HOSTS_FILE)?;

        let cleaned: String = content
            .lines()
            .filter(|l| !l.contains(MARKER))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(HOSTS_FILE, cleaned)?;
        Ok(())
    }
}
