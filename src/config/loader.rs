use {super::*, std::fs};

pub struct ConfigLoader {
    path: String,
}

impl ConfigLoader {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> anyhow::Result<Config> {
        let raw = fs::read_to_string(&self.path)?;
        Ok(toml::from_str(&raw)?)
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
