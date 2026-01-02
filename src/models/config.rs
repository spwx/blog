use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfig {
    pub site: SiteMetadata,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiteMetadata {
    pub name: String,
    pub domain: Option<String>,
    pub description: String,
    #[serde(default = "default_theme")]
    pub default_theme: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServerConfig {
    #[serde(default = "default_bind_address")]
    pub bind_address: String,
}

fn default_bind_address() -> String {
    "127.0.0.1:3000".to_string()
}

fn default_theme() -> String {
    "system".to_string()
}

impl SiteConfig {
    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;

        let config: SiteConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))?;

        Ok(config)
    }

    /// Load configuration from file or use defaults with environment variable overrides
    pub fn load() -> Result<Self> {
        // Try to load from site.toml, fall back to defaults
        let config = Self::from_file("site.toml").unwrap_or_else(|_| {
            eprintln!("Warning: site.toml not found, using defaults and environment variables");
            SiteConfig {
                site: SiteMetadata {
                    name: "Blog".to_string(),
                    domain: None,
                    description: "A technical blog".to_string(),
                    default_theme: default_theme(),
                },
                server: ServerConfig {
                    bind_address: default_bind_address(),
                },
            }
        });

        // Override with environment variables if present
        let config = Self::apply_env_overrides(config);

        Ok(config)
    }

    /// Apply environment variable overrides to configuration
    fn apply_env_overrides(mut config: Self) -> Self {
        if let Ok(description) = std::env::var("SITE_DESCRIPTION") {
            config.site.description = description;
        }

        if let Ok(domain) = std::env::var("SITE_DOMAIN") {
            config.site.domain = Some(domain);
        }

        if let Ok(name) = std::env::var("SITE_NAME") {
            config.site.name = name;
        }

        if let Ok(addr) = std::env::var("BIND_ADDRESS") {
            config.server.bind_address = addr;
        }

        config
    }
}
