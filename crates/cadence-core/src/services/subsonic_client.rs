use crate::model::{Album, Artist, PlaylistInfo, SearchResult, Song, Starred};
use dioxus::signals::GlobalSignal;
use opensubsonic_cli::{
    Client,
    types::{
        GetAlbumList2ResponseSubsonicResponse, GetAlbumResponseSubsonicResponse,
        GetArtistInfo2ResponseSubsonicResponse::GetArtistInfo2SuccessResponse,
        GetArtistResponseSubsonicResponse,
        GetPlaylistsResponseSubsonicResponse::{self, GetPlaylistsSuccessResponse},
        GetRandomSongsResponseSubsonicResponse, GetStarred2ResponseSubsonicResponse,
        Search3ResponseSubsonicResponse, SubsonicFailureResponse,
    },
};

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
        //   talk to OpenSubSonic maintainer to share the work with them.
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

    /// Get an album by ID
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
                Ok(Album::from(response.album))
            }
            GetAlbumResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }

    /// Get an artist by ID
    pub async fn get_artist(&self, id: &str) -> Result<Artist, ClientError> {
        let (artist, artist_info) = tokio::join!(
            self.client.get_artist(id),
            self.client.get_artist_info2(None, id, Some(false))
        );

        let artist = artist.map_err(ClientError::OpenSubSonic)?;
        let artist = artist
            .into_inner()
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        let mut artist = match artist {
            GetArtistResponseSubsonicResponse::GetArtistSuccessResponse(response) => {
                Artist::from(response.artist)
            }
            GetArtistResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => return Err(ClientError::Failure(subsonic_failure_response.clone())),
        };

        let artist_info = artist_info.map_err(ClientError::OpenSubSonic)?;
        let artist_info = artist_info.into_inner().subsonic_response;

        if let Some(GetArtistInfo2SuccessResponse(artist_info)) = artist_info {
            artist.bio = artist_info.artist_info2.biography;

            artist.similar = artist_info
                .artist_info2
                .similar_artist
                .into_iter()
                .map(Artist::from)
                .collect();
        }

        Ok(artist)
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
                    .map(SearchResult::from)
                    .chain(
                        response
                            .search_result3
                            .artist
                            .into_iter()
                            .map(SearchResult::from)
                            .chain(
                                response
                                    .search_result3
                                    .song
                                    .into_iter()
                                    .map(SearchResult::from),
                            ),
                    )
                    .collect();

                Ok(results)
            }
            Search3ResponseSubsonicResponse::SubsonicFailureResponse(failure) => {
                Err(ClientError::Failure(failure))
            }
        }
    }

    pub async fn get_public_playlist(&self) -> Result<Vec<PlaylistInfo>, ClientError> {
        let response = self
            .client
            .get_playlists(None)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            GetPlaylistsSuccessResponse(playlist) => Ok(playlist
                .playlists
                .playlist
                .into_iter()
                .map(PlaylistInfo::from)
                .collect()),
            GetPlaylistsResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }

    pub async fn get_random_songs(&self, limit: i64) -> Result<Vec<Song>, ClientError> {
        let response = self
            .client
            .get_random_songs(None, None, None, Some(limit), None)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            GetRandomSongsResponseSubsonicResponse::GetRandomSongsSuccessResponse(songs) => {
                Ok(songs
                    .random_songs
                    .song
                    .into_iter()
                    .map(Song::from)
                    .collect())
            }
            GetRandomSongsResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }

    pub async fn get_starred(&self) -> Result<Starred, ClientError> {
        let response = self
            .client
            .get_starred2(None)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            GetStarred2ResponseSubsonicResponse::GetStarred2SuccessResponse(response) => {
                let starred = Starred {
                    songs: response.starred2.song.into_iter().map(Song::from).collect(),
                    albums: response
                        .starred2
                        .album
                        .into_iter()
                        .map(Album::from)
                        .collect(),
                    artists: response
                        .starred2
                        .artist
                        .into_iter()
                        .map(Artist::from)
                        .collect(),
                };

                Ok(starred)
            }
            GetStarred2ResponseSubsonicResponse::SubsonicFailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }

    pub async fn star(&self, star: Star) -> Result<(), ClientError> {
        let (album_id, artist_id, id) = match star {
            Star::Album(album_id) => (Some(&vec![album_id]), None, None),
            Star::Song(id) => (None, None, Some(&vec![id])),
            Star::Artist(artist_id) => (None, Some(&vec![artist_id]), None),
        };

        let response = self
            .client
            .star(album_id, artist_id, id)
            .await
            .map(|response| response.into_inner())
            .map_err(ClientError::OpenSubSonic)?;

        let response = response
            .subsonic_response
            .ok_or_else(|| ClientError::Other("Empty response".to_string()))?;

        match response {
            opensubsonic_cli::types::SubsonicResponseSubsonicResponse::SuccessResponse(_) => Ok(()),
            opensubsonic_cli::types::SubsonicResponseSubsonicResponse::FailureResponse(
                subsonic_failure_response,
            ) => Err(ClientError::Failure(subsonic_failure_response)),
        }
    }
}

pub enum Star {
    Album(String),
    Song(String),
    Artist(String),
}
