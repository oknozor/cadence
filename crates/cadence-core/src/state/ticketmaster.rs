use serde::{Deserialize, Serialize};

fn default_radius() -> u32 {
    50
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TicketmasterSettings {
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub preferred_cities: Vec<String>,
    /// Search radius in kilometers (default: 50)
    #[serde(default = "default_radius")]
    pub radius_km: u32,
}

impl Default for TicketmasterSettings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            preferred_cities: Vec::new(),
            radius_km: 50,
        }
    }
}

impl TicketmasterSettings {
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && !self.preferred_cities.is_empty()
    }
}
