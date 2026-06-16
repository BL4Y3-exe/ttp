use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub last_selected_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_selected_mode: "30s".to_owned(),
        }
    }
}
