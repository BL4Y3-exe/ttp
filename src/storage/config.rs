use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
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
    let paths = config_paths();

    for path in &paths {
        if !path.exists() {
            continue;
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read config at {}", path.display()))?;
        let config = toml::from_str(&contents)
            .with_context(|| format!("failed to parse config at {}", path.display()))?;

        return Ok(config);
    }

    let config = AppConfig::default();
    save_config(&config)?;
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let mut last_error = None;

    for path in config_paths() {
        match write_config_to_path(config, &path) {
            Ok(()) => return Ok(()),
            Err(error) => last_error = Some(error),
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow!("no config paths available")))
}

fn write_config_to_path(config: &AppConfig, path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config dir {}", parent.display()))?;
    }

    let contents = toml::to_string_pretty(config).context("failed to serialize config")?;
    fs::write(path, contents)
        .with_context(|| format!("failed to write config at {}", path.display()))?;

    Ok(())
}

fn config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(2);

    if let Some(config_dir) = dirs::config_dir() {
        paths.push(config_dir.join("ttp").join("config.toml"));
    }

    paths.push(PathBuf::from(".ttp").join("config.toml"));
    dedupe_paths(paths)
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::with_capacity(paths.len());

    for path in paths {
        if !deduped.contains(&path) {
            deduped.push(path);
        }
    }

    deduped
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    #[test]
    fn app_config_default_mode_is_30s() {
        assert_eq!(AppConfig::default().last_selected_mode, "30s");
    }
}
