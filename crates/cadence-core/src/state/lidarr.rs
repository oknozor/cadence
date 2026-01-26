use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct LidarrSettings {
    pub url: String,
    pub api_key: String,
}

impl LidarrSettings {
    pub fn is_configured(&self) -> bool {
        !self.url.is_empty() && !self.api_key.is_empty()
    }
}

