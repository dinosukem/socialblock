use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DomainGroup {
    pub domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub facebook: Option<DomainGroup>,
    pub instagram: Option<DomainGroup>,
    pub whatsapp: Option<DomainGroup>,
    pub messenger: Option<DomainGroup>,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub enable: bool,
    pub block: String,
    pub unblock: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub meta: Option<Meta>,
    pub tiktok: Option<DomainGroup>,
    // pub schedule: Option<Schedule>,
}
