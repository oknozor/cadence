//! Subsonic client wrapper for the application

use std::sync::Arc;
use submarine::{auth::AuthBuilder, Client};

#[derive(Clone)]
pub struct SubsonicClient {
    client: Arc<Client>,
}

#[derive(Debug)]
pub enum ClientError {
    Submarine(submarine::SubsonicError),
    Other(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Submarine(e) => write!(f, "Submarine error: {:?}", e),
            ClientError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ClientError {}

impl SubsonicClient {
    /// Create a new Subsonic client
    pub fn new(server_url: &str, username: &str, password: &str) -> Result<Self, ClientError> {
        let auth = AuthBuilder::new(username, "v0.16.1")
            .client_name("cadence")
            .hashed(password);
        let client = Client::new(server_url, auth);
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Get the underlying submarine client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Test connection to the Subsonic server
    pub async fn ping(&self) -> Result<(), ClientError> {
        self.client
            .ping()
            .await
            .map(|_| ())
            .map_err(ClientError::Submarine)
    }
}
