use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SubSonicLogin {
    pub username: String,
    pub password: String,
    pub server_url: String,
}
