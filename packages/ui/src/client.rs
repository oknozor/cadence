use dioxus::signals::GlobalSignal;
use opensubsonic_cli::{
    Client,
    types::{
        GetAlbumList2ResponseSubsonicResponse, GetAlbumResponseSubsonicResponse,
        SubsonicFailureResponse,
    },
};

use crate::album_card::Album;

pub static SUBSONIC_CLIENT: GlobalSignal<Option<SubsonicClient>> = GlobalSignal::new(|| None);

pub enum AlbumListType {
    RecentlyReleased,
    RecentlyPlayed,
}

#[derive(Clone)]
pub struct SubsonicClient {
    client: opensubsonic_cli::Client,
}

#[derive(Debug)]
pub enum ClientError {
    OpenSubSonic(opensubsonic_cli::Error),
    Failure(SubsonicFailureResponse),
    Other(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::OpenSubSonic(e) => write!(f, "OpenSubSonic error: {:?}", e),
            ClientError::Failure(e) => write!(f, "OpenSubSonic error: {:?}", e.error.message),
            ClientError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ClientError {}

impl SubsonicClient {
    /// Create a new Subsonic client
    pub fn new(server_url: &str, username: &str, password: &str) -> Self {
        // TODO: we need to modify the openAPI spec to get rid of this, but  we'd rather
        // talk to OpenSubSonic maintainer to share the work with them.
        opensubsonic_cli::USERNAME.get_or_init(|| username.to_string());
        opensubsonic_cli::PASSWORD.get_or_init(|| password.to_string());

        Self {
            client: Client::new(server_url),
        }
    }

    /// Get the underlying submarine client
    pub fn client(&self) -> &opensubsonic_cli::Client {
        &self.client
    }

    /// Test connection to the Subsonic server
    pub async fn ping(&self) -> Result<(), ClientError> {
        self.client
            .ping()
            .await
            .map(|_| ())
            .map_err(ClientError::OpenSubSonic)
    }

    fn cover_url(&self, id: &str) -> String {
        format!(
            "http://music-api.hoohoot.org/rest/getCoverArt?id={}&f=json&u={}&v={}&p={}&c={}",
            id,
            opensubsonic_cli::USERNAME.get().unwrap(),
            "1.16.1",
            opensubsonic_cli::PASSWORD.get().unwrap(),
            "scrobz"
        )
    }
    /// Get album by ID
    pub async fn get_album(&self, id: &str) -> Result<Album, ClientError> {
        let response = self
            .client
            .get_album(id)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            GetAlbumResponseSubsonicResponse::GetAlbumSuccessResponse(response) => {
                let album = response.album;

                let mut album = album;
                let cover = album.cover_art.map(|c| self.cover_url(&c));
                album.cover_art = cover;

                Ok(Album::from(album))
            }
            GetAlbumResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }
    pub async fn list_album(&self, album_type: AlbumListType) -> Result<Vec<Album>, ClientError> {
        let album_list_type = match album_type {
            AlbumListType::RecentlyReleased => opensubsonic_cli::types::AlbumListType::Newest,
            AlbumListType::RecentlyPlayed => opensubsonic_cli::types::AlbumListType::Recent,
        };

        let response = self
            .client
            .get_album_list2(None, None, None, None, None, None, album_list_type)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            GetAlbumList2ResponseSubsonicResponse::GetAlbumList2SuccessResponse(response) => {
                Ok(response
                    .album_list2
                    .album
                    .into_iter()
                    .map(|a| {
                        let mut a = a;
                        let cover = a.cover_art.map(|c| self.cover_url(&c));
                        a.cover_art = cover;
                        a
                    })
                    .map(Album::from)
                    .collect())
            }
            GetAlbumList2ResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }
}
