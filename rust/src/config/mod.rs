use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Top-level r9s configuration, mirrors k9s config.yaml structure.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct R9sConfig {
    pub live_view_auto_refresh: bool,
    pub refresh_rate: u64,
    pub max_conn_retry: u32,
    pub current_context: Option<String>,
    pub current_cluster: Option<String>,
    pub headless: bool,
    pub logo_less: bool,
    pub crumbless: bool,
    pub read_only: bool,
    pub no_icons: bool,
    pub skin: String,
}

/// Return the configuration directory (~/.config/r9s or platform equivalent).
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("r9s")
}

/// Return the log directory.
pub fn log_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("r9s")
        .join("logs")
}

/// Load config from disk, falling back to defaults.
pub fn load() -> R9sConfig {
    let path = config_dir().join("config.yaml");
    if let Ok(contents) = std::fs::read_to_string(&path) {
        serde_yaml::from_str(&contents).unwrap_or_default()
    } else {
        R9sConfig::default()
    }
}