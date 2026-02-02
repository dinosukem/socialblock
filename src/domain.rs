pub struct DomainExpander {
    prefixes: Vec<&'static str>,
}

impl DomainExpander {
    pub fn new() -> Self {
        Self {
            prefixes: vec!["www", "m", "api", "cdn", "static", "graph"],
        }
    }

    pub fn expand(&self, domain: &str) -> Vec<String> {
        if !domain.starts_with("*.") {
            return vec![domain.to_string()];
        }

        let base = domain.trim_start_matches("*.");
        let mut out = vec![base.to_string()];

        for p in &self.prefixes {
            out.push(format!("{p}.{base}"));
        }

        out
    }
}
