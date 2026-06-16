use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub last_selected_mode: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            last_selected_mode: "30s".to_owned(),
        }
    }
}

pub fn load_config() -> Result<AppConfig> {
    let path = config_path();

    if !path.exists() {
        let config = AppConfig::default();
        save_config(&config)?;
        return Ok(config);
    }

    let contents = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config at {}", path.display()))?;
    let config = toml::from_str(&contents)
        .with_context(|| format!("failed to parse config at {}", path.display()))?;

    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config dir {}", parent.display()))?;
    }

    let contents = toml::to_string_pretty(config).context("failed to serialize config")?;
    fs::write(&path, contents)
        .with_context(|| format!("failed to write config at {}", path.display()))?;

    Ok(())
}

fn config_path() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("ttp").join("config.toml")
    } else {
        PathBuf::from(".ttp").join("config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    #[test]
    fn app_config_default_mode_is_30s() {
        assert_eq!(AppConfig::default().last_selected_mode, "30s");
    }
}
