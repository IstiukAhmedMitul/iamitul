use std::fs;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_threads: usize,
    pub default_timeout: u64,
    pub user_agent: String,
    pub default_wordlists: DefaultWordlists,
}

#[derive(Debug, Deserialize)]
pub struct DefaultWordlists {
    pub directories: String,
    pub api_endpoints: String,
    pub subdomains: String,
    pub common: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_threads: 10,
            default_timeout: 10,
            user_agent: "IAMitul/0.1.0".to_string(),
            default_wordlists: DefaultWordlists {
                directories: "wordlists/directories.txt".to_string(),
                api_endpoints: "wordlists/api_endpoints.txt".to_string(),
                subdomains: "wordlists/subdomains.txt".to_string(),
                common: "wordlists/common.txt".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = "iamitul.toml";
        if Path::new(config_path).exists() {
            let config_content = fs::read_to_string(config_path)
                .expect("Failed to read config file");
            toml::from_str(&config_content)
                .expect("Failed to parse config file")
        } else {
            Self::default()
        }
    }
}
