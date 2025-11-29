use dioxus::signals::GlobalSignal;
use opensubsonic_cli::types::Search3ResponseSubsonicResponse;
use opensubsonic_cli::{
    Client,
    types::{
        GetAlbumList2ResponseSubsonicResponse, GetAlbumResponseSubsonicResponse,
        SubsonicFailureResponse,
    },
};

use crate::model::{Album, SearchResult};

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

    /// Test connection to the Subsonic server
    pub async fn ping(&self) -> Result<bool, ClientError> {
        self.client
            .ping()
            .await
            .map(|response| {
                let response = response.into_inner().subsonic_response;

                matches!(
                    response,
                    Some(
                        opensubsonic_cli::types::SubsonicResponseSubsonicResponse::SuccessResponse(
                            _
                        )
                    )
                )
            })
            .map_err(ClientError::OpenSubSonic)
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
                let cover = album.cover_art.as_deref().map(cover_url);
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
                        let cover = a.cover_art.as_deref().map(cover_url);
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

    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>, ClientError> {
        let response = self
            .client
            .search3(Some(15), None, Some(15), None, None, query, Some(15), None)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            Search3ResponseSubsonicResponse::Search3SuccessResponse(response) => {
                let results: Vec<SearchResult> = response
                    .search_result3
                    .album
                    .into_iter()
                    .map(|album| SearchResult::Album {
                        id: album.id,
                        name: album.name,
                        cover: album.cover_art.as_deref().map(cover_url),
                        artist: album.display_artist.or(album.artist),
                    })
                    .chain(
                        response
                            .search_result3
                            .artist
                            .into_iter()
                            .map(|artist| SearchResult::Artist {
                                id: artist.id,
                                name: artist.name,
                                thumbnail: artist.artist_image_url.map(ensure_https),
                            })
                            .chain(response.search_result3.song.into_iter().map(|song| {
                                SearchResult::Song {
                                    id: song.id,
                                    name: song.title,
                                    cover: song.cover_art.as_deref().map(cover_url),
                                    artist: song.display_artist.or(song.artist),
                                }
                            })),
                    )
                    .collect();

                Ok(results)
            }
            Search3ResponseSubsonicResponse::SubsonicFailureResponse(failure) => {
                Err(ClientError::Failure(failure))
            }
        }
    }
}

fn ensure_https(url: String) -> String {
    if url.starts_with("https://") {
        url
    } else if url.starts_with("http://") {
        format!("https://{}", &url[7..])
    } else {
        format!("https://{}", url)
    }
}

fn cover_url(id: &str) -> String {
    format!(
        "https://music-api.hoohoot.org/rest/getCoverArt?id={}&f=json&u={}&v={}&p={}&c={}",
        id,
        opensubsonic_cli::USERNAME.get().unwrap(),
        "1.16.1",
        opensubsonic_cli::PASSWORD.get().unwrap(),
        "scrobz"
    )
}
