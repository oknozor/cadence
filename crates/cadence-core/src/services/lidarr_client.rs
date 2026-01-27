use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::state::LidarrSettings;

#[derive(Clone)]
pub struct LidarrClient {
    client: Client,
    base_url: String,
    api_key: String,
}

#[derive(Debug)]
pub enum LidarrError {
    Http(reqwest::Error),
    NotFound(String),
    ApiError(String),
}

impl std::fmt::Display for LidarrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LidarrError::Http(e) => write!(f, "HTTP error: {}", e),
            LidarrError::NotFound(msg) => write!(f, "Not found: {}", msg),
            LidarrError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for LidarrError {}

impl From<reqwest::Error> for LidarrError {
    fn from(e: reqwest::Error) -> Self {
        LidarrError::Http(e)
    }
}

// API Response types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResource {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub foreign_album_id: Option<String>,
    pub artist: Option<ArtistResource>,
    pub monitored: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistResource {
    pub id: Option<i32>,
    pub artist_name: Option<String>,
    pub foreign_artist_id: Option<String>,
    pub quality_profile_id: Option<i32>,
    pub metadata_profile_id: Option<i32>,
    pub root_folder_path: Option<String>,
    pub monitored: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAlbumRequest {
    pub foreign_album_id: String,
    pub monitored: bool,
    pub artist: AddArtistRequest,
    pub add_options: AddOptions,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistRequest {
    pub foreign_artist_id: String,
    pub quality_profile_id: i32,
    pub metadata_profile_id: i32,
    pub root_folder_path: String,
    pub monitored: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
    pub search_for_new_album: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootFolderResource {
    pub id: Option<i32>,
    pub path: Option<String>,
    pub default_quality_profile_id: Option<i32>,
    pub default_metadata_profile_id: Option<i32>,
}

impl LidarrClient {
    pub fn new(settings: &LidarrSettings) -> Self {
        let base_url = settings.url.trim_end_matches('/').to_string();
        Self {
            client: Client::new(),
            base_url,
            api_key: settings.api_key.clone(),
        }
    }

    /// Search for an album by artist and title
    pub async fn search_album(
        &self,
        artist: &str,
        album: &str,
    ) -> Result<Vec<AlbumResource>, LidarrError> {
        let term = format!("{} {}", artist, album);
        let url = format!("{}/api/v1/album/lookup", self.base_url);

        tracing::info!("[Lidarr] Searching for album: {}", term);

        let response = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .query(&[("term", &term)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("[Lidarr] Search failed: {} - {}", status, text);
            return Err(LidarrError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let albums: Vec<AlbumResource> = response.json().await?;
        tracing::info!("[Lidarr] Found {} albums", albums.len());
        Ok(albums)
    }

    /// Get root folders (to determine default paths and profiles)
    pub async fn get_root_folders(&self) -> Result<Vec<RootFolderResource>, LidarrError> {
        let url = format!("{}/api/v1/rootfolder", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(LidarrError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        Ok(response.json().await?)
    }

    /// Add an album to Lidarr and trigger a search
    pub async fn add_album(&self, album: &AlbumResource) -> Result<AlbumResource, LidarrError> {
        // Get root folder for defaults
        let root_folders = self.get_root_folders().await?;
        let root_folder = root_folders.first().ok_or_else(|| {
            LidarrError::NotFound("No root folder configured in Lidarr".to_string())
        })?;

        let foreign_album_id = album
            .foreign_album_id
            .as_ref()
            .ok_or_else(|| LidarrError::NotFound("Album has no foreign ID".to_string()))?;

        let artist = album
            .artist
            .as_ref()
            .ok_or_else(|| LidarrError::NotFound("Album has no artist info".to_string()))?;

        let foreign_artist_id = artist
            .foreign_artist_id
            .as_ref()
            .ok_or_else(|| LidarrError::NotFound("Artist has no foreign ID".to_string()))?;

        let url = format!("{}/api/v1/album", self.base_url);

        let request = AddAlbumRequest {
            foreign_album_id: foreign_album_id.clone(),
            monitored: true,
            artist: AddArtistRequest {
                foreign_artist_id: foreign_artist_id.clone(),
                quality_profile_id: root_folder.default_quality_profile_id.unwrap_or(1),
                metadata_profile_id: root_folder.default_metadata_profile_id.unwrap_or(1),
                root_folder_path: root_folder.path.clone().unwrap_or_default(),
                monitored: true,
            },
            add_options: AddOptions {
                search_for_new_album: true,
            },
        };

        tracing::info!("[Lidarr] Adding album: {:?}", album.title);

        let response = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("[Lidarr] Add album failed: {} - {}", status, text);
            return Err(LidarrError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let added_album: AlbumResource = response.json().await?;
        tracing::info!("[Lidarr] Album added with ID: {:?}", added_album.id);
        Ok(added_album)
    }

    /// Trigger an album search command
    pub async fn search_for_album(&self, album_id: i32) -> Result<(), LidarrError> {
        let url = format!("{}/api/v1/command", self.base_url);

        let request = CommandRequest {
            name: "AlbumSearch".to_string(),
            album_ids: Some(vec![album_id]),
        };

        tracing::info!("[Lidarr] Triggering search for album ID: {}", album_id);

        let response = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("[Lidarr] Search command failed: {} - {}", status, text);
            return Err(LidarrError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        tracing::info!("[Lidarr] Search command queued successfully");
        Ok(())
    }
}
