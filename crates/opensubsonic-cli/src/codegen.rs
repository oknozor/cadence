#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///Album with songs.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Album with songs.",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "songCount"
    ///  ],
    ///  "properties": {
    ///    "artist": {
    ///      "description": "Artist name.",
    ///      "type": "string"
    ///    },
    ///    "artistId": {
    ///      "description": "The id of the artist",
    ///      "type": "string"
    ///    },
    ///    "artists": {
    ///      "description": "The list of all album artists of the album.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    },
    ///    "coverArt": {
    ///      "description": "A covertArt id.",
    ///      "type": "string"
    ///    },
    ///    "created": {
    ///      "description": "Date the album was added. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "discTitles": {
    ///      "description": "The list of all disc titles of the album.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DiscTitle"
    ///      }
    ///    },
    ///    "displayArtist": {
    ///      "description": "The single value display artist.",
    ///      "type": "string"
    ///    },
    ///    "duration": {
    ///      "description": "Total duration of the album in seconds",
    ///      "type": "integer"
    ///    },
    ///    "explicitStatus": {
    ///      "$ref": "#/components/schemas/ExplicitStatus"
    ///    },
    ///    "genre": {
    ///      "description": "The album genre",
    ///      "type": "string"
    ///    },
    ///    "genres": {
    ///      "description": "The list of all genres of the album.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ItemGenre"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "The id of the album",
    ///      "type": "string"
    ///    },
    ///    "isCompilation": {
    ///      "description": "True if the album is a compilation.",
    ///      "type": "boolean"
    ///    },
    ///    "moods": {
    ///      "description": "The list of all moods of the album.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "musicBrainzId": {
    ///      "description": "The album MusicBrainzID.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The album name.",
    ///      "type": "string"
    ///    },
    ///    "originalReleaseDate": {
    ///      "description": "Date the album was originally released.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ItemDate"
    ///        }
    ///      ]
    ///    },
    ///    "playCount": {
    ///      "description": "Number of play of the album",
    ///      "type": "integer"
    ///    },
    ///    "played": {
    ///      "description": "Date the album was last played. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "recordLabels": {
    ///      "description": "The labels producing the album.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RecordLabel"
    ///      }
    ///    },
    ///    "releaseDate": {
    ///      "description": "Date the specific edition of the album was released. Note: for files using ID3 tags, releaseDate should generally be read from the TDRL tag. Servers that use a different source for this field should document the behavior.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ItemDate"
    ///        }
    ///      ]
    ///    },
    ///    "releaseTypes": {
    ///      "description": "The types of this album release. (Album, Compilation, EP, Remix, …).",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "song": {
    ///      "description": "The list of songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "songCount": {
    ///      "description": "Number of songs",
    ///      "type": "integer"
    ///    },
    ///    "sortName": {
    ///      "description": "The album sort name.",
    ///      "type": "string"
    ///    },
    ///    "starred": {
    ///      "description": "Date the album was added. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "userRating": {
    ///      "description": "The user rating of the album. [1-5]",
    ///      "type": "integer",
    ///      "maximum": 5.0,
    ///      "minimum": 0.0
    ///    },
    ///    "version": {
    ///      "description": "The album version name (Remastered, Anniversary Box Set, …).",
    ///      "type": "string"
    ///    },
    ///    "year": {
    ///      "description": "The album year",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlbumId3 {
        ///Artist name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The id of the artist
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_id: ::std::option::Option<::std::string::String>,
        ///The list of all album artists of the album.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artists: ::std::vec::Vec<ArtistId3>,
        ///A covertArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Date the album was added. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The list of all disc titles of the album.
        #[serde(
            rename = "discTitles",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub disc_titles: ::std::vec::Vec<DiscTitle>,
        ///The single value display artist.
        #[serde(
            rename = "displayArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_artist: ::std::option::Option<::std::string::String>,
        ///Total duration of the album in seconds
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<i64>,
        #[serde(
            rename = "explicitStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub explicit_status: ::std::option::Option<ExplicitStatus>,
        ///The album genre
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///The list of all genres of the album.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub genres: ::std::vec::Vec<ItemGenre>,
        ///The id of the album
        pub id: ::std::string::String,
        ///True if the album is a compilation.
        #[serde(
            rename = "isCompilation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_compilation: ::std::option::Option<bool>,
        ///The list of all moods of the album.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub moods: ::std::vec::Vec<::std::string::String>,
        ///The album MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The album name.
        pub name: ::std::string::String,
        ///Date the album was originally released.
        #[serde(
            rename = "originalReleaseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_release_date: ::std::option::Option<ItemDate>,
        ///Number of play of the album
        #[serde(
            rename = "playCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub play_count: ::std::option::Option<i64>,
        ///Date the album was last played. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub played: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The labels producing the album.
        #[serde(
            rename = "recordLabels",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub record_labels: ::std::vec::Vec<RecordLabel>,
        ///Date the specific edition of the album was released. Note: for files using ID3 tags, releaseDate should generally be read from the TDRL tag. Servers that use a different source for this field should document the behavior.
        #[serde(
            rename = "releaseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub release_date: ::std::option::Option<ItemDate>,
        ///The types of this album release. (Album, Compilation, EP, Remix, …).
        #[serde(
            rename = "releaseTypes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub release_types: ::std::vec::Vec<::std::string::String>,
        ///The list of songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
        ///Number of songs
        #[serde(rename = "songCount")]
        pub song_count: i64,
        ///The album sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the album was added. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The user rating of the album. [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
        ///The album version name (Remastered, Anniversary Box Set, …).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
        ///The album year
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&AlbumId3> for AlbumId3 {
        fn from(value: &AlbumId3) -> Self {
            value.clone()
        }
    }
    ///`AlbumId3WithSongs`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AlbumID3"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "song"
    ///      ],
    ///      "properties": {
    ///        "song": {
    ///          "description": "The list of songs",
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/Child"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlbumId3WithSongs {
        ///Artist name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The id of the artist
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_id: ::std::option::Option<::std::string::String>,
        ///The list of all album artists of the album.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artists: ::std::vec::Vec<ArtistId3>,
        ///A covertArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Date the album was added. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The list of all disc titles of the album.
        #[serde(
            rename = "discTitles",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub disc_titles: ::std::vec::Vec<DiscTitle>,
        ///The single value display artist.
        #[serde(
            rename = "displayArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_artist: ::std::option::Option<::std::string::String>,
        ///Total duration of the album in seconds
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<i64>,
        #[serde(
            rename = "explicitStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub explicit_status: ::std::option::Option<ExplicitStatus>,
        ///The album genre
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///The list of all genres of the album.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub genres: ::std::vec::Vec<ItemGenre>,
        ///The id of the album
        pub id: ::std::string::String,
        ///True if the album is a compilation.
        #[serde(
            rename = "isCompilation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_compilation: ::std::option::Option<bool>,
        ///The list of all moods of the album.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub moods: ::std::vec::Vec<::std::string::String>,
        ///The album MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The album name.
        pub name: ::std::string::String,
        ///Date the album was originally released.
        #[serde(
            rename = "originalReleaseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_release_date: ::std::option::Option<ItemDate>,
        ///Number of play of the album
        #[serde(
            rename = "playCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub play_count: ::std::option::Option<i64>,
        ///Date the album was last played. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub played: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The labels producing the album.
        #[serde(
            rename = "recordLabels",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub record_labels: ::std::vec::Vec<RecordLabel>,
        ///Date the specific edition of the album was released. Note: for files using ID3 tags, releaseDate should generally be read from the TDRL tag. Servers that use a different source for this field should document the behavior.
        #[serde(
            rename = "releaseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub release_date: ::std::option::Option<ItemDate>,
        ///The types of this album release. (Album, Compilation, EP, Remix, …).
        #[serde(
            rename = "releaseTypes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub release_types: ::std::vec::Vec<::std::string::String>,
        pub song: ::std::vec::Vec<Child>,
        ///Number of songs
        #[serde(rename = "songCount")]
        pub song_count: i64,
        ///The album sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the album was added. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The user rating of the album. [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
        ///The album version name (Remastered, Anniversary Box Set, …).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
        ///The album year
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&AlbumId3WithSongs> for AlbumId3WithSongs {
        fn from(value: &AlbumId3WithSongs) -> Self {
            value.clone()
        }
    }
    ///Album info.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Album info.",
    ///  "type": "object",
    ///  "properties": {
    ///    "largeImageUrl": {
    ///      "description": "Album largeImageUrl",
    ///      "type": "string"
    ///    },
    ///    "lastFmUrl": {
    ///      "description": "Album lastFmUrl",
    ///      "type": "string"
    ///    },
    ///    "mediumImageUrl": {
    ///      "description": "Album mediumImageUrl",
    ///      "type": "string"
    ///    },
    ///    "musicBrainzId": {
    ///      "description": "Album musicBrainzId",
    ///      "type": "string"
    ///    },
    ///    "notes": {
    ///      "description": "Album notes",
    ///      "type": "string"
    ///    },
    ///    "smallImageUrl": {
    ///      "description": "Album smallImageUrl",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlbumInfo {
        ///Album largeImageUrl
        #[serde(
            rename = "largeImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub large_image_url: ::std::option::Option<::std::string::String>,
        ///Album lastFmUrl
        #[serde(
            rename = "lastFmUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_fm_url: ::std::option::Option<::std::string::String>,
        ///Album mediumImageUrl
        #[serde(
            rename = "mediumImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub medium_image_url: ::std::option::Option<::std::string::String>,
        ///Album musicBrainzId
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///Album notes
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub notes: ::std::option::Option<::std::string::String>,
        ///Album smallImageUrl
        #[serde(
            rename = "smallImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub small_image_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&AlbumInfo> for AlbumInfo {
        fn from(value: &AlbumInfo) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for AlbumInfo {
        fn default() -> Self {
            Self {
                large_image_url: Default::default(),
                last_fm_url: Default::default(),
                medium_image_url: Default::default(),
                music_brainz_id: Default::default(),
                notes: Default::default(),
                small_image_url: Default::default(),
            }
        }
    }
    ///Album list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Album list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Artist albums",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlbumList {
        ///Artist albums
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub album: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&AlbumList> for AlbumList {
        fn from(value: &AlbumList) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for AlbumList {
        fn default() -> Self {
            Self {
                album: Default::default(),
            }
        }
    }
    ///`AlbumList2`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Artist albums",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AlbumID3"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlbumList2 {
        ///Artist albums
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub album: ::std::vec::Vec<AlbumId3>,
    }
    impl ::std::convert::From<&AlbumList2> for AlbumList2 {
        fn from(value: &AlbumList2) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for AlbumList2 {
        fn default() -> Self {
            Self {
                album: Default::default(),
            }
        }
    }
    ///The list type. Must be one of the following: random, newest, highest, frequent, recent. Since 1.8.0 you can also use alphabeticalByName or alphabeticalByArtist to page through all albums alphabetically, and starred to retrieve starred albums. Since 1.10.1 you can use byYear and byGenre to list albums in a given year range or genre.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The list type. Must be one of the following: random, newest, highest, frequent, recent. Since 1.8.0 you can also use alphabeticalByName or alphabeticalByArtist to page through all albums alphabetically, and starred to retrieve starred albums. Since 1.10.1 you can use byYear and byGenre to list albums in a given year range or genre.",
    ///  "type": "string",
    ///  "enum": [
    ///    "random",
    ///    "newest",
    ///    "highest",
    ///    "frequent",
    ///    "recent",
    ///    "alphabeticalByName",
    ///    "alphabeticalByArtist",
    ///    "starred",
    ///    "byYear",
    ///    "byGenre"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AlbumListType {
        #[serde(rename = "random")]
        Random,
        #[serde(rename = "newest")]
        Newest,
        #[serde(rename = "highest")]
        Highest,
        #[serde(rename = "frequent")]
        Frequent,
        #[serde(rename = "recent")]
        Recent,
        #[serde(rename = "alphabeticalByName")]
        AlphabeticalByName,
        #[serde(rename = "alphabeticalByArtist")]
        AlphabeticalByArtist,
        #[serde(rename = "starred")]
        Starred,
        #[serde(rename = "byYear")]
        ByYear,
        #[serde(rename = "byGenre")]
        ByGenre,
    }
    impl ::std::convert::From<&Self> for AlbumListType {
        fn from(value: &AlbumListType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for AlbumListType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Random => f.write_str("random"),
                Self::Newest => f.write_str("newest"),
                Self::Highest => f.write_str("highest"),
                Self::Frequent => f.write_str("frequent"),
                Self::Recent => f.write_str("recent"),
                Self::AlphabeticalByName => f.write_str("alphabeticalByName"),
                Self::AlphabeticalByArtist => f.write_str("alphabeticalByArtist"),
                Self::Starred => f.write_str("starred"),
                Self::ByYear => f.write_str("byYear"),
                Self::ByGenre => f.write_str("byGenre"),
            }
        }
    }
    impl ::std::str::FromStr for AlbumListType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "random" => Ok(Self::Random),
                "newest" => Ok(Self::Newest),
                "highest" => Ok(Self::Highest),
                "frequent" => Ok(Self::Frequent),
                "recent" => Ok(Self::Recent),
                "alphabeticalByName" => Ok(Self::AlphabeticalByName),
                "alphabeticalByArtist" => Ok(Self::AlphabeticalByArtist),
                "starred" => Ok(Self::Starred),
                "byYear" => Ok(Self::ByYear),
                "byGenre" => Ok(Self::ByGenre),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AlbumListType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for AlbumListType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for AlbumListType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Artist details.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Artist details.",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "artistImageUrl": {
    ///      "description": "Artist image url",
    ///      "type": "string"
    ///    },
    ///    "averageRating": {
    ///      "description": "Artist average rating [1.0-5.0]",
    ///      "type": "number",
    ///      "maximum": 5.0,
    ///      "minimum": 1.0
    ///    },
    ///    "id": {
    ///      "description": "Artist id",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Artist name",
    ///      "type": "string"
    ///    },
    ///    "starred": {
    ///      "description": "Artist starred date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "userRating": {
    ///      "description": "Artist rating [1-5]",
    ///      "type": "integer",
    ///      "maximum": 5.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Artist {
        ///Artist image url
        #[serde(
            rename = "artistImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_image_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "averageRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_rating: ::std::option::Option<f64>,
        ///Artist id
        pub id: ::std::string::String,
        ///Artist name
        pub name: ::std::string::String,
        ///Artist starred date [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Artist rating [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&Artist> for Artist {
        fn from(value: &Artist) -> Self {
            value.clone()
        }
    }
    ///An artist from ID3 tags.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An artist from ID3 tags.",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "albumCount": {
    ///      "description": "Artist album count.",
    ///      "type": "integer"
    ///    },
    ///    "artistImageUrl": {
    ///      "description": "An url to an external image source.",
    ///      "type": "string"
    ///    },
    ///    "coverArt": {
    ///      "description": "A covertArt id.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "The id of the artist",
    ///      "type": "string"
    ///    },
    ///    "musicBrainzId": {
    ///      "description": "The artist MusicBrainzID.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The artist name.",
    ///      "type": "string"
    ///    },
    ///    "roles": {
    ///      "description": "The list of all roles this artist has in the library.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "sortName": {
    ///      "description": "The artist sort name.",
    ///      "type": "string"
    ///    },
    ///    "starred": {
    ///      "description": "Date the artist was starred. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ArtistId3 {
        ///Artist album count.
        #[serde(
            rename = "albumCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_count: ::std::option::Option<i64>,
        ///An url to an external image source.
        #[serde(
            rename = "artistImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_image_url: ::std::option::Option<::std::string::String>,
        ///A covertArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///The id of the artist
        pub id: ::std::string::String,
        ///The artist MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The artist name.
        pub name: ::std::string::String,
        ///The list of all roles this artist has in the library.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub roles: ::std::vec::Vec<::std::string::String>,
        ///The artist sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the artist was starred. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }
    impl ::std::convert::From<&ArtistId3> for ArtistId3 {
        fn from(value: &ArtistId3) -> Self {
            value.clone()
        }
    }
    ///Artist info.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Artist info.",
    ///  "type": "object",
    ///  "properties": {
    ///    "biography": {
    ///      "description": "Artist biography",
    ///      "type": "string"
    ///    },
    ///    "largeImageUrl": {
    ///      "description": "Artist largeImageUrl",
    ///      "type": "string"
    ///    },
    ///    "lastFmUrl": {
    ///      "description": "Artist lastFmUrl",
    ///      "type": "string"
    ///    },
    ///    "mediumImageUrl": {
    ///      "description": "Artist mediumImageUrl",
    ///      "type": "string"
    ///    },
    ///    "musicBrainzId": {
    ///      "description": "Artist musicBrainzId",
    ///      "type": "string"
    ///    },
    ///    "similarArtist": {
    ///      "description": "Similar artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Artist"
    ///      }
    ///    },
    ///    "smallImageUrl": {
    ///      "description": "Artist smallImageUrl",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ArtistInfo {
        ///Artist biography
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub biography: ::std::option::Option<::std::string::String>,
        ///Artist largeImageUrl
        #[serde(
            rename = "largeImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub large_image_url: ::std::option::Option<::std::string::String>,
        ///Artist lastFmUrl
        #[serde(
            rename = "lastFmUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_fm_url: ::std::option::Option<::std::string::String>,
        ///Artist mediumImageUrl
        #[serde(
            rename = "mediumImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub medium_image_url: ::std::option::Option<::std::string::String>,
        ///Artist musicBrainzId
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///Similar artists
        #[serde(
            rename = "similarArtist",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub similar_artist: ::std::vec::Vec<Artist>,
        ///Artist smallImageUrl
        #[serde(
            rename = "smallImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub small_image_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ArtistInfo> for ArtistInfo {
        fn from(value: &ArtistInfo) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ArtistInfo {
        fn default() -> Self {
            Self {
                biography: Default::default(),
                large_image_url: Default::default(),
                last_fm_url: Default::default(),
                medium_image_url: Default::default(),
                music_brainz_id: Default::default(),
                similar_artist: Default::default(),
                small_image_url: Default::default(),
            }
        }
    }
    ///Artist info.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Artist info.",
    ///  "type": "object",
    ///  "properties": {
    ///    "biography": {
    ///      "description": "Artist biography",
    ///      "type": "string"
    ///    },
    ///    "largeImageUrl": {
    ///      "description": "Artist largeImageUrl",
    ///      "type": "string"
    ///    },
    ///    "lastFmUrl": {
    ///      "description": "Artist lastFmUrl",
    ///      "type": "string"
    ///    },
    ///    "mediumImageUrl": {
    ///      "description": "Artist mediumImageUrl",
    ///      "type": "string"
    ///    },
    ///    "musicBrainzId": {
    ///      "description": "Artist musicBrainzId",
    ///      "type": "string"
    ///    },
    ///    "similarArtist": {
    ///      "description": "Similar artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    },
    ///    "smallImageUrl": {
    ///      "description": "Artist smallImageUrl",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ArtistInfo2 {
        ///Artist biography
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub biography: ::std::option::Option<::std::string::String>,
        ///Artist largeImageUrl
        #[serde(
            rename = "largeImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub large_image_url: ::std::option::Option<::std::string::String>,
        ///Artist lastFmUrl
        #[serde(
            rename = "lastFmUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_fm_url: ::std::option::Option<::std::string::String>,
        ///Artist mediumImageUrl
        #[serde(
            rename = "mediumImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub medium_image_url: ::std::option::Option<::std::string::String>,
        ///Artist musicBrainzId
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///Similar artists
        #[serde(
            rename = "similarArtist",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub similar_artist: ::std::vec::Vec<ArtistId3>,
        ///Artist smallImageUrl
        #[serde(
            rename = "smallImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub small_image_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ArtistInfo2> for ArtistInfo2 {
        fn from(value: &ArtistInfo2) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ArtistInfo2 {
        fn default() -> Self {
            Self {
                biography: Default::default(),
                large_image_url: Default::default(),
                last_fm_url: Default::default(),
                medium_image_url: Default::default(),
                music_brainz_id: Default::default(),
                similar_artist: Default::default(),
                small_image_url: Default::default(),
            }
        }
    }
    ///`ArtistWithAlbumsId3`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/ArtistID3"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "album"
    ///      ],
    ///      "properties": {
    ///        "album": {
    ///          "description": "Artist albums",
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/AlbumID3"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ArtistWithAlbumsId3 {
        ///Artist albums
        pub album: ::std::vec::Vec<AlbumId3>,
        ///Artist album count.
        #[serde(
            rename = "albumCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_count: ::std::option::Option<i64>,
        ///An url to an external image source.
        #[serde(
            rename = "artistImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_image_url: ::std::option::Option<::std::string::String>,
        ///A covertArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///The id of the artist
        pub id: ::std::string::String,
        ///The artist MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The artist name.
        pub name: ::std::string::String,
        ///The list of all roles this artist has in the library.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub roles: ::std::vec::Vec<::std::string::String>,
        ///The artist sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the artist was starred. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }
    impl ::std::convert::From<&ArtistWithAlbumsId3> for ArtistWithAlbumsId3 {
        fn from(value: &ArtistWithAlbumsId3) -> Self {
            value.clone()
        }
    }
    ///A list of indexed Artists.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A list of indexed Artists.",
    ///  "type": "object",
    ///  "properties": {
    ///    "ignoredArticles": {
    ///      "description": "List of ignored articles space separated",
    ///      "type": "string"
    ///    },
    ///    "index": {
    ///      "description": "Index list",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ArtistsId3 {
        ///List of ignored articles space separated
        #[serde(
            rename = "ignoredArticles",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ignored_articles: ::std::option::Option<::std::string::String>,
        ///Index list
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub index: ::std::vec::Vec<ArtistId3>,
    }
    impl ::std::convert::From<&ArtistsId3> for ArtistsId3 {
        fn from(value: &ArtistsId3) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ArtistsId3 {
        fn default() -> Self {
            Self {
                ignored_articles: Default::default(),
                index: Default::default(),
            }
        }
    }
    ///A bookmark.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A bookmark.",
    ///  "type": "object",
    ///  "required": [
    ///    "changed",
    ///    "created",
    ///    "entry",
    ///    "position",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "changed": {
    ///      "description": "Bookmark last updated date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "comment": {
    ///      "description": "Bookmark comment",
    ///      "type": "string"
    ///    },
    ///    "created": {
    ///      "description": "Bookmark creation date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "entry": {
    ///      "description": "The bookmark file",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Child"
    ///        }
    ///      ]
    ///    },
    ///    "position": {
    ///      "description": "Bookmark position in milliseconds",
    ///      "type": "integer"
    ///    },
    ///    "username": {
    ///      "description": "Username",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Bookmark {
        ///Bookmark last updated date [ISO 8601]
        pub changed: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Bookmark comment
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///Bookmark creation date [ISO 8601]
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///The bookmark file
        pub entry: Child,
        ///Bookmark position in milliseconds
        pub position: i64,
        ///Username
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&Bookmark> for Bookmark {
        fn from(value: &Bookmark) -> Self {
            value.clone()
        }
    }
    ///Bookmarks list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Bookmarks list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "bookmark": {
    ///      "description": "List of bookmark",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Bookmark"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Bookmarks {
        ///List of bookmark
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub bookmark: ::std::vec::Vec<Bookmark>,
    }
    impl ::std::convert::From<&Bookmarks> for Bookmarks {
        fn from(value: &Bookmarks) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Bookmarks {
        fn default() -> Self {
            Self {
                bookmark: Default::default(),
            }
        }
    }
    ///A chatMessage.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A chatMessage.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "time",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "The message",
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "description": "Time in millis since Jan 1 1970",
    ///      "type": "integer"
    ///    },
    ///    "username": {
    ///      "description": "Username",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChatMessage {
        ///The message
        pub message: ::std::string::String,
        ///Time in millis since Jan 1 1970
        pub time: i64,
        ///Username
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&ChatMessage> for ChatMessage {
        fn from(value: &ChatMessage) -> Self {
            value.clone()
        }
    }
    ///Chat messages list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Chat messages list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "chatMessage": {
    ///      "description": "List of chatMessage",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ChatMessage"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChatMessages {
        ///List of chatMessage
        #[serde(
            rename = "chatMessage",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub chat_message: ::std::vec::Vec<ChatMessage>,
    }
    impl ::std::convert::From<&ChatMessages> for ChatMessages {
        fn from(value: &ChatMessages) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ChatMessages {
        fn default() -> Self {
            Self {
                chat_message: Default::default(),
            }
        }
    }
    ///A media.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A media.",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "isDir",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "album": {
    ///      "description": "The album name.",
    ///      "type": "string"
    ///    },
    ///    "albumArtists": {
    ///      "description": "The list of all album artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    },
    ///    "albumId": {
    ///      "description": "The corresponding album id",
    ///      "type": "string"
    ///    },
    ///    "artist": {
    ///      "description": "The artist name.",
    ///      "type": "string"
    ///    },
    ///    "artistId": {
    ///      "description": "The corresponding artist id",
    ///      "type": "string"
    ///    },
    ///    "artists": {
    ///      "description": "The list of all song artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    },
    ///    "averageRating": {
    ///      "description": "The average rating of the media [1.0-5.0]",
    ///      "type": "number",
    ///      "maximum": 5.0,
    ///      "minimum": 0.0
    ///    },
    ///    "bitDepth": {
    ///      "description": "The bit depth of the media.",
    ///      "type": "integer"
    ///    },
    ///    "bitRate": {
    ///      "description": "The bitrate of the media.",
    ///      "type": "integer"
    ///    },
    ///    "bookmarkPosition": {
    ///      "description": "The bookmark position in seconds",
    ///      "type": "integer"
    ///    },
    ///    "bpm": {
    ///      "description": "The BPM of the song.",
    ///      "type": "integer"
    ///    },
    ///    "channelCount": {
    ///      "description": "The number of channels of the media.",
    ///      "type": "integer"
    ///    },
    ///    "comment": {
    ///      "description": "The comment tag of the song.",
    ///      "type": "string"
    ///    },
    ///    "contentType": {
    ///      "description": "The mimeType of the media.",
    ///      "type": "string"
    ///    },
    ///    "contributors": {
    ///      "description": "The list of all contributor artists of the song.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Contributor"
    ///      }
    ///    },
    ///    "coverArt": {
    ///      "description": "The coverArt id.",
    ///      "type": "string"
    ///    },
    ///    "created": {
    ///      "description": "Date the media was created. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "discNumber": {
    ///      "description": "The disc number.",
    ///      "type": "integer"
    ///    },
    ///    "displayAlbumArtist": {
    ///      "description": "The single value display album artist.",
    ///      "type": "string"
    ///    },
    ///    "displayArtist": {
    ///      "description": "The single value display artist.",
    ///      "type": "string"
    ///    },
    ///    "displayComposer": {
    ///      "description": "The single value display composer.",
    ///      "type": "string"
    ///    },
    ///    "duration": {
    ///      "description": "The duration of the media in seconds.",
    ///      "type": "integer"
    ///    },
    ///    "explicitStatus": {
    ///      "$ref": "#/components/schemas/ExplicitStatus"
    ///    },
    ///    "genre": {
    ///      "description": "The media genre",
    ///      "type": "string"
    ///    },
    ///    "genres": {
    ///      "description": "The list of all genres of the song.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ItemGenre"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "The id of the media.",
    ///      "type": "string"
    ///    },
    ///    "isDir": {
    ///      "description": "The media is a directory",
    ///      "type": "boolean"
    ///    },
    ///    "isVideo": {
    ///      "description": "Media is a video",
    ///      "type": "boolean"
    ///    },
    ///    "isrc": {
    ///      "description": "The track ISRC(s).",
    ///      "examples": [
    ///        [
    ///          "USSM18300073",
    ///          "DELV42300297"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "mediaType": {
    ///      "$ref": "#/components/schemas/MediaType"
    ///    },
    ///    "moods": {
    ///      "description": "The list of all moods of the song.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "musicBrainzId": {
    ///      "description": "The track MusicBrainzID.",
    ///      "type": "string"
    ///    },
    ///    "originalHeight": {
    ///      "description": "The video original Height",
    ///      "type": "integer"
    ///    },
    ///    "originalWidth": {
    ///      "description": "The video original Width",
    ///      "type": "integer"
    ///    },
    ///    "parent": {
    ///      "description": "The id of the parent (folder/album)",
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "description": "The full path of the media.",
    ///      "type": "string"
    ///    },
    ///    "playCount": {
    ///      "description": "The play count.",
    ///      "type": "integer"
    ///    },
    ///    "played": {
    ///      "description": "Date the album was last played. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "replayGain": {
    ///      "description": "The replay gain data of the song.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ReplayGain"
    ///        }
    ///      ]
    ///    },
    ///    "samplingRate": {
    ///      "description": "The sampling rate of the media.",
    ///      "type": "integer"
    ///    },
    ///    "size": {
    ///      "description": "A file size of the media.",
    ///      "type": "integer"
    ///    },
    ///    "sortName": {
    ///      "description": "The song sort name.",
    ///      "type": "string"
    ///    },
    ///    "starred": {
    ///      "description": "Date the media was starred. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "suffix": {
    ///      "description": "The file suffix of the media.",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "The media name.",
    ///      "type": "string"
    ///    },
    ///    "track": {
    ///      "description": "The track number.",
    ///      "type": "integer"
    ///    },
    ///    "transcodedContentType": {
    ///      "description": "The transcoded mediaType if transcoding should happen.",
    ///      "type": "string"
    ///    },
    ///    "transcodedSuffix": {
    ///      "description": "The file suffix of the transcoded media.",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "$ref": "#/components/schemas/GenericMediaType"
    ///    },
    ///    "userRating": {
    ///      "description": "The user rating of the media [1-5]",
    ///      "type": "integer",
    ///      "maximum": 5.0,
    ///      "minimum": 0.0
    ///    },
    ///    "year": {
    ///      "description": "The media year.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Child {
        ///The album name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub album: ::std::option::Option<::std::string::String>,
        ///The list of all album artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)
        #[serde(
            rename = "albumArtists",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub album_artists: ::std::vec::Vec<ArtistId3>,
        ///The corresponding album id
        #[serde(
            rename = "albumId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_id: ::std::option::Option<::std::string::String>,
        ///The artist name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The corresponding artist id
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_id: ::std::option::Option<::std::string::String>,
        ///The list of all song artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artists: ::std::vec::Vec<ArtistId3>,
        #[serde(
            rename = "averageRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_rating: ::std::option::Option<f64>,
        ///The bit depth of the media.
        #[serde(
            rename = "bitDepth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_depth: ::std::option::Option<i64>,
        ///The bitrate of the media.
        #[serde(
            rename = "bitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_rate: ::std::option::Option<i64>,
        ///The bookmark position in seconds
        #[serde(
            rename = "bookmarkPosition",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bookmark_position: ::std::option::Option<i64>,
        ///The BPM of the song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bpm: ::std::option::Option<i64>,
        ///The number of channels of the media.
        #[serde(
            rename = "channelCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub channel_count: ::std::option::Option<i64>,
        ///The comment tag of the song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///The mimeType of the media.
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<::std::string::String>,
        ///The list of all contributor artists of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub contributors: ::std::vec::Vec<Contributor>,
        ///The coverArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Date the media was created. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The disc number.
        #[serde(
            rename = "discNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disc_number: ::std::option::Option<i64>,
        ///The single value display album artist.
        #[serde(
            rename = "displayAlbumArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_album_artist: ::std::option::Option<::std::string::String>,
        ///The single value display artist.
        #[serde(
            rename = "displayArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_artist: ::std::option::Option<::std::string::String>,
        ///The single value display composer.
        #[serde(
            rename = "displayComposer",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_composer: ::std::option::Option<::std::string::String>,
        ///The duration of the media in seconds.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<i64>,
        #[serde(
            rename = "explicitStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub explicit_status: ::std::option::Option<ExplicitStatus>,
        ///The media genre
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///The list of all genres of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub genres: ::std::vec::Vec<ItemGenre>,
        ///The id of the media.
        pub id: ::std::string::String,
        ///The media is a directory
        #[serde(rename = "isDir")]
        pub is_dir: bool,
        ///Media is a video
        #[serde(
            rename = "isVideo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_video: ::std::option::Option<bool>,
        ///The track ISRC(s).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub isrc: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "mediaType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub media_type: ::std::option::Option<MediaType>,
        ///The list of all moods of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub moods: ::std::vec::Vec<::std::string::String>,
        ///The track MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The video original Height
        #[serde(
            rename = "originalHeight",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_height: ::std::option::Option<i64>,
        ///The video original Width
        #[serde(
            rename = "originalWidth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_width: ::std::option::Option<i64>,
        ///The id of the parent (folder/album)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<::std::string::String>,
        ///The full path of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        ///The play count.
        #[serde(
            rename = "playCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub play_count: ::std::option::Option<i64>,
        ///Date the album was last played. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub played: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The replay gain data of the song.
        #[serde(
            rename = "replayGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub replay_gain: ::std::option::Option<ReplayGain>,
        ///The sampling rate of the media.
        #[serde(
            rename = "samplingRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sampling_rate: ::std::option::Option<i64>,
        ///A file size of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
        ///The song sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the media was starred. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The file suffix of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suffix: ::std::option::Option<::std::string::String>,
        ///The media name.
        pub title: ::std::string::String,
        ///The track number.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub track: ::std::option::Option<i64>,
        ///The transcoded mediaType if transcoding should happen.
        #[serde(
            rename = "transcodedContentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transcoded_content_type: ::std::option::Option<::std::string::String>,
        ///The file suffix of the transcoded media.
        #[serde(
            rename = "transcodedSuffix",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transcoded_suffix: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<GenericMediaType>,
        ///The user rating of the media [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
        ///The media year.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&Child> for Child {
        fn from(value: &Child) -> Self {
            value.clone()
        }
    }
    ///A contributor artist for a song or an album
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A contributor artist for a song or an album",
    ///  "type": "object",
    ///  "required": [
    ///    "artist",
    ///    "role"
    ///  ],
    ///  "properties": {
    ///    "artist": {
    ///      "$ref": "#/components/schemas/ArtistID3"
    ///    },
    ///    "role": {
    ///      "description": "The contributor role.",
    ///      "type": "string"
    ///    },
    ///    "subRole": {
    ///      "description": "The subRole for roles that may require it. Ex: The instrument for the performer role (TMCL/performer tags). Note: For consistency between different tag formats, the TIPL sub roles should be directly exposed in the role field.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Contributor {
        pub artist: ArtistId3,
        ///The contributor role.
        pub role: ::std::string::String,
        ///The subRole for roles that may require it. Ex: The instrument for the performer role (TMCL/performer tags). Note: For consistency between different tag formats, the TIPL sub roles should be directly exposed in the role field.
        #[serde(
            rename = "subRole",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sub_role: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Contributor> for Contributor {
        fn from(value: &Contributor) -> Self {
            value.clone()
        }
    }
    ///A subsonic-response element with a nested playlist element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested playlist element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CreatePlaylistSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreatePlaylistResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<CreatePlaylistResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&CreatePlaylistResponse> for CreatePlaylistResponse {
        fn from(value: &CreatePlaylistResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for CreatePlaylistResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`CreatePlaylistResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/CreatePlaylistSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CreatePlaylistResponseSubsonicResponse {
        CreatePlaylistSuccessResponse(CreatePlaylistSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for CreatePlaylistResponseSubsonicResponse {
        fn from(value: &CreatePlaylistResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<CreatePlaylistSuccessResponse>
        for CreatePlaylistResponseSubsonicResponse
    {
        fn from(value: CreatePlaylistSuccessResponse) -> Self {
            Self::CreatePlaylistSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for CreatePlaylistResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`CreatePlaylistSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "playlist"
    ///      ],
    ///      "properties": {
    ///        "playlist": {
    ///          "$ref": "#/components/schemas/PlaylistWithSongs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreatePlaylistSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        pub playlist: PlaylistWithSongs,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: CreatePlaylistSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&CreatePlaylistSuccessResponse> for CreatePlaylistSuccessResponse {
        fn from(value: &CreatePlaylistSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreatePlaylistSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for CreatePlaylistSuccessResponseStatus {
        fn from(value: &CreatePlaylistSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for CreatePlaylistSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for CreatePlaylistSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreatePlaylistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreatePlaylistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreatePlaylistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested shares element on success. Which in turns contains a single share element for the newly created share
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested shares element on success. Which in turns contains a single share element for the newly created share",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CreateSharesSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSharesResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<CreateSharesResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&CreateSharesResponse> for CreateSharesResponse {
        fn from(value: &CreateSharesResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for CreateSharesResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`CreateSharesResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/CreateSharesSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CreateSharesResponseSubsonicResponse {
        CreateSharesSuccessResponse(CreateSharesSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for CreateSharesResponseSubsonicResponse {
        fn from(value: &CreateSharesResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<CreateSharesSuccessResponse> for CreateSharesResponseSubsonicResponse {
        fn from(value: CreateSharesSuccessResponse) -> Self {
            Self::CreateSharesSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for CreateSharesResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`CreateSharesSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "shares"
    ///      ],
    ///      "properties": {
    ///        "shares": {
    ///          "$ref": "#/components/schemas/Shares"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSharesSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        pub shares: Shares,
        ///The command result. `ok`
        pub status: CreateSharesSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&CreateSharesSuccessResponse> for CreateSharesSuccessResponse {
        fn from(value: &CreateSharesSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateSharesSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for CreateSharesSuccessResponseStatus {
        fn from(value: &CreateSharesSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for CreateSharesSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for CreateSharesSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateSharesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreateSharesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateSharesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Directory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Directory.",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "averageRating": {
    ///      "description": "The average rating [1-5]",
    ///      "type": "number",
    ///      "maximum": 5.0,
    ///      "minimum": 1.0
    ///    },
    ///    "child": {
    ///      "description": "The directory content",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "The id",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The directory name",
    ///      "type": "string"
    ///    },
    ///    "parent": {
    ///      "description": "Parent item",
    ///      "type": "string"
    ///    },
    ///    "playCount": {
    ///      "description": "The play count",
    ///      "type": "integer"
    ///    },
    ///    "starred": {
    ///      "description": "Starred date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "userRating": {
    ///      "description": "The user rating [1-5]",
    ///      "type": "integer",
    ///      "maximum": 5.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Directory {
        #[serde(
            rename = "averageRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_rating: ::std::option::Option<f64>,
        ///The directory content
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub child: ::std::vec::Vec<Child>,
        ///The id
        pub id: ::std::string::String,
        ///The directory name
        pub name: ::std::string::String,
        ///Parent item
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<::std::string::String>,
        ///The play count
        #[serde(
            rename = "playCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub play_count: ::std::option::Option<i64>,
        ///Starred date [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The user rating [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&Directory> for Directory {
        fn from(value: &Directory) -> Self {
            value.clone()
        }
    }
    ///A disc title for an album
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A disc title for an album",
    ///  "type": "object",
    ///  "required": [
    ///    "disc",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "disc": {
    ///      "description": "The disc number.",
    ///      "type": "integer"
    ///    },
    ///    "title": {
    ///      "description": "The name of the disc.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DiscTitle {
        ///The disc number.
        pub disc: i64,
        ///The name of the disc.
        pub title: ::std::string::String,
    }
    impl ::std::convert::From<&DiscTitle> for DiscTitle {
        fn from(value: &DiscTitle) -> Self {
            value.clone()
        }
    }
    ///Returns “explicit”, “clean” or “”. (For songs extracted from tags “ITUNESADVISORY”: 1 = explicit, 2 = clean, MP4 “rtng”: 1 or 4 = explicit, 2 = clean. See `albumID3` for albums)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Returns “explicit”, “clean” or “”. (For songs extracted from tags “ITUNESADVISORY”: 1 = explicit, 2 = clean, MP4 “rtng”: 1 or 4 = explicit, 2 = clean. See `albumID3` for albums)",
    ///  "type": "string",
    ///  "enum": [
    ///    "clean",
    ///    "explicit",
    ///    ""
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ExplicitStatus {
        #[serde(rename = "clean")]
        Clean,
        #[serde(rename = "explicit")]
        Explicit,
        #[serde(rename = "")]
        X,
    }
    impl ::std::convert::From<&Self> for ExplicitStatus {
        fn from(value: &ExplicitStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for ExplicitStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Clean => f.write_str("clean"),
                Self::Explicit => f.write_str("explicit"),
                Self::X => f.write_str(""),
            }
        }
    }
    impl ::std::str::FromStr for ExplicitStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "clean" => Ok(Self::Clean),
                "explicit" => Ok(Self::Explicit),
                "" => Ok(Self::X),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ExplicitStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ExplicitStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ExplicitStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The generic type of media [music/podcast/audiobook/video]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The generic type of media [music/podcast/audiobook/video]",
    ///  "type": "string",
    ///  "enum": [
    ///    "music",
    ///    "video",
    ///    "podcast",
    ///    "audiobook"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GenericMediaType {
        #[serde(rename = "music")]
        Music,
        #[serde(rename = "video")]
        Video,
        #[serde(rename = "podcast")]
        Podcast,
        #[serde(rename = "audiobook")]
        Audiobook,
    }
    impl ::std::convert::From<&Self> for GenericMediaType {
        fn from(value: &GenericMediaType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GenericMediaType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Music => f.write_str("music"),
                Self::Video => f.write_str("video"),
                Self::Podcast => f.write_str("podcast"),
                Self::Audiobook => f.write_str("audiobook"),
            }
        }
    }
    impl ::std::str::FromStr for GenericMediaType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "music" => Ok(Self::Music),
                "video" => Ok(Self::Video),
                "podcast" => Ok(Self::Podcast),
                "audiobook" => Ok(Self::Audiobook),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GenericMediaType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GenericMediaType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GenericMediaType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A genre.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A genre.",
    ///  "type": "object",
    ///  "required": [
    ///    "albumCount",
    ///    "songCount",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "albumCount": {
    ///      "description": "Genre album count",
    ///      "type": "integer"
    ///    },
    ///    "songCount": {
    ///      "description": "Genre song count",
    ///      "type": "integer"
    ///    },
    ///    "value": {
    ///      "description": "Genre name",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Genre {
        ///Genre album count
        #[serde(rename = "albumCount")]
        pub album_count: i64,
        ///Genre song count
        #[serde(rename = "songCount")]
        pub song_count: i64,
        ///Genre name
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&Genre> for Genre {
        fn from(value: &Genre) -> Self {
            value.clone()
        }
    }
    ///Genres list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Genres list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "genre": {
    ///      "description": "List of genre",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Genre"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Genres {
        ///List of genre
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub genre: ::std::vec::Vec<Genre>,
    }
    impl ::std::convert::From<&Genres> for Genres {
        fn from(value: &Genres) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Genres {
        fn default() -> Self {
            Self {
                genre: Default::default(),
            }
        }
    }
    ///A subsonic-response element with a nested albumInfo element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested albumInfo element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetAlbumInfoSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumInfoResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetAlbumInfoResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetAlbumInfoResponse> for GetAlbumInfoResponse {
        fn from(value: &GetAlbumInfoResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetAlbumInfoResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetAlbumInfoResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetAlbumInfoSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetAlbumInfoResponseSubsonicResponse {
        GetAlbumInfoSuccessResponse(GetAlbumInfoSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetAlbumInfoResponseSubsonicResponse {
        fn from(value: &GetAlbumInfoResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetAlbumInfoSuccessResponse> for GetAlbumInfoResponseSubsonicResponse {
        fn from(value: GetAlbumInfoSuccessResponse) -> Self {
            Self::GetAlbumInfoSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetAlbumInfoResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetAlbumInfoSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "albumInfo"
    ///      ],
    ///      "properties": {
    ///        "albumInfo": {
    ///          "$ref": "#/components/schemas/AlbumInfo"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumInfoSuccessResponse {
        #[serde(rename = "albumInfo")]
        pub album_info: AlbumInfo,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetAlbumInfoSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetAlbumInfoSuccessResponse> for GetAlbumInfoSuccessResponse {
        fn from(value: &GetAlbumInfoSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetAlbumInfoSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetAlbumInfoSuccessResponseStatus {
        fn from(value: &GetAlbumInfoSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetAlbumInfoSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetAlbumInfoSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetAlbumInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetAlbumInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetAlbumInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested albumList2 element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested albumList2 element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetAlbumList2SuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumList2Response {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetAlbumList2ResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetAlbumList2Response> for GetAlbumList2Response {
        fn from(value: &GetAlbumList2Response) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetAlbumList2Response {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetAlbumList2ResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetAlbumList2SuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetAlbumList2ResponseSubsonicResponse {
        GetAlbumList2SuccessResponse(GetAlbumList2SuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetAlbumList2ResponseSubsonicResponse {
        fn from(value: &GetAlbumList2ResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetAlbumList2SuccessResponse> for GetAlbumList2ResponseSubsonicResponse {
        fn from(value: GetAlbumList2SuccessResponse) -> Self {
            Self::GetAlbumList2SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetAlbumList2ResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetAlbumList2SuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "albumList2"
    ///      ],
    ///      "properties": {
    ///        "albumList2": {
    ///          "$ref": "#/components/schemas/AlbumList2"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumList2SuccessResponse {
        #[serde(rename = "albumList2")]
        pub album_list2: AlbumList2,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetAlbumList2SuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetAlbumList2SuccessResponse> for GetAlbumList2SuccessResponse {
        fn from(value: &GetAlbumList2SuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetAlbumList2SuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetAlbumList2SuccessResponseStatus {
        fn from(value: &GetAlbumList2SuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetAlbumList2SuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetAlbumList2SuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetAlbumList2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetAlbumList2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetAlbumList2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested albumList element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested albumList element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetAlbumListSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumListResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetAlbumListResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetAlbumListResponse> for GetAlbumListResponse {
        fn from(value: &GetAlbumListResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetAlbumListResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetAlbumListResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetAlbumListSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetAlbumListResponseSubsonicResponse {
        GetAlbumListSuccessResponse(GetAlbumListSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetAlbumListResponseSubsonicResponse {
        fn from(value: &GetAlbumListResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetAlbumListSuccessResponse> for GetAlbumListResponseSubsonicResponse {
        fn from(value: GetAlbumListSuccessResponse) -> Self {
            Self::GetAlbumListSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetAlbumListResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetAlbumListSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "albumList"
    ///      ],
    ///      "properties": {
    ///        "albumList": {
    ///          "$ref": "#/components/schemas/AlbumList"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumListSuccessResponse {
        #[serde(rename = "albumList")]
        pub album_list: AlbumList,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetAlbumListSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetAlbumListSuccessResponse> for GetAlbumListSuccessResponse {
        fn from(value: &GetAlbumListSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetAlbumListSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetAlbumListSuccessResponseStatus {
        fn from(value: &GetAlbumListSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetAlbumListSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetAlbumListSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetAlbumListSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetAlbumListSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetAlbumListSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested album element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested album element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetAlbumSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetAlbumResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetAlbumResponse> for GetAlbumResponse {
        fn from(value: &GetAlbumResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetAlbumResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetAlbumResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetAlbumSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetAlbumResponseSubsonicResponse {
        GetAlbumSuccessResponse(GetAlbumSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetAlbumResponseSubsonicResponse {
        fn from(value: &GetAlbumResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetAlbumSuccessResponse> for GetAlbumResponseSubsonicResponse {
        fn from(value: GetAlbumSuccessResponse) -> Self {
            Self::GetAlbumSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetAlbumResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetAlbumSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "album"
    ///      ],
    ///      "properties": {
    ///        "album": {
    ///          "$ref": "#/components/schemas/AlbumID3WithSongs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetAlbumSuccessResponse {
        pub album: AlbumId3WithSongs,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetAlbumSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetAlbumSuccessResponse> for GetAlbumSuccessResponse {
        fn from(value: &GetAlbumSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetAlbumSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetAlbumSuccessResponseStatus {
        fn from(value: &GetAlbumSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetAlbumSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetAlbumSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetAlbumSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetAlbumSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetAlbumSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested artistInfo2 element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested artistInfo2 element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetArtistInfo2SuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistInfo2Response {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetArtistInfo2ResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetArtistInfo2Response> for GetArtistInfo2Response {
        fn from(value: &GetArtistInfo2Response) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetArtistInfo2Response {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetArtistInfo2ResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetArtistInfo2SuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetArtistInfo2ResponseSubsonicResponse {
        GetArtistInfo2SuccessResponse(GetArtistInfo2SuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetArtistInfo2ResponseSubsonicResponse {
        fn from(value: &GetArtistInfo2ResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetArtistInfo2SuccessResponse>
        for GetArtistInfo2ResponseSubsonicResponse
    {
        fn from(value: GetArtistInfo2SuccessResponse) -> Self {
            Self::GetArtistInfo2SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetArtistInfo2ResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetArtistInfo2SuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "artistInfo2"
    ///      ],
    ///      "properties": {
    ///        "artistInfo2": {
    ///          "$ref": "#/components/schemas/ArtistInfo2"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistInfo2SuccessResponse {
        #[serde(rename = "artistInfo2")]
        pub artist_info2: ArtistInfo2,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetArtistInfo2SuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetArtistInfo2SuccessResponse> for GetArtistInfo2SuccessResponse {
        fn from(value: &GetArtistInfo2SuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetArtistInfo2SuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetArtistInfo2SuccessResponseStatus {
        fn from(value: &GetArtistInfo2SuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetArtistInfo2SuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetArtistInfo2SuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetArtistInfo2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetArtistInfo2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetArtistInfo2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested artistInfo element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested artistInfo element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetArtistInfoSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistInfoResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetArtistInfoResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetArtistInfoResponse> for GetArtistInfoResponse {
        fn from(value: &GetArtistInfoResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetArtistInfoResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetArtistInfoResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetArtistInfoSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetArtistInfoResponseSubsonicResponse {
        GetArtistInfoSuccessResponse(GetArtistInfoSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetArtistInfoResponseSubsonicResponse {
        fn from(value: &GetArtistInfoResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetArtistInfoSuccessResponse> for GetArtistInfoResponseSubsonicResponse {
        fn from(value: GetArtistInfoSuccessResponse) -> Self {
            Self::GetArtistInfoSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetArtistInfoResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetArtistInfoSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "artistInfo"
    ///      ],
    ///      "properties": {
    ///        "artistInfo": {
    ///          "$ref": "#/components/schemas/ArtistInfo"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistInfoSuccessResponse {
        #[serde(rename = "artistInfo")]
        pub artist_info: ArtistInfo,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetArtistInfoSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetArtistInfoSuccessResponse> for GetArtistInfoSuccessResponse {
        fn from(value: &GetArtistInfoSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetArtistInfoSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetArtistInfoSuccessResponseStatus {
        fn from(value: &GetArtistInfoSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetArtistInfoSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetArtistInfoSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetArtistInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetArtistInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetArtistInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested artist element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested artist element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetArtistSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetArtistResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetArtistResponse> for GetArtistResponse {
        fn from(value: &GetArtistResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetArtistResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetArtistResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetArtistSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetArtistResponseSubsonicResponse {
        GetArtistSuccessResponse(GetArtistSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetArtistResponseSubsonicResponse {
        fn from(value: &GetArtistResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetArtistSuccessResponse> for GetArtistResponseSubsonicResponse {
        fn from(value: GetArtistSuccessResponse) -> Self {
            Self::GetArtistSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetArtistResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetArtistSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "artist"
    ///      ],
    ///      "properties": {
    ///        "artist": {
    ///          "$ref": "#/components/schemas/ArtistWithAlbumsID3"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistSuccessResponse {
        pub artist: ArtistWithAlbumsId3,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetArtistSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetArtistSuccessResponse> for GetArtistSuccessResponse {
        fn from(value: &GetArtistSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetArtistSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetArtistSuccessResponseStatus {
        fn from(value: &GetArtistSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetArtistSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetArtistSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetArtistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetArtistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetArtistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested artists element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested artists element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetArtistsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetArtistsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetArtistsResponse> for GetArtistsResponse {
        fn from(value: &GetArtistsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetArtistsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetArtistsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetArtistsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetArtistsResponseSubsonicResponse {
        GetArtistsSuccessResponse(GetArtistsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetArtistsResponseSubsonicResponse {
        fn from(value: &GetArtistsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetArtistsSuccessResponse> for GetArtistsResponseSubsonicResponse {
        fn from(value: GetArtistsSuccessResponse) -> Self {
            Self::GetArtistsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetArtistsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetArtistsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "artists"
    ///      ],
    ///      "properties": {
    ///        "artists": {
    ///          "$ref": "#/components/schemas/ArtistsID3"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetArtistsSuccessResponse {
        pub artists: ArtistsId3,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetArtistsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetArtistsSuccessResponse> for GetArtistsSuccessResponse {
        fn from(value: &GetArtistsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetArtistsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetArtistsSuccessResponseStatus {
        fn from(value: &GetArtistsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetArtistsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetArtistsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetArtistsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetArtistsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetArtistsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested bookmarks element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested bookmarks element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetBookmarksSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetBookmarksResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetBookmarksResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetBookmarksResponse> for GetBookmarksResponse {
        fn from(value: &GetBookmarksResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetBookmarksResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetBookmarksResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetBookmarksSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetBookmarksResponseSubsonicResponse {
        GetBookmarksSuccessResponse(GetBookmarksSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetBookmarksResponseSubsonicResponse {
        fn from(value: &GetBookmarksResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetBookmarksSuccessResponse> for GetBookmarksResponseSubsonicResponse {
        fn from(value: GetBookmarksSuccessResponse) -> Self {
            Self::GetBookmarksSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetBookmarksResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetBookmarksSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "bookmarks"
    ///      ],
    ///      "properties": {
    ///        "bookmarks": {
    ///          "$ref": "#/components/schemas/Bookmarks"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetBookmarksSuccessResponse {
        pub bookmarks: Bookmarks,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetBookmarksSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetBookmarksSuccessResponse> for GetBookmarksSuccessResponse {
        fn from(value: &GetBookmarksSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetBookmarksSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetBookmarksSuccessResponseStatus {
        fn from(value: &GetBookmarksSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetBookmarksSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetBookmarksSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetBookmarksSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetBookmarksSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetBookmarksSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetCaptionsFormat`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "srt",
    ///    "vtt"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetCaptionsFormat {
        #[serde(rename = "srt")]
        Srt,
        #[serde(rename = "vtt")]
        Vtt,
    }
    impl ::std::convert::From<&Self> for GetCaptionsFormat {
        fn from(value: &GetCaptionsFormat) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetCaptionsFormat {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Srt => f.write_str("srt"),
                Self::Vtt => f.write_str("vtt"),
            }
        }
    }
    impl ::std::str::FromStr for GetCaptionsFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "srt" => Ok(Self::Srt),
                "vtt" => Ok(Self::Vtt),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetCaptionsFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetCaptionsFormat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetCaptionsFormat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested chatMessages element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested chatMessages element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetChatMessagesSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetChatMessagesResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetChatMessagesResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetChatMessagesResponse> for GetChatMessagesResponse {
        fn from(value: &GetChatMessagesResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetChatMessagesResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetChatMessagesResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetChatMessagesSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetChatMessagesResponseSubsonicResponse {
        GetChatMessagesSuccessResponse(GetChatMessagesSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetChatMessagesResponseSubsonicResponse {
        fn from(value: &GetChatMessagesResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetChatMessagesSuccessResponse>
        for GetChatMessagesResponseSubsonicResponse
    {
        fn from(value: GetChatMessagesSuccessResponse) -> Self {
            Self::GetChatMessagesSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetChatMessagesResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetChatMessagesSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "chatMessages"
    ///      ],
    ///      "properties": {
    ///        "chatMessages": {
    ///          "$ref": "#/components/schemas/ChatMessages"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetChatMessagesSuccessResponse {
        #[serde(rename = "chatMessages")]
        pub chat_messages: ChatMessages,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetChatMessagesSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetChatMessagesSuccessResponse> for GetChatMessagesSuccessResponse {
        fn from(value: &GetChatMessagesSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetChatMessagesSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetChatMessagesSuccessResponseStatus {
        fn from(value: &GetChatMessagesSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetChatMessagesSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetChatMessagesSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetChatMessagesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetChatMessagesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetChatMessagesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested genres element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested genres element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetGenresSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetGenresResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetGenresResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetGenresResponse> for GetGenresResponse {
        fn from(value: &GetGenresResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetGenresResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetGenresResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetGenresSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetGenresResponseSubsonicResponse {
        GetGenresSuccessResponse(GetGenresSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetGenresResponseSubsonicResponse {
        fn from(value: &GetGenresResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetGenresSuccessResponse> for GetGenresResponseSubsonicResponse {
        fn from(value: GetGenresSuccessResponse) -> Self {
            Self::GetGenresSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetGenresResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetGenresSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "genres"
    ///      ],
    ///      "properties": {
    ///        "genres": {
    ///          "$ref": "#/components/schemas/Genres"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetGenresSuccessResponse {
        pub genres: Genres,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetGenresSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetGenresSuccessResponse> for GetGenresSuccessResponse {
        fn from(value: &GetGenresSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetGenresSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetGenresSuccessResponseStatus {
        fn from(value: &GetGenresSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetGenresSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetGenresSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetGenresSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetGenresSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetGenresSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested indexes element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested indexes element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetIndexesSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetIndexesResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetIndexesResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetIndexesResponse> for GetIndexesResponse {
        fn from(value: &GetIndexesResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetIndexesResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetIndexesResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetIndexesSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetIndexesResponseSubsonicResponse {
        GetIndexesSuccessResponse(GetIndexesSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetIndexesResponseSubsonicResponse {
        fn from(value: &GetIndexesResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetIndexesSuccessResponse> for GetIndexesResponseSubsonicResponse {
        fn from(value: GetIndexesSuccessResponse) -> Self {
            Self::GetIndexesSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetIndexesResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetIndexesSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "indexes"
    ///      ],
    ///      "properties": {
    ///        "indexes": {
    ///          "$ref": "#/components/schemas/Indexes"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetIndexesSuccessResponse {
        pub indexes: Indexes,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetIndexesSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetIndexesSuccessResponse> for GetIndexesSuccessResponse {
        fn from(value: &GetIndexesSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetIndexesSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetIndexesSuccessResponseStatus {
        fn from(value: &GetIndexesSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetIndexesSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetIndexesSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetIndexesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetIndexesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetIndexesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested internetRadioStations element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested internetRadioStations element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetInternetRadioStationsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetInternetRadioStationsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response:
            ::std::option::Option<GetInternetRadioStationsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetInternetRadioStationsResponse> for GetInternetRadioStationsResponse {
        fn from(value: &GetInternetRadioStationsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetInternetRadioStationsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetInternetRadioStationsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetInternetRadioStationsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetInternetRadioStationsResponseSubsonicResponse {
        GetInternetRadioStationsSuccessResponse(GetInternetRadioStationsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetInternetRadioStationsResponseSubsonicResponse {
        fn from(value: &GetInternetRadioStationsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetInternetRadioStationsSuccessResponse>
        for GetInternetRadioStationsResponseSubsonicResponse
    {
        fn from(value: GetInternetRadioStationsSuccessResponse) -> Self {
            Self::GetInternetRadioStationsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse>
        for GetInternetRadioStationsResponseSubsonicResponse
    {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetInternetRadioStationsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "internetRadioStations"
    ///      ],
    ///      "properties": {
    ///        "internetRadioStations": {
    ///          "$ref": "#/components/schemas/InternetRadioStations"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetInternetRadioStationsSuccessResponse {
        #[serde(rename = "internetRadioStations")]
        pub internet_radio_stations: InternetRadioStations,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetInternetRadioStationsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetInternetRadioStationsSuccessResponse>
        for GetInternetRadioStationsSuccessResponse
    {
        fn from(value: &GetInternetRadioStationsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetInternetRadioStationsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetInternetRadioStationsSuccessResponseStatus {
        fn from(value: &GetInternetRadioStationsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetInternetRadioStationsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetInternetRadioStationsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetInternetRadioStationsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for GetInternetRadioStationsSuccessResponseStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetInternetRadioStationsSuccessResponseStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested license element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested license element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetLicenseSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLicenseResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetLicenseResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetLicenseResponse> for GetLicenseResponse {
        fn from(value: &GetLicenseResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetLicenseResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetLicenseResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetLicenseSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetLicenseResponseSubsonicResponse {
        GetLicenseSuccessResponse(GetLicenseSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetLicenseResponseSubsonicResponse {
        fn from(value: &GetLicenseResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetLicenseSuccessResponse> for GetLicenseResponseSubsonicResponse {
        fn from(value: GetLicenseSuccessResponse) -> Self {
            Self::GetLicenseSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetLicenseResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetLicenseSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "license"
    ///      ],
    ///      "properties": {
    ///        "license": {
    ///          "$ref": "#/components/schemas/License"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLicenseSuccessResponse {
        pub license: License,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetLicenseSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetLicenseSuccessResponse> for GetLicenseSuccessResponse {
        fn from(value: &GetLicenseSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetLicenseSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetLicenseSuccessResponseStatus {
        fn from(value: &GetLicenseSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetLicenseSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetLicenseSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetLicenseSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetLicenseSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetLicenseSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested lyricsList
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested lyricsList",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetLyricsBySongIdSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLyricsBySongIdResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetLyricsBySongIdResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetLyricsBySongIdResponse> for GetLyricsBySongIdResponse {
        fn from(value: &GetLyricsBySongIdResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetLyricsBySongIdResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetLyricsBySongIdResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetLyricsBySongIdSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetLyricsBySongIdResponseSubsonicResponse {
        GetLyricsBySongIdSuccessResponse(GetLyricsBySongIdSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetLyricsBySongIdResponseSubsonicResponse {
        fn from(value: &GetLyricsBySongIdResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetLyricsBySongIdSuccessResponse>
        for GetLyricsBySongIdResponseSubsonicResponse
    {
        fn from(value: GetLyricsBySongIdSuccessResponse) -> Self {
            Self::GetLyricsBySongIdSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetLyricsBySongIdResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetLyricsBySongIdSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "lyricsList"
    ///      ],
    ///      "properties": {
    ///        "lyricsList": {
    ///          "$ref": "#/components/schemas/LyricsList"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLyricsBySongIdSuccessResponse {
        #[serde(rename = "lyricsList")]
        pub lyrics_list: LyricsList,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetLyricsBySongIdSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetLyricsBySongIdSuccessResponse> for GetLyricsBySongIdSuccessResponse {
        fn from(value: &GetLyricsBySongIdSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetLyricsBySongIdSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetLyricsBySongIdSuccessResponseStatus {
        fn from(value: &GetLyricsBySongIdSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetLyricsBySongIdSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetLyricsBySongIdSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetLyricsBySongIdSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetLyricsBySongIdSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetLyricsBySongIdSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested lyrics element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested lyrics element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetLyricsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLyricsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetLyricsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetLyricsResponse> for GetLyricsResponse {
        fn from(value: &GetLyricsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetLyricsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetLyricsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetLyricsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetLyricsResponseSubsonicResponse {
        GetLyricsSuccessResponse(GetLyricsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetLyricsResponseSubsonicResponse {
        fn from(value: &GetLyricsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetLyricsSuccessResponse> for GetLyricsResponseSubsonicResponse {
        fn from(value: GetLyricsSuccessResponse) -> Self {
            Self::GetLyricsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetLyricsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetLyricsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "lyrics"
    ///      ],
    ///      "properties": {
    ///        "lyrics": {
    ///          "$ref": "#/components/schemas/Lyrics"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetLyricsSuccessResponse {
        pub lyrics: Lyrics,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetLyricsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetLyricsSuccessResponse> for GetLyricsSuccessResponse {
        fn from(value: &GetLyricsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetLyricsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetLyricsSuccessResponseStatus {
        fn from(value: &GetLyricsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetLyricsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetLyricsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetLyricsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetLyricsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetLyricsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested directory element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested directory element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetMusicDirectorySuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMusicDirectoryResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetMusicDirectoryResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetMusicDirectoryResponse> for GetMusicDirectoryResponse {
        fn from(value: &GetMusicDirectoryResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetMusicDirectoryResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetMusicDirectoryResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetMusicDirectorySuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetMusicDirectoryResponseSubsonicResponse {
        GetMusicDirectorySuccessResponse(GetMusicDirectorySuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetMusicDirectoryResponseSubsonicResponse {
        fn from(value: &GetMusicDirectoryResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetMusicDirectorySuccessResponse>
        for GetMusicDirectoryResponseSubsonicResponse
    {
        fn from(value: GetMusicDirectorySuccessResponse) -> Self {
            Self::GetMusicDirectorySuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetMusicDirectoryResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetMusicDirectorySuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "directory"
    ///      ],
    ///      "properties": {
    ///        "directory": {
    ///          "$ref": "#/components/schemas/Directory"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMusicDirectorySuccessResponse {
        pub directory: Directory,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetMusicDirectorySuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetMusicDirectorySuccessResponse> for GetMusicDirectorySuccessResponse {
        fn from(value: &GetMusicDirectorySuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetMusicDirectorySuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetMusicDirectorySuccessResponseStatus {
        fn from(value: &GetMusicDirectorySuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetMusicDirectorySuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetMusicDirectorySuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetMusicDirectorySuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetMusicDirectorySuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetMusicDirectorySuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested musicFolders element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested musicFolders element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetMusicFoldersSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMusicFoldersResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetMusicFoldersResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetMusicFoldersResponse> for GetMusicFoldersResponse {
        fn from(value: &GetMusicFoldersResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetMusicFoldersResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetMusicFoldersResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetMusicFoldersSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetMusicFoldersResponseSubsonicResponse {
        GetMusicFoldersSuccessResponse(GetMusicFoldersSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetMusicFoldersResponseSubsonicResponse {
        fn from(value: &GetMusicFoldersResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetMusicFoldersSuccessResponse>
        for GetMusicFoldersResponseSubsonicResponse
    {
        fn from(value: GetMusicFoldersSuccessResponse) -> Self {
            Self::GetMusicFoldersSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetMusicFoldersResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetMusicFoldersSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "musicFolders"
    ///      ],
    ///      "properties": {
    ///        "musicFolders": {
    ///          "$ref": "#/components/schemas/MusicFolders"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMusicFoldersSuccessResponse {
        #[serde(rename = "musicFolders")]
        pub music_folders: MusicFolders,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetMusicFoldersSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetMusicFoldersSuccessResponse> for GetMusicFoldersSuccessResponse {
        fn from(value: &GetMusicFoldersSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetMusicFoldersSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetMusicFoldersSuccessResponseStatus {
        fn from(value: &GetMusicFoldersSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetMusicFoldersSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetMusicFoldersSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetMusicFoldersSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetMusicFoldersSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetMusicFoldersSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `newestPodcasts` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `newestPodcasts` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetNewestPodcastsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNewestPodcastsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetNewestPodcastsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetNewestPodcastsResponse> for GetNewestPodcastsResponse {
        fn from(value: &GetNewestPodcastsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetNewestPodcastsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetNewestPodcastsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetNewestPodcastsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetNewestPodcastsResponseSubsonicResponse {
        GetNewestPodcastsSuccessResponse(GetNewestPodcastsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetNewestPodcastsResponseSubsonicResponse {
        fn from(value: &GetNewestPodcastsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetNewestPodcastsSuccessResponse>
        for GetNewestPodcastsResponseSubsonicResponse
    {
        fn from(value: GetNewestPodcastsSuccessResponse) -> Self {
            Self::GetNewestPodcastsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetNewestPodcastsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetNewestPodcastsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "newestPodcasts"
    ///      ],
    ///      "properties": {
    ///        "newestPodcasts": {
    ///          "$ref": "#/components/schemas/NewestPodcasts"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNewestPodcastsSuccessResponse {
        #[serde(rename = "newestPodcasts")]
        pub newest_podcasts: NewestPodcasts,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetNewestPodcastsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetNewestPodcastsSuccessResponse> for GetNewestPodcastsSuccessResponse {
        fn from(value: &GetNewestPodcastsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetNewestPodcastsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetNewestPodcastsSuccessResponseStatus {
        fn from(value: &GetNewestPodcastsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetNewestPodcastsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetNewestPodcastsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetNewestPodcastsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetNewestPodcastsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetNewestPodcastsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `nowPlaying` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `nowPlaying` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetNowPlayingSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNowPlayingResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetNowPlayingResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetNowPlayingResponse> for GetNowPlayingResponse {
        fn from(value: &GetNowPlayingResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetNowPlayingResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetNowPlayingResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetNowPlayingSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetNowPlayingResponseSubsonicResponse {
        GetNowPlayingSuccessResponse(GetNowPlayingSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetNowPlayingResponseSubsonicResponse {
        fn from(value: &GetNowPlayingResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetNowPlayingSuccessResponse> for GetNowPlayingResponseSubsonicResponse {
        fn from(value: GetNowPlayingSuccessResponse) -> Self {
            Self::GetNowPlayingSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetNowPlayingResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetNowPlayingSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "nowPlaying"
    ///      ],
    ///      "properties": {
    ///        "nowPlaying": {
    ///          "$ref": "#/components/schemas/NowPlaying"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetNowPlayingSuccessResponse {
        #[serde(rename = "nowPlaying")]
        pub now_playing: NowPlaying,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetNowPlayingSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetNowPlayingSuccessResponse> for GetNowPlayingSuccessResponse {
        fn from(value: &GetNowPlayingSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetNowPlayingSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetNowPlayingSuccessResponseStatus {
        fn from(value: &GetNowPlayingSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetNowPlayingSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetNowPlayingSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetNowPlayingSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetNowPlayingSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetNowPlayingSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `openSubsonicExtensions` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `openSubsonicExtensions` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetOpenSubsonicExtensionsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetOpenSubsonicExtensionsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response:
            ::std::option::Option<GetOpenSubsonicExtensionsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetOpenSubsonicExtensionsResponse>
        for GetOpenSubsonicExtensionsResponse
    {
        fn from(value: &GetOpenSubsonicExtensionsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetOpenSubsonicExtensionsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetOpenSubsonicExtensionsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetOpenSubsonicExtensionsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetOpenSubsonicExtensionsResponseSubsonicResponse {
        GetOpenSubsonicExtensionsSuccessResponse(GetOpenSubsonicExtensionsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetOpenSubsonicExtensionsResponseSubsonicResponse {
        fn from(value: &GetOpenSubsonicExtensionsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetOpenSubsonicExtensionsSuccessResponse>
        for GetOpenSubsonicExtensionsResponseSubsonicResponse
    {
        fn from(value: GetOpenSubsonicExtensionsSuccessResponse) -> Self {
            Self::GetOpenSubsonicExtensionsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse>
        for GetOpenSubsonicExtensionsResponseSubsonicResponse
    {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetOpenSubsonicExtensionsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "openSubsonicExtensions"
    ///      ],
    ///      "properties": {
    ///        "openSubsonicExtensions": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/OpenSubsonicExtension"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetOpenSubsonicExtensionsSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "openSubsonicExtensions")]
        pub open_subsonic_extensions: ::std::vec::Vec<OpenSubsonicExtension>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetOpenSubsonicExtensionsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetOpenSubsonicExtensionsSuccessResponse>
        for GetOpenSubsonicExtensionsSuccessResponse
    {
        fn from(value: &GetOpenSubsonicExtensionsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetOpenSubsonicExtensionsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetOpenSubsonicExtensionsSuccessResponseStatus {
        fn from(value: &GetOpenSubsonicExtensionsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetOpenSubsonicExtensionsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetOpenSubsonicExtensionsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetOpenSubsonicExtensionsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for GetOpenSubsonicExtensionsSuccessResponseStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for GetOpenSubsonicExtensionsSuccessResponseStatus
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `PlayQueueByIndex` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `PlayQueueByIndex` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetPlayQueueByIndexSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlayQueueByIndexResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetPlayQueueByIndexResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetPlayQueueByIndexResponse> for GetPlayQueueByIndexResponse {
        fn from(value: &GetPlayQueueByIndexResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetPlayQueueByIndexResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetPlayQueueByIndexResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetPlayQueueByIndexSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetPlayQueueByIndexResponseSubsonicResponse {
        GetPlayQueueByIndexSuccessResponse(GetPlayQueueByIndexSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetPlayQueueByIndexResponseSubsonicResponse {
        fn from(value: &GetPlayQueueByIndexResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetPlayQueueByIndexSuccessResponse>
        for GetPlayQueueByIndexResponseSubsonicResponse
    {
        fn from(value: GetPlayQueueByIndexSuccessResponse) -> Self {
            Self::GetPlayQueueByIndexSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetPlayQueueByIndexResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetPlayQueueByIndexSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "playQueueByIndex"
    ///      ],
    ///      "properties": {
    ///        "playQueueByIndex": {
    ///          "$ref": "#/components/schemas/PlayQueueByIndex"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlayQueueByIndexSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "playQueueByIndex")]
        pub play_queue_by_index: PlayQueueByIndex,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetPlayQueueByIndexSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetPlayQueueByIndexSuccessResponse>
        for GetPlayQueueByIndexSuccessResponse
    {
        fn from(value: &GetPlayQueueByIndexSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPlayQueueByIndexSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetPlayQueueByIndexSuccessResponseStatus {
        fn from(value: &GetPlayQueueByIndexSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetPlayQueueByIndexSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetPlayQueueByIndexSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPlayQueueByIndexSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetPlayQueueByIndexSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetPlayQueueByIndexSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `playQueue` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `playQueue` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetPlayQueueSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlayQueueResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetPlayQueueResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetPlayQueueResponse> for GetPlayQueueResponse {
        fn from(value: &GetPlayQueueResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetPlayQueueResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetPlayQueueResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetPlayQueueSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetPlayQueueResponseSubsonicResponse {
        GetPlayQueueSuccessResponse(GetPlayQueueSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetPlayQueueResponseSubsonicResponse {
        fn from(value: &GetPlayQueueResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetPlayQueueSuccessResponse> for GetPlayQueueResponseSubsonicResponse {
        fn from(value: GetPlayQueueSuccessResponse) -> Self {
            Self::GetPlayQueueSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetPlayQueueResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetPlayQueueSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "playQueue"
    ///      ],
    ///      "properties": {
    ///        "playQueue": {
    ///          "$ref": "#/components/schemas/PlayQueue"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlayQueueSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "playQueue")]
        pub play_queue: PlayQueue,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetPlayQueueSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetPlayQueueSuccessResponse> for GetPlayQueueSuccessResponse {
        fn from(value: &GetPlayQueueSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPlayQueueSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetPlayQueueSuccessResponseStatus {
        fn from(value: &GetPlayQueueSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetPlayQueueSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetPlayQueueSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPlayQueueSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetPlayQueueSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetPlayQueueSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested playlist element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested playlist element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetPlaylistSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlaylistResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetPlaylistResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetPlaylistResponse> for GetPlaylistResponse {
        fn from(value: &GetPlaylistResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetPlaylistResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetPlaylistResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetPlaylistSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetPlaylistResponseSubsonicResponse {
        GetPlaylistSuccessResponse(GetPlaylistSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetPlaylistResponseSubsonicResponse {
        fn from(value: &GetPlaylistResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetPlaylistSuccessResponse> for GetPlaylistResponseSubsonicResponse {
        fn from(value: GetPlaylistSuccessResponse) -> Self {
            Self::GetPlaylistSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetPlaylistResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetPlaylistSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "playlist"
    ///      ],
    ///      "properties": {
    ///        "playlist": {
    ///          "$ref": "#/components/schemas/PlaylistWithSongs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlaylistSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        pub playlist: PlaylistWithSongs,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetPlaylistSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetPlaylistSuccessResponse> for GetPlaylistSuccessResponse {
        fn from(value: &GetPlaylistSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPlaylistSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetPlaylistSuccessResponseStatus {
        fn from(value: &GetPlaylistSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetPlaylistSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetPlaylistSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPlaylistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetPlaylistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetPlaylistSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `playlists` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `playlists` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetPlaylistsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlaylistsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetPlaylistsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetPlaylistsResponse> for GetPlaylistsResponse {
        fn from(value: &GetPlaylistsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetPlaylistsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetPlaylistsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetPlaylistsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetPlaylistsResponseSubsonicResponse {
        GetPlaylistsSuccessResponse(GetPlaylistsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetPlaylistsResponseSubsonicResponse {
        fn from(value: &GetPlaylistsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetPlaylistsSuccessResponse> for GetPlaylistsResponseSubsonicResponse {
        fn from(value: GetPlaylistsSuccessResponse) -> Self {
            Self::GetPlaylistsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetPlaylistsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetPlaylistsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "playlists"
    ///      ],
    ///      "properties": {
    ///        "playlists": {
    ///          "$ref": "#/components/schemas/Playlists"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPlaylistsSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        pub playlists: Playlists,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetPlaylistsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetPlaylistsSuccessResponse> for GetPlaylistsSuccessResponse {
        fn from(value: &GetPlaylistsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPlaylistsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetPlaylistsSuccessResponseStatus {
        fn from(value: &GetPlaylistsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetPlaylistsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetPlaylistsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPlaylistsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetPlaylistsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetPlaylistsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `podcastEpisode` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `podcastEpisode` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetPodcastEpisodeSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPodcastEpisodeResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetPodcastEpisodeResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetPodcastEpisodeResponse> for GetPodcastEpisodeResponse {
        fn from(value: &GetPodcastEpisodeResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetPodcastEpisodeResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetPodcastEpisodeResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetPodcastEpisodeSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetPodcastEpisodeResponseSubsonicResponse {
        GetPodcastEpisodeSuccessResponse(GetPodcastEpisodeSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetPodcastEpisodeResponseSubsonicResponse {
        fn from(value: &GetPodcastEpisodeResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetPodcastEpisodeSuccessResponse>
        for GetPodcastEpisodeResponseSubsonicResponse
    {
        fn from(value: GetPodcastEpisodeSuccessResponse) -> Self {
            Self::GetPodcastEpisodeSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetPodcastEpisodeResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetPodcastEpisodeSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "podcastEpisode"
    ///      ],
    ///      "properties": {
    ///        "podcastEpisode": {
    ///          "$ref": "#/components/schemas/PodcastEpisode"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPodcastEpisodeSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "podcastEpisode")]
        pub podcast_episode: PodcastEpisode,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetPodcastEpisodeSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetPodcastEpisodeSuccessResponse> for GetPodcastEpisodeSuccessResponse {
        fn from(value: &GetPodcastEpisodeSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPodcastEpisodeSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetPodcastEpisodeSuccessResponseStatus {
        fn from(value: &GetPodcastEpisodeSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetPodcastEpisodeSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetPodcastEpisodeSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPodcastEpisodeSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetPodcastEpisodeSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetPodcastEpisodeSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `podcasts` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `podcasts` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetPodcastsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPodcastsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetPodcastsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetPodcastsResponse> for GetPodcastsResponse {
        fn from(value: &GetPodcastsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetPodcastsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetPodcastsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetPodcastsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetPodcastsResponseSubsonicResponse {
        GetPodcastsSuccessResponse(GetPodcastsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetPodcastsResponseSubsonicResponse {
        fn from(value: &GetPodcastsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetPodcastsSuccessResponse> for GetPodcastsResponseSubsonicResponse {
        fn from(value: GetPodcastsSuccessResponse) -> Self {
            Self::GetPodcastsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetPodcastsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetPodcastsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "podcasts"
    ///      ],
    ///      "properties": {
    ///        "podcasts": {
    ///          "$ref": "#/components/schemas/Podcasts"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetPodcastsSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        pub podcasts: Podcasts,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetPodcastsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetPodcastsSuccessResponse> for GetPodcastsSuccessResponse {
        fn from(value: &GetPodcastsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPodcastsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetPodcastsSuccessResponseStatus {
        fn from(value: &GetPodcastsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetPodcastsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetPodcastsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetPodcastsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetPodcastsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetPodcastsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `randomSongs` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `randomSongs` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetRandomSongsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetRandomSongsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetRandomSongsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetRandomSongsResponse> for GetRandomSongsResponse {
        fn from(value: &GetRandomSongsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetRandomSongsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetRandomSongsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetRandomSongsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetRandomSongsResponseSubsonicResponse {
        GetRandomSongsSuccessResponse(GetRandomSongsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetRandomSongsResponseSubsonicResponse {
        fn from(value: &GetRandomSongsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetRandomSongsSuccessResponse>
        for GetRandomSongsResponseSubsonicResponse
    {
        fn from(value: GetRandomSongsSuccessResponse) -> Self {
            Self::GetRandomSongsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetRandomSongsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetRandomSongsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "randomSongs"
    ///      ],
    ///      "properties": {
    ///        "randomSongs": {
    ///          "$ref": "#/components/schemas/Songs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetRandomSongsSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "randomSongs")]
        pub random_songs: Songs,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetRandomSongsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetRandomSongsSuccessResponse> for GetRandomSongsSuccessResponse {
        fn from(value: &GetRandomSongsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetRandomSongsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetRandomSongsSuccessResponseStatus {
        fn from(value: &GetRandomSongsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetRandomSongsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetRandomSongsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetRandomSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetRandomSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetRandomSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `scanStatus` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `scanStatus` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetScanStatusSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetScanStatusResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetScanStatusResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetScanStatusResponse> for GetScanStatusResponse {
        fn from(value: &GetScanStatusResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetScanStatusResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetScanStatusResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetScanStatusSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetScanStatusResponseSubsonicResponse {
        GetScanStatusSuccessResponse(GetScanStatusSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetScanStatusResponseSubsonicResponse {
        fn from(value: &GetScanStatusResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetScanStatusSuccessResponse> for GetScanStatusResponseSubsonicResponse {
        fn from(value: GetScanStatusSuccessResponse) -> Self {
            Self::GetScanStatusSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetScanStatusResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetScanStatusSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "scanStatus"
    ///      ],
    ///      "properties": {
    ///        "scanStatus": {
    ///          "$ref": "#/components/schemas/ScanStatus"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetScanStatusSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "scanStatus")]
        pub scan_status: ScanStatus,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetScanStatusSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetScanStatusSuccessResponse> for GetScanStatusSuccessResponse {
        fn from(value: &GetScanStatusSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetScanStatusSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetScanStatusSuccessResponseStatus {
        fn from(value: &GetScanStatusSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetScanStatusSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetScanStatusSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetScanStatusSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetScanStatusSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetScanStatusSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `shares` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `shares` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetSharesSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSharesResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetSharesResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetSharesResponse> for GetSharesResponse {
        fn from(value: &GetSharesResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetSharesResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetSharesResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetSharesSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetSharesResponseSubsonicResponse {
        GetSharesSuccessResponse(GetSharesSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetSharesResponseSubsonicResponse {
        fn from(value: &GetSharesResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetSharesSuccessResponse> for GetSharesResponseSubsonicResponse {
        fn from(value: GetSharesSuccessResponse) -> Self {
            Self::GetSharesSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetSharesResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetSharesSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "shares"
    ///      ],
    ///      "properties": {
    ///        "shares": {
    ///          "$ref": "#/components/schemas/Shares"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSharesSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        pub shares: Shares,
        ///The command result. `ok`
        pub status: GetSharesSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetSharesSuccessResponse> for GetSharesSuccessResponse {
        fn from(value: &GetSharesSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetSharesSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetSharesSuccessResponseStatus {
        fn from(value: &GetSharesSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetSharesSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetSharesSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetSharesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetSharesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetSharesSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `similarSongs2` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `similarSongs2` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetSimilarSongs2SuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSimilarSongs2Response {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetSimilarSongs2ResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetSimilarSongs2Response> for GetSimilarSongs2Response {
        fn from(value: &GetSimilarSongs2Response) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetSimilarSongs2Response {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetSimilarSongs2ResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetSimilarSongs2SuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetSimilarSongs2ResponseSubsonicResponse {
        GetSimilarSongs2SuccessResponse(GetSimilarSongs2SuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetSimilarSongs2ResponseSubsonicResponse {
        fn from(value: &GetSimilarSongs2ResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetSimilarSongs2SuccessResponse>
        for GetSimilarSongs2ResponseSubsonicResponse
    {
        fn from(value: GetSimilarSongs2SuccessResponse) -> Self {
            Self::GetSimilarSongs2SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetSimilarSongs2ResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetSimilarSongs2SuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "similarSongs2"
    ///      ],
    ///      "properties": {
    ///        "similarSongs2": {
    ///          "$ref": "#/components/schemas/SimilarSongs2"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSimilarSongs2SuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        #[serde(rename = "similarSongs2")]
        pub similar_songs2: SimilarSongs2,
        ///The command result. `ok`
        pub status: GetSimilarSongs2SuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetSimilarSongs2SuccessResponse> for GetSimilarSongs2SuccessResponse {
        fn from(value: &GetSimilarSongs2SuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetSimilarSongs2SuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetSimilarSongs2SuccessResponseStatus {
        fn from(value: &GetSimilarSongs2SuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetSimilarSongs2SuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetSimilarSongs2SuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetSimilarSongs2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetSimilarSongs2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetSimilarSongs2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `similarSongs` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `similarSongs` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetSimilarSongsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSimilarSongsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetSimilarSongsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetSimilarSongsResponse> for GetSimilarSongsResponse {
        fn from(value: &GetSimilarSongsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetSimilarSongsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetSimilarSongsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetSimilarSongsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetSimilarSongsResponseSubsonicResponse {
        GetSimilarSongsSuccessResponse(GetSimilarSongsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetSimilarSongsResponseSubsonicResponse {
        fn from(value: &GetSimilarSongsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetSimilarSongsSuccessResponse>
        for GetSimilarSongsResponseSubsonicResponse
    {
        fn from(value: GetSimilarSongsSuccessResponse) -> Self {
            Self::GetSimilarSongsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetSimilarSongsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetSimilarSongsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "similarSongs"
    ///      ],
    ///      "properties": {
    ///        "similarSongs": {
    ///          "$ref": "#/components/schemas/SimilarSongs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSimilarSongsSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        #[serde(rename = "similarSongs")]
        pub similar_songs: SimilarSongs,
        ///The command result. `ok`
        pub status: GetSimilarSongsSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetSimilarSongsSuccessResponse> for GetSimilarSongsSuccessResponse {
        fn from(value: &GetSimilarSongsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetSimilarSongsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetSimilarSongsSuccessResponseStatus {
        fn from(value: &GetSimilarSongsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetSimilarSongsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetSimilarSongsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetSimilarSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetSimilarSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetSimilarSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `song` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `song` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetSongSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSongResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetSongResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetSongResponse> for GetSongResponse {
        fn from(value: &GetSongResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetSongResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetSongResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetSongSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetSongResponseSubsonicResponse {
        GetSongSuccessResponse(GetSongSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetSongResponseSubsonicResponse {
        fn from(value: &GetSongResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetSongSuccessResponse> for GetSongResponseSubsonicResponse {
        fn from(value: GetSongSuccessResponse) -> Self {
            Self::GetSongSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetSongResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetSongSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "song"
    ///      ],
    ///      "properties": {
    ///        "song": {
    ///          "$ref": "#/components/schemas/Child"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSongSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        pub song: Child,
        ///The command result. `ok`
        pub status: GetSongSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetSongSuccessResponse> for GetSongSuccessResponse {
        fn from(value: &GetSongSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetSongSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetSongSuccessResponseStatus {
        fn from(value: &GetSongSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetSongSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetSongSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetSongSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetSongSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetSongSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `songsByGenre` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `songsByGenre` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetSongsByGenreSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSongsByGenreResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetSongsByGenreResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetSongsByGenreResponse> for GetSongsByGenreResponse {
        fn from(value: &GetSongsByGenreResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetSongsByGenreResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetSongsByGenreResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetSongsByGenreSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetSongsByGenreResponseSubsonicResponse {
        GetSongsByGenreSuccessResponse(GetSongsByGenreSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetSongsByGenreResponseSubsonicResponse {
        fn from(value: &GetSongsByGenreResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetSongsByGenreSuccessResponse>
        for GetSongsByGenreResponseSubsonicResponse
    {
        fn from(value: GetSongsByGenreSuccessResponse) -> Self {
            Self::GetSongsByGenreSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetSongsByGenreResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetSongsByGenreSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "songsByGenre"
    ///      ],
    ///      "properties": {
    ///        "songsByGenre": {
    ///          "$ref": "#/components/schemas/Songs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSongsByGenreSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        #[serde(rename = "songsByGenre")]
        pub songs_by_genre: Songs,
        ///The command result. `ok`
        pub status: GetSongsByGenreSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetSongsByGenreSuccessResponse> for GetSongsByGenreSuccessResponse {
        fn from(value: &GetSongsByGenreSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetSongsByGenreSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetSongsByGenreSuccessResponseStatus {
        fn from(value: &GetSongsByGenreSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetSongsByGenreSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetSongsByGenreSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetSongsByGenreSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetSongsByGenreSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetSongsByGenreSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `starred2` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `starred2` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetStarred2SuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetStarred2Response {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetStarred2ResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetStarred2Response> for GetStarred2Response {
        fn from(value: &GetStarred2Response) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetStarred2Response {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetStarred2ResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetStarred2SuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetStarred2ResponseSubsonicResponse {
        GetStarred2SuccessResponse(GetStarred2SuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetStarred2ResponseSubsonicResponse {
        fn from(value: &GetStarred2ResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetStarred2SuccessResponse> for GetStarred2ResponseSubsonicResponse {
        fn from(value: GetStarred2SuccessResponse) -> Self {
            Self::GetStarred2SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetStarred2ResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetStarred2SuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "starred2"
    ///      ],
    ///      "properties": {
    ///        "starred2": {
    ///          "$ref": "#/components/schemas/Starred2"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetStarred2SuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        pub starred2: Starred2,
        ///The command result. `ok`
        pub status: GetStarred2SuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetStarred2SuccessResponse> for GetStarred2SuccessResponse {
        fn from(value: &GetStarred2SuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetStarred2SuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetStarred2SuccessResponseStatus {
        fn from(value: &GetStarred2SuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetStarred2SuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetStarred2SuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetStarred2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetStarred2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetStarred2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `starred` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `starred` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetStarredSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetStarredResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetStarredResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetStarredResponse> for GetStarredResponse {
        fn from(value: &GetStarredResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetStarredResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetStarredResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetStarredSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetStarredResponseSubsonicResponse {
        GetStarredSuccessResponse(GetStarredSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetStarredResponseSubsonicResponse {
        fn from(value: &GetStarredResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetStarredSuccessResponse> for GetStarredResponseSubsonicResponse {
        fn from(value: GetStarredSuccessResponse) -> Self {
            Self::GetStarredSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetStarredResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetStarredSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "starred"
    ///      ],
    ///      "properties": {
    ///        "starred": {
    ///          "$ref": "#/components/schemas/Starred"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetStarredSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        pub starred: Starred,
        ///The command result. `ok`
        pub status: GetStarredSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetStarredSuccessResponse> for GetStarredSuccessResponse {
        fn from(value: &GetStarredSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetStarredSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetStarredSuccessResponseStatus {
        fn from(value: &GetStarredSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetStarredSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetStarredSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetStarredSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetStarredSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetStarredSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested tokenInfo on success, or error 44 on invalid token.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested tokenInfo on success, or error 44 on invalid token.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetTokenInfoSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTokenInfoResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetTokenInfoResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetTokenInfoResponse> for GetTokenInfoResponse {
        fn from(value: &GetTokenInfoResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetTokenInfoResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetTokenInfoResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetTokenInfoSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetTokenInfoResponseSubsonicResponse {
        GetTokenInfoSuccessResponse(GetTokenInfoSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetTokenInfoResponseSubsonicResponse {
        fn from(value: &GetTokenInfoResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetTokenInfoSuccessResponse> for GetTokenInfoResponseSubsonicResponse {
        fn from(value: GetTokenInfoSuccessResponse) -> Self {
            Self::GetTokenInfoSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetTokenInfoResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetTokenInfoSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "tokenInfo"
    ///      ],
    ///      "properties": {
    ///        "tokenInfo": {
    ///          "$ref": "#/components/schemas/TokenInfo"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTokenInfoSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetTokenInfoSuccessResponseStatus,
        #[serde(rename = "tokenInfo")]
        pub token_info: TokenInfo,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetTokenInfoSuccessResponse> for GetTokenInfoSuccessResponse {
        fn from(value: &GetTokenInfoSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetTokenInfoSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetTokenInfoSuccessResponseStatus {
        fn from(value: &GetTokenInfoSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetTokenInfoSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetTokenInfoSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTokenInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetTokenInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetTokenInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `topSongs` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `topSongs` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetTopSongsSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTopSongsResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetTopSongsResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetTopSongsResponse> for GetTopSongsResponse {
        fn from(value: &GetTopSongsResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetTopSongsResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetTopSongsResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetTopSongsSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetTopSongsResponseSubsonicResponse {
        GetTopSongsSuccessResponse(GetTopSongsSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetTopSongsResponseSubsonicResponse {
        fn from(value: &GetTopSongsResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetTopSongsSuccessResponse> for GetTopSongsResponseSubsonicResponse {
        fn from(value: GetTopSongsSuccessResponse) -> Self {
            Self::GetTopSongsSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetTopSongsResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetTopSongsSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "topSongs"
    ///      ],
    ///      "properties": {
    ///        "topSongs": {
    ///          "$ref": "#/components/schemas/TopSongs"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTopSongsSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetTopSongsSuccessResponseStatus,
        #[serde(rename = "topSongs")]
        pub top_songs: TopSongs,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetTopSongsSuccessResponse> for GetTopSongsSuccessResponse {
        fn from(value: &GetTopSongsSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetTopSongsSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetTopSongsSuccessResponseStatus {
        fn from(value: &GetTopSongsSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetTopSongsSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetTopSongsSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTopSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetTopSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetTopSongsSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `user` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `user` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetUserSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetUserResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetUserResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetUserResponse> for GetUserResponse {
        fn from(value: &GetUserResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetUserResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetUserResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetUserSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetUserResponseSubsonicResponse {
        GetUserSuccessResponse(GetUserSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetUserResponseSubsonicResponse {
        fn from(value: &GetUserResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetUserSuccessResponse> for GetUserResponseSubsonicResponse {
        fn from(value: GetUserSuccessResponse) -> Self {
            Self::GetUserSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetUserResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetUserSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "user"
    ///      ],
    ///      "properties": {
    ///        "user": {
    ///          "$ref": "#/components/schemas/User"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetUserSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetUserSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        pub user: User,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetUserSuccessResponse> for GetUserSuccessResponse {
        fn from(value: &GetUserSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetUserSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetUserSuccessResponseStatus {
        fn from(value: &GetUserSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetUserSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetUserSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetUserSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetUserSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetUserSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `user` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `user` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetUsersSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetUsersResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetUsersResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetUsersResponse> for GetUsersResponse {
        fn from(value: &GetUsersResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetUsersResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetUsersResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetUsersSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetUsersResponseSubsonicResponse {
        GetUsersSuccessResponse(GetUsersSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetUsersResponseSubsonicResponse {
        fn from(value: &GetUsersResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetUsersSuccessResponse> for GetUsersResponseSubsonicResponse {
        fn from(value: GetUsersSuccessResponse) -> Self {
            Self::GetUsersSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetUsersResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetUsersSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "users"
    ///      ],
    ///      "properties": {
    ///        "users": {
    ///          "$ref": "#/components/schemas/Users"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetUsersSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetUsersSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        pub users: Users,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&GetUsersSuccessResponse> for GetUsersSuccessResponse {
        fn from(value: &GetUsersSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetUsersSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetUsersSuccessResponseStatus {
        fn from(value: &GetUsersSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetUsersSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetUsersSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetUsersSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetUsersSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetUsersSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `videoInfo` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `videoInfo` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetVideoInfoSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetVideoInfoResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetVideoInfoResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetVideoInfoResponse> for GetVideoInfoResponse {
        fn from(value: &GetVideoInfoResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetVideoInfoResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetVideoInfoResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetVideoInfoSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetVideoInfoResponseSubsonicResponse {
        GetVideoInfoSuccessResponse(GetVideoInfoSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetVideoInfoResponseSubsonicResponse {
        fn from(value: &GetVideoInfoResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetVideoInfoSuccessResponse> for GetVideoInfoResponseSubsonicResponse {
        fn from(value: GetVideoInfoSuccessResponse) -> Self {
            Self::GetVideoInfoSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetVideoInfoResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetVideoInfoSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "videoInfo"
    ///      ],
    ///      "properties": {
    ///        "videoInfo": {
    ///          "$ref": "#/components/schemas/VideoInfo"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetVideoInfoSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetVideoInfoSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
        #[serde(rename = "videoInfo")]
        pub video_info: VideoInfo,
    }
    impl ::std::convert::From<&GetVideoInfoSuccessResponse> for GetVideoInfoSuccessResponse {
        fn from(value: &GetVideoInfoSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetVideoInfoSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetVideoInfoSuccessResponseStatus {
        fn from(value: &GetVideoInfoSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetVideoInfoSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetVideoInfoSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetVideoInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetVideoInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetVideoInfoSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `videos` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `videos` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GetVideosSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetVideosResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<GetVideosResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&GetVideosResponse> for GetVideosResponse {
        fn from(value: &GetVideosResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for GetVideosResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`GetVideosResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/GetVideosSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetVideosResponseSubsonicResponse {
        GetVideosSuccessResponse(GetVideosSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for GetVideosResponseSubsonicResponse {
        fn from(value: &GetVideosResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<GetVideosSuccessResponse> for GetVideosResponseSubsonicResponse {
        fn from(value: GetVideosSuccessResponse) -> Self {
            Self::GetVideosSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for GetVideosResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`GetVideosSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "videos"
    ///      ],
    ///      "properties": {
    ///        "videos": {
    ///          "$ref": "#/components/schemas/Videos"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetVideosSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: GetVideosSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
        pub videos: Videos,
    }
    impl ::std::convert::From<&GetVideosSuccessResponse> for GetVideosSuccessResponse {
        fn from(value: &GetVideosSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetVideosSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for GetVideosSuccessResponseStatus {
        fn from(value: &GetVideosSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetVideosSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for GetVideosSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetVideosSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetVideosSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetVideosSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///An indexed artist list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An indexed artist list.",
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "artist": {
    ///      "description": "Artist list",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Artist"
    ///      }
    ///    },
    ///    "name": {
    ///      "description": "Index name",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Index {
        ///Artist list
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artist: ::std::vec::Vec<Artist>,
        ///Index name
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&Index> for Index {
        fn from(value: &Index) -> Self {
            value.clone()
        }
    }
    ///Artist list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Artist list.",
    ///  "type": "object",
    ///  "required": [
    ///    "ignoredArticles",
    ///    "lastModified"
    ///  ],
    ///  "properties": {
    ///    "child": {
    ///      "description": "Array of children",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "ignoredArticles": {
    ///      "description": "The ignored articles",
    ///      "type": "string"
    ///    },
    ///    "index": {
    ///      "description": "Indexed artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Index"
    ///      }
    ///    },
    ///    "lastModified": {
    ///      "description": "Last time the index was modified in milliseconds after January 1, 1970 UTC",
    ///      "type": "integer"
    ///    },
    ///    "shortcut": {
    ///      "description": "Shortcut",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Artist"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Indexes {
        ///Array of children
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub child: ::std::vec::Vec<Child>,
        ///The ignored articles
        #[serde(rename = "ignoredArticles")]
        pub ignored_articles: ::std::string::String,
        ///Indexed artists
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub index: ::std::vec::Vec<Index>,
        ///Last time the index was modified in milliseconds after January 1, 1970 UTC
        #[serde(rename = "lastModified")]
        pub last_modified: i64,
        ///Shortcut
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub shortcut: ::std::vec::Vec<Artist>,
    }
    impl ::std::convert::From<&Indexes> for Indexes {
        fn from(value: &Indexes) -> Self {
            value.clone()
        }
    }
    ///An internetRadioStation.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An internetRadioStation.",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "streamUrl"
    ///  ],
    ///  "properties": {
    ///    "homePageUrl": {
    ///      "description": "Genre name",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "The Id",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name",
    ///      "type": "string"
    ///    },
    ///    "streamUrl": {
    ///      "description": "The streamUrl",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InternetRadioStation {
        ///Genre name
        #[serde(
            rename = "homePageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub home_page_url: ::std::option::Option<::std::string::String>,
        ///The Id
        pub id: ::std::string::String,
        ///The name
        pub name: ::std::string::String,
        ///The streamUrl
        #[serde(rename = "streamUrl")]
        pub stream_url: ::std::string::String,
    }
    impl ::std::convert::From<&InternetRadioStation> for InternetRadioStation {
        fn from(value: &InternetRadioStation) -> Self {
            value.clone()
        }
    }
    ///internetRadioStations.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "internetRadioStations.",
    ///  "type": "object",
    ///  "properties": {
    ///    "internetRadioStation": {
    ///      "description": "A list of internetRadioStation",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InternetRadioStation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InternetRadioStations {
        ///A list of internetRadioStation
        #[serde(
            rename = "internetRadioStation",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub internet_radio_station: ::std::vec::Vec<InternetRadioStation>,
    }
    impl ::std::convert::From<&InternetRadioStations> for InternetRadioStations {
        fn from(value: &InternetRadioStations) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for InternetRadioStations {
        fn default() -> Self {
            Self {
                internet_radio_station: Default::default(),
            }
        }
    }
    ///A date for a media item that may be just a year, or year-month, or full date.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A date for a media item that may be just a year, or year-month, or full date.",
    ///  "type": "object",
    ///  "properties": {
    ///    "day": {
    ///      "description": "The day (1-31)",
    ///      "type": "integer",
    ///      "maximum": 31.0,
    ///      "minimum": 1.0
    ///    },
    ///    "month": {
    ///      "description": "The month (1-12)",
    ///      "type": "integer",
    ///      "maximum": 12.0,
    ///      "minimum": 1.0
    ///    },
    ///    "year": {
    ///      "description": "The year",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ItemDate {
        ///The day (1-31)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub day: ::std::option::Option<::std::num::NonZeroU64>,
        ///The month (1-12)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub month: ::std::option::Option<::std::num::NonZeroU64>,
        ///The year
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&ItemDate> for ItemDate {
        fn from(value: &ItemDate) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ItemDate {
        fn default() -> Self {
            Self {
                day: Default::default(),
                month: Default::default(),
                year: Default::default(),
            }
        }
    }
    ///A genre returned in list of genres for an item.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A genre returned in list of genres for an item.",
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "Genre name",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ItemGenre {
        ///Genre name
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&ItemGenre> for ItemGenre {
        fn from(value: &ItemGenre) -> Self {
            value.clone()
        }
    }
    ///JukeBox action.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "JukeBox action.",
    ///  "type": "string",
    ///  "enum": [
    ///    "get",
    ///    "status",
    ///    "set",
    ///    "start",
    ///    "stop",
    ///    "skip",
    ///    "add",
    ///    "clear",
    ///    "remove",
    ///    "shuffle",
    ///    "setGain"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum JukeboxAction {
        #[serde(rename = "get")]
        Get,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "set")]
        Set,
        #[serde(rename = "start")]
        Start,
        #[serde(rename = "stop")]
        Stop,
        #[serde(rename = "skip")]
        Skip,
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "clear")]
        Clear,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "shuffle")]
        Shuffle,
        #[serde(rename = "setGain")]
        SetGain,
    }
    impl ::std::convert::From<&Self> for JukeboxAction {
        fn from(value: &JukeboxAction) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for JukeboxAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Get => f.write_str("get"),
                Self::Status => f.write_str("status"),
                Self::Set => f.write_str("set"),
                Self::Start => f.write_str("start"),
                Self::Stop => f.write_str("stop"),
                Self::Skip => f.write_str("skip"),
                Self::Add => f.write_str("add"),
                Self::Clear => f.write_str("clear"),
                Self::Remove => f.write_str("remove"),
                Self::Shuffle => f.write_str("shuffle"),
                Self::SetGain => f.write_str("setGain"),
            }
        }
    }
    impl ::std::str::FromStr for JukeboxAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "get" => Ok(Self::Get),
                "status" => Ok(Self::Status),
                "set" => Ok(Self::Set),
                "start" => Ok(Self::Start),
                "stop" => Ok(Self::Stop),
                "skip" => Ok(Self::Skip),
                "add" => Ok(Self::Add),
                "clear" => Ok(Self::Clear),
                "remove" => Ok(Self::Remove),
                "shuffle" => Ok(Self::Shuffle),
                "setGain" => Ok(Self::SetGain),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for JukeboxAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for JukeboxAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for JukeboxAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A subsonic-response element with a nested :

    - jukeboxStatus for all actions but get
    - jukeboxPlaylist for get action*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested :\n\n- jukeboxStatus for all actions but get\n- jukeboxPlaylist for get action",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/JukeboxControlSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct JukeboxControlResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<JukeboxControlResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&JukeboxControlResponse> for JukeboxControlResponse {
        fn from(value: &JukeboxControlResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for JukeboxControlResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`JukeboxControlResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/JukeboxControlSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum JukeboxControlResponseSubsonicResponse {
        JukeboxControlSuccessResponse(JukeboxControlSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for JukeboxControlResponseSubsonicResponse {
        fn from(value: &JukeboxControlResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<JukeboxControlSuccessResponse>
        for JukeboxControlResponseSubsonicResponse
    {
        fn from(value: JukeboxControlSuccessResponse) -> Self {
            Self::JukeboxControlSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for JukeboxControlResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`JukeboxControlSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "jukeboxPlaylist": {
    ///          "$ref": "#/components/schemas/JukeboxPlaylist"
    ///        },
    ///        "jukeboxStatus": {
    ///          "$ref": "#/components/schemas/JukeboxStatus"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct JukeboxControlSuccessResponse {
        #[serde(
            rename = "jukeboxPlaylist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub jukebox_playlist: ::std::option::Option<JukeboxPlaylist>,
        #[serde(
            rename = "jukeboxStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub jukebox_status: ::std::option::Option<JukeboxStatus>,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: JukeboxControlSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&JukeboxControlSuccessResponse> for JukeboxControlSuccessResponse {
        fn from(value: &JukeboxControlSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum JukeboxControlSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for JukeboxControlSuccessResponseStatus {
        fn from(value: &JukeboxControlSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for JukeboxControlSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for JukeboxControlSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for JukeboxControlSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for JukeboxControlSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for JukeboxControlSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`JukeboxPlaylist`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/JukeboxStatus"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "entry": {
    ///          "description": "The songs currently enqueued in the jukebox",
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/Child"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct JukeboxPlaylist {
        ///The current index of the song being played
        #[serde(rename = "currentIndex")]
        pub current_index: i64,
        ///The songs currently enqueued in the jukebox
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub entry: ::std::vec::Vec<Child>,
        ///Whether the queue is currently playing
        pub playing: bool,
        ///The current position of the track in seconds
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<i64>,
        ///Volume, in a range of [0.0, 1.0]
        pub volume: i64,
    }
    impl ::std::convert::From<&JukeboxPlaylist> for JukeboxPlaylist {
        fn from(value: &JukeboxPlaylist) -> Self {
            value.clone()
        }
    }
    ///jukeboxStatus.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "jukeboxStatus.",
    ///  "type": "object",
    ///  "required": [
    ///    "currentIndex",
    ///    "playing",
    ///    "volume"
    ///  ],
    ///  "properties": {
    ///    "currentIndex": {
    ///      "description": "The current index of the song being played",
    ///      "type": "integer"
    ///    },
    ///    "playing": {
    ///      "description": "Whether the queue is currently playing",
    ///      "type": "boolean"
    ///    },
    ///    "position": {
    ///      "description": "The current position of the track in seconds",
    ///      "type": "integer"
    ///    },
    ///    "volume": {
    ///      "description": "Volume, in a range of [0.0, 1.0]",
    ///      "type": "integer",
    ///      "format": "float",
    ///      "maximum": 1.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct JukeboxStatus {
        ///The current index of the song being played
        #[serde(rename = "currentIndex")]
        pub current_index: i64,
        ///Whether the queue is currently playing
        pub playing: bool,
        ///The current position of the track in seconds
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<i64>,
        ///Volume, in a range of [0.0, 1.0]
        pub volume: i64,
    }
    impl ::std::convert::From<&JukeboxStatus> for JukeboxStatus {
        fn from(value: &JukeboxStatus) -> Self {
            value.clone()
        }
    }
    ///getLicense result.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "getLicense result.",
    ///  "type": "object",
    ///  "required": [
    ///    "valid"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "description": "User email",
    ///      "type": "string"
    ///    },
    ///    "licenseExpires": {
    ///      "description": "End of license date. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "trialExpires": {
    ///      "description": "End of trial date. [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "valid": {
    ///      "description": "The status of the license",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct License {
        ///User email
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        ///End of license date. [ISO 8601]
        #[serde(
            rename = "licenseExpires",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub license_expires: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///End of trial date. [ISO 8601]
        #[serde(
            rename = "trialExpires",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trial_expires: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The status of the license
        pub valid: bool,
    }
    impl ::std::convert::From<&License> for License {
        fn from(value: &License) -> Self {
            value.clone()
        }
    }
    ///One line of a song lyric
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "One line of a song lyric",
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "start": {
    ///      "description": "The start time of the lyrics, relative to the start time of the track, in milliseconds. If this is not part of synced lyrics, start __must__ be omitted",
    ///      "type": "number"
    ///    },
    ///    "value": {
    ///      "description": "The actual text of this line",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Line {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start: ::std::option::Option<f64>,
        ///The actual text of this line
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&Line> for Line {
        fn from(value: &Line) -> Self {
            value.clone()
        }
    }
    ///Lyrics.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Lyrics.",
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "artist": {
    ///      "description": "The artist name",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "The song title",
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "description": "The lyrics",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Lyrics {
        ///The artist name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The song title
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        ///The lyrics
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&Lyrics> for Lyrics {
        fn from(value: &Lyrics) -> Self {
            value.clone()
        }
    }
    ///List of structured lyrics
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "List of structured lyrics",
    ///  "type": "object",
    ///  "properties": {
    ///    "structuredLyrics": {
    ///      "description": "Structured lyrics. There can be multiple lyrics of the same type with the same language",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/StructuredLyrics"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LyricsList {
        ///Structured lyrics. There can be multiple lyrics of the same type with the same language
        #[serde(
            rename = "structuredLyrics",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub structured_lyrics: ::std::vec::Vec<StructuredLyrics>,
    }
    impl ::std::convert::From<&LyricsList> for LyricsList {
        fn from(value: &LyricsList) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for LyricsList {
        fn default() -> Self {
            Self {
                structured_lyrics: Default::default(),
            }
        }
    }
    ///Note: If you support `musicBrainzId` you must support this field to ensure clients knows what the ID refers to.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Note: If you support `musicBrainzId` you must support this field to ensure clients knows what the ID refers to.",
    ///  "type": "string",
    ///  "enum": [
    ///    "song",
    ///    "album",
    ///    "artist"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MediaType {
        #[serde(rename = "song")]
        Song,
        #[serde(rename = "album")]
        Album,
        #[serde(rename = "artist")]
        Artist,
    }
    impl ::std::convert::From<&Self> for MediaType {
        fn from(value: &MediaType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for MediaType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Song => f.write_str("song"),
                Self::Album => f.write_str("album"),
                Self::Artist => f.write_str("artist"),
            }
        }
    }
    impl ::std::str::FromStr for MediaType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "song" => Ok(Self::Song),
                "album" => Ok(Self::Album),
                "artist" => Ok(Self::Artist),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MediaType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MediaType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MediaType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///MusicFolder.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "MusicFolder.",
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The id",
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "description": "The folder name",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MusicFolder {
        ///The id
        pub id: i64,
        ///The folder name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&MusicFolder> for MusicFolder {
        fn from(value: &MusicFolder) -> Self {
            value.clone()
        }
    }
    ///MusicFolders.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "MusicFolders.",
    ///  "type": "object",
    ///  "properties": {
    ///    "musicFolder": {
    ///      "description": "The folders",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MusicFolder"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MusicFolders {
        ///The folders
        #[serde(
            rename = "musicFolder",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub music_folder: ::std::vec::Vec<MusicFolder>,
    }
    impl ::std::convert::From<&MusicFolders> for MusicFolders {
        fn from(value: &MusicFolders) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for MusicFolders {
        fn default() -> Self {
            Self {
                music_folder: Default::default(),
            }
        }
    }
    ///NewestPodcasts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NewestPodcasts.",
    ///  "type": "object",
    ///  "properties": {
    ///    "episode": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PodcastEpisode"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NewestPodcasts {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub episode: ::std::vec::Vec<PodcastEpisode>,
    }
    impl ::std::convert::From<&NewestPodcasts> for NewestPodcasts {
        fn from(value: &NewestPodcasts) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for NewestPodcasts {
        fn default() -> Self {
            Self {
                episode: Default::default(),
            }
        }
    }
    ///nowPlaying.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "nowPlaying.",
    ///  "type": "object",
    ///  "required": [
    ///    "entry"
    ///  ],
    ///  "properties": {
    ///    "entry": {
    ///      "description": "The now playing entries",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NowPlayingEntry"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NowPlaying {
        ///The now playing entries
        pub entry: ::std::vec::Vec<NowPlayingEntry>,
    }
    impl ::std::convert::From<&NowPlaying> for NowPlaying {
        fn from(value: &NowPlaying) -> Self {
            value.clone()
        }
    }
    ///`NowPlayingEntry`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Child"
    ///    },
    ///    {
    ///      "description": "NowPlayingEntry.",
    ///      "type": "object",
    ///      "required": [
    ///        "minutesAgo",
    ///        "playerId",
    ///        "username"
    ///      ],
    ///      "properties": {
    ///        "minutesAgo": {
    ///          "description": "Last update",
    ///          "type": "integer"
    ///        },
    ///        "playerId": {
    ///          "description": "Player Id",
    ///          "type": "integer"
    ///        },
    ///        "playerName": {
    ///          "description": "Player name",
    ///          "type": "string"
    ///        },
    ///        "username": {
    ///          "description": "The username",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NowPlayingEntry {
        ///The album name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub album: ::std::option::Option<::std::string::String>,
        ///The list of all album artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)
        #[serde(
            rename = "albumArtists",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub album_artists: ::std::vec::Vec<ArtistId3>,
        ///The corresponding album id
        #[serde(
            rename = "albumId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_id: ::std::option::Option<::std::string::String>,
        ///The artist name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The corresponding artist id
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_id: ::std::option::Option<::std::string::String>,
        ///The list of all song artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artists: ::std::vec::Vec<ArtistId3>,
        #[serde(
            rename = "averageRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_rating: ::std::option::Option<f64>,
        ///The bit depth of the media.
        #[serde(
            rename = "bitDepth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_depth: ::std::option::Option<i64>,
        ///The bitrate of the media.
        #[serde(
            rename = "bitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_rate: ::std::option::Option<i64>,
        ///The bookmark position in seconds
        #[serde(
            rename = "bookmarkPosition",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bookmark_position: ::std::option::Option<i64>,
        ///The BPM of the song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bpm: ::std::option::Option<i64>,
        ///The number of channels of the media.
        #[serde(
            rename = "channelCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub channel_count: ::std::option::Option<i64>,
        ///The comment tag of the song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///The mimeType of the media.
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<::std::string::String>,
        ///The list of all contributor artists of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub contributors: ::std::vec::Vec<Contributor>,
        ///The coverArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Date the media was created. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The disc number.
        #[serde(
            rename = "discNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disc_number: ::std::option::Option<i64>,
        ///The single value display album artist.
        #[serde(
            rename = "displayAlbumArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_album_artist: ::std::option::Option<::std::string::String>,
        ///The single value display artist.
        #[serde(
            rename = "displayArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_artist: ::std::option::Option<::std::string::String>,
        ///The single value display composer.
        #[serde(
            rename = "displayComposer",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_composer: ::std::option::Option<::std::string::String>,
        ///The duration of the media in seconds.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<i64>,
        #[serde(
            rename = "explicitStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub explicit_status: ::std::option::Option<ExplicitStatus>,
        ///The media genre
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///The list of all genres of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub genres: ::std::vec::Vec<ItemGenre>,
        ///The id of the media.
        pub id: ::std::string::String,
        ///The media is a directory
        #[serde(rename = "isDir")]
        pub is_dir: bool,
        ///Media is a video
        #[serde(
            rename = "isVideo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_video: ::std::option::Option<bool>,
        ///The track ISRC(s).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub isrc: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "mediaType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub media_type: ::std::option::Option<MediaType>,
        ///Last update
        #[serde(rename = "minutesAgo")]
        pub minutes_ago: i64,
        ///The list of all moods of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub moods: ::std::vec::Vec<::std::string::String>,
        ///The track MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The video original Height
        #[serde(
            rename = "originalHeight",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_height: ::std::option::Option<i64>,
        ///The video original Width
        #[serde(
            rename = "originalWidth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_width: ::std::option::Option<i64>,
        ///The id of the parent (folder/album)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<::std::string::String>,
        ///The full path of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        ///The play count.
        #[serde(
            rename = "playCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub play_count: ::std::option::Option<i64>,
        ///Date the album was last played. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub played: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Player Id
        #[serde(rename = "playerId")]
        pub player_id: i64,
        ///Player name
        #[serde(
            rename = "playerName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub player_name: ::std::option::Option<::std::string::String>,
        ///The replay gain data of the song.
        #[serde(
            rename = "replayGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub replay_gain: ::std::option::Option<ReplayGain>,
        ///The sampling rate of the media.
        #[serde(
            rename = "samplingRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sampling_rate: ::std::option::Option<i64>,
        ///A file size of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
        ///The song sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the media was starred. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The file suffix of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suffix: ::std::option::Option<::std::string::String>,
        ///The media name.
        pub title: ::std::string::String,
        ///The track number.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub track: ::std::option::Option<i64>,
        ///The transcoded mediaType if transcoding should happen.
        #[serde(
            rename = "transcodedContentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transcoded_content_type: ::std::option::Option<::std::string::String>,
        ///The file suffix of the transcoded media.
        #[serde(
            rename = "transcodedSuffix",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transcoded_suffix: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<GenericMediaType>,
        ///The user rating of the media [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
        ///The username
        pub username: ::std::string::String,
        ///The media year.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&NowPlayingEntry> for NowPlayingEntry {
        fn from(value: &NowPlayingEntry) -> Self {
            value.clone()
        }
    }
    ///A supported OpenSubsonic API extension.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A supported OpenSubsonic API extension.",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "versions"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "The name of the extension.",
    ///      "type": "string"
    ///    },
    ///    "versions": {
    ///      "description": "The list of supported versions of the this extension.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OpenSubsonicExtension {
        ///The name of the extension.
        pub name: ::std::string::String,
        ///The list of supported versions of the this extension.
        pub versions: ::std::vec::Vec<i64>,
    }
    impl ::std::convert::From<&OpenSubsonicExtension> for OpenSubsonicExtension {
        fn from(value: &OpenSubsonicExtension) -> Self {
            value.clone()
        }
    }
    ///NowPlayingEntry.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NowPlayingEntry.",
    ///  "type": "object",
    ///  "required": [
    ///    "changed",
    ///    "changedBy",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "changed": {
    ///      "description": "Date modified [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "changedBy": {
    ///      "description": "Name of client app",
    ///      "type": "string"
    ///    },
    ///    "current": {
    ///      "description": "ID of currently playing track. This will be provided if one or more entries exists",
    ///      "type": "string"
    ///    },
    ///    "entry": {
    ///      "description": "The list of songs in the queue",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "position": {
    ///      "description": "Position in milliseconds of currently playing track. If not provided, treat this value as 0",
    ///      "type": "integer"
    ///    },
    ///    "username": {
    ///      "description": "The user this queue belongs to",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlayQueue {
        ///Date modified [ISO 8601]
        pub changed: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Name of client app
        #[serde(rename = "changedBy")]
        pub changed_by: ::std::string::String,
        ///ID of currently playing track. This will be provided if one or more entries exists
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub current: ::std::option::Option<::std::string::String>,
        ///The list of songs in the queue
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub entry: ::std::vec::Vec<Child>,
        ///Position in milliseconds of currently playing track. If not provided, treat this value as 0
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<i64>,
        ///The user this queue belongs to
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&PlayQueue> for PlayQueue {
        fn from(value: &PlayQueue) -> Self {
            value.clone()
        }
    }
    ///NowPlayingEntry with queue index.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NowPlayingEntry with queue index.",
    ///  "type": "object",
    ///  "required": [
    ///    "changed",
    ///    "changedBy",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "changed": {
    ///      "description": "Date modified [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "changedBy": {
    ///      "description": "Name of client app",
    ///      "type": "string"
    ///    },
    ///    "currentIndex": {
    ///      "description": "Index of currently playing track.  This must be provided if one or more entries exists",
    ///      "type": "integer"
    ///    },
    ///    "entry": {
    ///      "description": "The list of songs in the queue",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "position": {
    ///      "description": "Position in milliseconds of currently playing track. If not provided, treat this value as 0",
    ///      "type": "integer"
    ///    },
    ///    "username": {
    ///      "description": "The user this queue belongs to",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlayQueueByIndex {
        ///Date modified [ISO 8601]
        pub changed: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Name of client app
        #[serde(rename = "changedBy")]
        pub changed_by: ::std::string::String,
        ///Index of currently playing track.  This must be provided if one or more entries exists
        #[serde(
            rename = "currentIndex",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_index: ::std::option::Option<i64>,
        ///The list of songs in the queue
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub entry: ::std::vec::Vec<Child>,
        ///Position in milliseconds of currently playing track. If not provided, treat this value as 0
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<i64>,
        ///The user this queue belongs to
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&PlayQueueByIndex> for PlayQueueByIndex {
        fn from(value: &PlayQueueByIndex) -> Self {
            value.clone()
        }
    }
    ///Playlist.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Playlist.",
    ///  "type": "object",
    ///  "required": [
    ///    "changed",
    ///    "created",
    ///    "duration",
    ///    "id",
    ///    "name",
    ///    "songCount"
    ///  ],
    ///  "properties": {
    ///    "allowedUser": {
    ///      "description": "A list of allowed usernames",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "changed": {
    ///      "description": "Last changed date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "comment": {
    ///      "description": "A comment",
    ///      "type": "string"
    ///    },
    ///    "coverArt": {
    ///      "description": "A cover Art Id",
    ///      "type": "string"
    ///    },
    ///    "created": {
    ///      "description": "Creation date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "duration": {
    ///      "description": "Playlist duration in seconds",
    ///      "type": "integer"
    ///    },
    ///    "id": {
    ///      "description": "Id of the playlist",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Name of the playlist",
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "description": "Owner of the playlist",
    ///      "type": "string"
    ///    },
    ///    "public": {
    ///      "description": "Is the playlist public",
    ///      "type": "boolean"
    ///    },
    ///    "songCount": {
    ///      "description": "number of songs",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Playlist {
        ///A list of allowed usernames
        #[serde(
            rename = "allowedUser",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub allowed_user: ::std::vec::Vec<::std::string::String>,
        ///Last changed date [ISO 8601]
        pub changed: ::chrono::DateTime<::chrono::offset::Utc>,
        ///A comment
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///A cover Art Id
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Creation date [ISO 8601]
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Playlist duration in seconds
        pub duration: i64,
        ///Id of the playlist
        pub id: ::std::string::String,
        ///Name of the playlist
        pub name: ::std::string::String,
        ///Owner of the playlist
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<::std::string::String>,
        ///Is the playlist public
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public: ::std::option::Option<bool>,
        ///number of songs
        #[serde(rename = "songCount")]
        pub song_count: i64,
    }
    impl ::std::convert::From<&Playlist> for Playlist {
        fn from(value: &Playlist) -> Self {
            value.clone()
        }
    }
    ///`PlaylistWithSongs`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Playlist"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "entry": {
    ///          "description": "The list of songs",
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/Child"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlaylistWithSongs {
        ///A list of allowed usernames
        #[serde(
            rename = "allowedUser",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub allowed_user: ::std::vec::Vec<::std::string::String>,
        ///Last changed date [ISO 8601]
        pub changed: ::chrono::DateTime<::chrono::offset::Utc>,
        ///A comment
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///A cover Art Id
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Creation date [ISO 8601]
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Playlist duration in seconds
        pub duration: i64,
        ///The list of songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub entry: ::std::vec::Vec<Child>,
        ///Id of the playlist
        pub id: ::std::string::String,
        ///Name of the playlist
        pub name: ::std::string::String,
        ///Owner of the playlist
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<::std::string::String>,
        ///Is the playlist public
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public: ::std::option::Option<bool>,
        ///number of songs
        #[serde(rename = "songCount")]
        pub song_count: i64,
    }
    impl ::std::convert::From<&PlaylistWithSongs> for PlaylistWithSongs {
        fn from(value: &PlaylistWithSongs) -> Self {
            value.clone()
        }
    }
    ///Playlists.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Playlists.",
    ///  "type": "object",
    ///  "properties": {
    ///    "playlist": {
    ///      "description": "The playlists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Playlist"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Playlists {
        ///The playlists
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub playlist: ::std::vec::Vec<Playlist>,
    }
    impl ::std::convert::From<&Playlists> for Playlists {
        fn from(value: &Playlists) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Playlists {
        fn default() -> Self {
            Self {
                playlist: Default::default(),
            }
        }
    }
    ///A Podcast channel
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Podcast channel",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "status",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "coverArt": {
    ///      "description": "ID used for retrieving cover art",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "The channel description",
    ///      "type": "string"
    ///    },
    ///    "episode": {
    ///      "description": "Podcast episodes with this channel",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PodcastEpisode"
    ///      }
    ///    },
    ///    "errorMessage": {
    ///      "description": "An error message",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "The channel ID",
    ///      "type": "string"
    ///    },
    ///    "originalImageUrl": {
    ///      "description": "URL for original image of podcast channel",
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/PodcastStatus"
    ///    },
    ///    "title": {
    ///      "description": "The channel title",
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "description": "Podcast channel URL",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PodcastChannel {
        ///ID used for retrieving cover art
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///The channel description
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Podcast episodes with this channel
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub episode: ::std::vec::Vec<PodcastEpisode>,
        ///An error message
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<::std::string::String>,
        ///The channel ID
        pub id: ::std::string::String,
        ///URL for original image of podcast channel
        #[serde(
            rename = "originalImageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_image_url: ::std::option::Option<::std::string::String>,
        pub status: PodcastStatus,
        ///The channel title
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        ///Podcast channel URL
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&PodcastChannel> for PodcastChannel {
        fn from(value: &PodcastChannel) -> Self {
            value.clone()
        }
    }
    ///`PodcastEpisode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Child"
    ///    },
    ///    {
    ///      "description": "A podcast episode.",
    ///      "type": "object",
    ///      "required": [
    ///        "channelId",
    ///        "status"
    ///      ],
    ///      "properties": {
    ///        "channelId": {
    ///          "description": "TID of the podcast channel",
    ///          "type": "string"
    ///        },
    ///        "description": {
    ///          "description": "Episode description",
    ///          "type": "string"
    ///        },
    ///        "publishDate": {
    ///          "description": "Date the episode was published [ISO 8601]",
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "status": {
    ///          "$ref": "#/components/schemas/PodcastStatus"
    ///        },
    ///        "streamId": {
    ///          "description": "ID used for streaming podcast",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PodcastEpisode {
        ///The album name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub album: ::std::option::Option<::std::string::String>,
        ///The list of all album artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)
        #[serde(
            rename = "albumArtists",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub album_artists: ::std::vec::Vec<ArtistId3>,
        ///The corresponding album id
        #[serde(
            rename = "albumId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_id: ::std::option::Option<::std::string::String>,
        ///The artist name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The corresponding artist id
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_id: ::std::option::Option<::std::string::String>,
        ///The list of all song artists of the song. (Note: Only the required `ArtistID3` fields should be returned by default)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artists: ::std::vec::Vec<ArtistId3>,
        #[serde(
            rename = "averageRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_rating: ::std::option::Option<f64>,
        ///The bit depth of the media.
        #[serde(
            rename = "bitDepth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_depth: ::std::option::Option<i64>,
        ///The bitrate of the media.
        #[serde(
            rename = "bitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_rate: ::std::option::Option<i64>,
        ///The bookmark position in seconds
        #[serde(
            rename = "bookmarkPosition",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bookmark_position: ::std::option::Option<i64>,
        ///The BPM of the song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bpm: ::std::option::Option<i64>,
        ///The number of channels of the media.
        #[serde(
            rename = "channelCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub channel_count: ::std::option::Option<i64>,
        ///TID of the podcast channel
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
        ///The comment tag of the song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///The mimeType of the media.
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<::std::string::String>,
        ///The list of all contributor artists of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub contributors: ::std::vec::Vec<Contributor>,
        ///The coverArt id.
        #[serde(
            rename = "coverArt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cover_art: ::std::option::Option<::std::string::String>,
        ///Date the media was created. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Episode description
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///The disc number.
        #[serde(
            rename = "discNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disc_number: ::std::option::Option<i64>,
        ///The single value display album artist.
        #[serde(
            rename = "displayAlbumArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_album_artist: ::std::option::Option<::std::string::String>,
        ///The single value display artist.
        #[serde(
            rename = "displayArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_artist: ::std::option::Option<::std::string::String>,
        ///The single value display composer.
        #[serde(
            rename = "displayComposer",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_composer: ::std::option::Option<::std::string::String>,
        ///The duration of the media in seconds.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<i64>,
        #[serde(
            rename = "explicitStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub explicit_status: ::std::option::Option<ExplicitStatus>,
        ///The media genre
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///The list of all genres of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub genres: ::std::vec::Vec<ItemGenre>,
        ///The id of the media.
        pub id: ::std::string::String,
        ///The media is a directory
        #[serde(rename = "isDir")]
        pub is_dir: bool,
        ///Media is a video
        #[serde(
            rename = "isVideo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_video: ::std::option::Option<bool>,
        ///The track ISRC(s).
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub isrc: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "mediaType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub media_type: ::std::option::Option<MediaType>,
        ///The list of all moods of the song.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub moods: ::std::vec::Vec<::std::string::String>,
        ///The track MusicBrainzID.
        #[serde(
            rename = "musicBrainzId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_brainz_id: ::std::option::Option<::std::string::String>,
        ///The video original Height
        #[serde(
            rename = "originalHeight",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_height: ::std::option::Option<i64>,
        ///The video original Width
        #[serde(
            rename = "originalWidth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_width: ::std::option::Option<i64>,
        ///The id of the parent (folder/album)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<::std::string::String>,
        ///The full path of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        ///The play count.
        #[serde(
            rename = "playCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub play_count: ::std::option::Option<i64>,
        ///Date the album was last played. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub played: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Date the episode was published [ISO 8601]
        #[serde(
            rename = "publishDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub publish_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The replay gain data of the song.
        #[serde(
            rename = "replayGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub replay_gain: ::std::option::Option<ReplayGain>,
        ///The sampling rate of the media.
        #[serde(
            rename = "samplingRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sampling_rate: ::std::option::Option<i64>,
        ///A file size of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
        ///The song sort name.
        #[serde(
            rename = "sortName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sort_name: ::std::option::Option<::std::string::String>,
        ///Date the media was starred. [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starred: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub status: PodcastStatus,
        ///ID used for streaming podcast
        #[serde(
            rename = "streamId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stream_id: ::std::option::Option<::std::string::String>,
        ///The file suffix of the media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suffix: ::std::option::Option<::std::string::String>,
        ///The media name.
        pub title: ::std::string::String,
        ///The track number.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub track: ::std::option::Option<i64>,
        ///The transcoded mediaType if transcoding should happen.
        #[serde(
            rename = "transcodedContentType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transcoded_content_type: ::std::option::Option<::std::string::String>,
        ///The file suffix of the transcoded media.
        #[serde(
            rename = "transcodedSuffix",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transcoded_suffix: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<GenericMediaType>,
        ///The user rating of the media [1-5]
        #[serde(
            rename = "userRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_rating: ::std::option::Option<i64>,
        ///The media year.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&PodcastEpisode> for PodcastEpisode {
        fn from(value: &PodcastEpisode) -> Self {
            value.clone()
        }
    }
    ///An enumeration of possible podcast statuses
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An enumeration of possible podcast statuses",
    ///  "type": "string",
    ///  "enum": [
    ///    "new",
    ///    "downloading",
    ///    "completed",
    ///    "error",
    ///    "deleted",
    ///    "skipped"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PodcastStatus {
        #[serde(rename = "new")]
        New,
        #[serde(rename = "downloading")]
        Downloading,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(rename = "skipped")]
        Skipped,
    }
    impl ::std::convert::From<&Self> for PodcastStatus {
        fn from(value: &PodcastStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PodcastStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::New => f.write_str("new"),
                Self::Downloading => f.write_str("downloading"),
                Self::Completed => f.write_str("completed"),
                Self::Error => f.write_str("error"),
                Self::Deleted => f.write_str("deleted"),
                Self::Skipped => f.write_str("skipped"),
            }
        }
    }
    impl ::std::str::FromStr for PodcastStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "new" => Ok(Self::New),
                "downloading" => Ok(Self::Downloading),
                "completed" => Ok(Self::Completed),
                "error" => Ok(Self::Error),
                "deleted" => Ok(Self::Deleted),
                "skipped" => Ok(Self::Skipped),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PodcastStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PodcastStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PodcastStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Podcasts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Podcasts.",
    ///  "type": "object",
    ///  "properties": {
    ///    "channel": {
    ///      "description": "Podcast channel(s)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PodcastChannel"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Podcasts {
        ///Podcast channel(s)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub channel: ::std::vec::Vec<PodcastChannel>,
    }
    impl ::std::convert::From<&Podcasts> for Podcasts {
        fn from(value: &Podcasts) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Podcasts {
        fn default() -> Self {
            Self {
                channel: Default::default(),
            }
        }
    }
    ///`PostAddChatMessageBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "The chat message.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostAddChatMessageBody {
        ///The chat message.
        pub message: ::std::string::String,
    }
    impl ::std::convert::From<&PostAddChatMessageBody> for PostAddChatMessageBody {
        fn from(value: &PostAddChatMessageBody) -> Self {
            value.clone()
        }
    }
    ///`PostChangePasswordBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "password",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "password": {
    ///      "description": "The new password of the new user, either in clear text of hex-encoded (see above).",
    ///      "type": "string"
    ///    },
    ///    "username": {
    ///      "description": "The name of the user which should change its password.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostChangePasswordBody {
        ///The new password of the new user, either in clear text of hex-encoded (see above).
        pub password: ::std::string::String,
        ///The name of the user which should change its password.
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&PostChangePasswordBody> for PostChangePasswordBody {
        fn from(value: &PostChangePasswordBody) -> Self {
            value.clone()
        }
    }
    ///`PostCreateBookmarkBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "position"
    ///  ],
    ///  "properties": {
    ///    "comment": {
    ///      "description": "A user-defined comment.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "ID of the media file to bookmark. If a bookmark already exists for this file it will be overwritten.",
    ///      "type": "string"
    ///    },
    ///    "position": {
    ///      "description": "The position (in milliseconds) within the media file.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostCreateBookmarkBody {
        ///A user-defined comment.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///ID of the media file to bookmark. If a bookmark already exists for this file it will be overwritten.
        pub id: ::std::string::String,
        ///The position (in milliseconds) within the media file.
        pub position: i64,
    }
    impl ::std::convert::From<&PostCreateBookmarkBody> for PostCreateBookmarkBody {
        fn from(value: &PostCreateBookmarkBody) -> Self {
            value.clone()
        }
    }
    ///`PostCreateInternetRadioStationBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "streamUrl"
    ///  ],
    ///  "properties": {
    ///    "homepageUrl": {
    ///      "description": "The home page URL for the station.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The station name.",
    ///      "type": "string"
    ///    },
    ///    "streamUrl": {
    ///      "description": "The stream URL for the station.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostCreateInternetRadioStationBody {
        ///The home page URL for the station.
        #[serde(
            rename = "homepageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub homepage_url: ::std::option::Option<::std::string::String>,
        ///The station name.
        pub name: ::std::string::String,
        ///The stream URL for the station.
        #[serde(rename = "streamUrl")]
        pub stream_url: ::std::string::String,
    }
    impl ::std::convert::From<&PostCreateInternetRadioStationBody>
        for PostCreateInternetRadioStationBody
    {
        fn from(value: &PostCreateInternetRadioStationBody) -> Self {
            value.clone()
        }
    }
    ///`PostCreatePlaylistBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "description": "The human-readable name of the playlist. Required if creating a new playlist.",
    ///      "type": "string"
    ///    },
    ///    "playlistId": {
    ///      "description": "The playlist ID. Required if updating an existing playlist.",
    ///      "type": "string"
    ///    },
    ///    "songId": {
    ///      "description": "ID of a song in the playlist. Use one `songId` parameter for each song in the playlist.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostCreatePlaylistBody {
        ///The human-readable name of the playlist. Required if creating a new playlist.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The playlist ID. Required if updating an existing playlist.
        #[serde(
            rename = "playlistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub playlist_id: ::std::option::Option<::std::string::String>,
        ///ID of a song in the playlist. Use one `songId` parameter for each song in the playlist.
        #[serde(
            rename = "songId",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub song_id: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::convert::From<&PostCreatePlaylistBody> for PostCreatePlaylistBody {
        fn from(value: &PostCreatePlaylistBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostCreatePlaylistBody {
        fn default() -> Self {
            Self {
                name: Default::default(),
                playlist_id: Default::default(),
                song_id: Default::default(),
            }
        }
    }
    ///`PostCreatePodcastChannelBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "url": {
    ///      "description": "The URL of the Podcast to add.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostCreatePodcastChannelBody {
        ///The URL of the Podcast to add.
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&PostCreatePodcastChannelBody> for PostCreatePodcastChannelBody {
        fn from(value: &PostCreatePodcastChannelBody) -> Self {
            value.clone()
        }
    }
    ///`PostCreateShareBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "A user-defined description that will be displayed to people visiting the shared media.",
    ///      "type": "string"
    ///    },
    ///    "expires": {
    ///      "description": "The time at which the share expires. Given as milliseconds since 1970.",
    ///      "type": "integer"
    ///    },
    ///    "id": {
    ///      "description": "ID of a song, album or video to share. Use one id parameter for each entry to share.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostCreateShareBody {
        ///A user-defined description that will be displayed to people visiting the shared media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///The time at which the share expires. Given as milliseconds since 1970.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires: ::std::option::Option<i64>,
        ///ID of a song, album or video to share. Use one id parameter for each entry to share.
        pub id: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::convert::From<&PostCreateShareBody> for PostCreateShareBody {
        fn from(value: &PostCreateShareBody) -> Self {
            value.clone()
        }
    }
    ///`PostCreateUserBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "password",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "adminRole": {
    ///      "description": "Whether the user is administrator.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "commentRole": {
    ///      "description": "Whether the user is allowed to create and edit comments and ratings.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "coverArtRole": {
    ///      "description": "Whether the user is allowed to change cover art and tags.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "downloadRole": {
    ///      "description": "Whether the user is allowed to download files.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "email": {
    ///      "description": "The email address of the new user.",
    ///      "type": "string"
    ///    },
    ///    "jukeboxRole": {
    ///      "description": "Whether the user is allowed to play files in jukebox mode.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "ldapAuthenticated": {
    ///      "description": "Whether the user is authenticated in LDAP.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.12.0) IDs of the music folders the user is allowed access to. Include the parameter once for each folder. Default all folders.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "password": {
    ///      "description": "The password of the new user, either in clear text or hex-encoded.",
    ///      "type": "string"
    ///    },
    ///    "playlistRole": {
    ///      "description": "Whether the user is allowed to create and delete playlists. Since 1.8.0, changing this role has no effect.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "podcastRole": {
    ///      "description": "Whether the user is allowed to administrate Podcasts.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "settingsRole": {
    ///      "description": "Whether the user is allowed to change personal settings and password.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "shareRole": {
    ///      "description": "(Since 1.8.0) Whether the user is allowed to share files with anyone.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "streamRole": {
    ///      "description": "Whether the user is allowed to play files.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "uploadRole": {
    ///      "description": "Whether the user is allowed to upload files.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "username": {
    ///      "description": "The name of the new user.",
    ///      "type": "string"
    ///    },
    ///    "videoConversionRole": {
    ///      "description": "(Since 1.15.0) Whether the user is allowed to start video conversions.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostCreateUserBody {
        ///Whether the user is administrator.
        #[serde(rename = "adminRole", default)]
        pub admin_role: bool,
        ///Whether the user is allowed to create and edit comments and ratings.
        #[serde(rename = "commentRole", default)]
        pub comment_role: bool,
        ///Whether the user is allowed to change cover art and tags.
        #[serde(rename = "coverArtRole", default)]
        pub cover_art_role: bool,
        ///Whether the user is allowed to download files.
        #[serde(rename = "downloadRole", default)]
        pub download_role: bool,
        ///The email address of the new user.
        pub email: ::std::string::String,
        ///Whether the user is allowed to play files in jukebox mode.
        #[serde(rename = "jukeboxRole", default)]
        pub jukebox_role: bool,
        ///Whether the user is authenticated in LDAP.
        #[serde(rename = "ldapAuthenticated", default)]
        pub ldap_authenticated: bool,
        ///(Since 1.12.0) IDs of the music folders the user is allowed access to. Include the parameter once for each folder. Default all folders.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub music_folder_id: ::std::vec::Vec<::std::string::String>,
        ///The password of the new user, either in clear text or hex-encoded.
        pub password: ::std::string::String,
        ///Whether the user is allowed to create and delete playlists. Since 1.8.0, changing this role has no effect.
        #[serde(rename = "playlistRole", default)]
        pub playlist_role: bool,
        ///Whether the user is allowed to administrate Podcasts.
        #[serde(rename = "podcastRole", default)]
        pub podcast_role: bool,
        ///Whether the user is allowed to change personal settings and password.
        #[serde(rename = "settingsRole", default = "defaults::default_bool::<true>")]
        pub settings_role: bool,
        ///(Since 1.8.0) Whether the user is allowed to share files with anyone.
        #[serde(rename = "shareRole", default)]
        pub share_role: bool,
        ///Whether the user is allowed to play files.
        #[serde(rename = "streamRole", default = "defaults::default_bool::<true>")]
        pub stream_role: bool,
        ///Whether the user is allowed to upload files.
        #[serde(rename = "uploadRole", default)]
        pub upload_role: bool,
        ///The name of the new user.
        pub username: ::std::string::String,
        ///(Since 1.15.0) Whether the user is allowed to start video conversions.
        #[serde(rename = "videoConversionRole", default)]
        pub video_conversion_role: bool,
    }
    impl ::std::convert::From<&PostCreateUserBody> for PostCreateUserBody {
        fn from(value: &PostCreateUserBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeleteBookmarkBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "ID of the media file for which to delete the bookmark. Other users’ bookmarks are not affected.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeleteBookmarkBody {
        ///ID of the media file for which to delete the bookmark. Other users’ bookmarks are not affected.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeleteBookmarkBody> for PostDeleteBookmarkBody {
        fn from(value: &PostDeleteBookmarkBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeleteInternetRadioStationBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The ID for the station.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeleteInternetRadioStationBody {
        ///The ID for the station.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeleteInternetRadioStationBody>
        for PostDeleteInternetRadioStationBody
    {
        fn from(value: &PostDeleteInternetRadioStationBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeletePlaylistBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "ID of the playlist to delete, as obtained by `getPlaylists`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeletePlaylistBody {
        ///ID of the playlist to delete, as obtained by `getPlaylists`.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeletePlaylistBody> for PostDeletePlaylistBody {
        fn from(value: &PostDeletePlaylistBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeletePodcastChannelBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The ID of the Podcast channel to delete.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeletePodcastChannelBody {
        ///The ID of the Podcast channel to delete.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeletePodcastChannelBody> for PostDeletePodcastChannelBody {
        fn from(value: &PostDeletePodcastChannelBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeletePodcastEpisodeBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The ID of the Podcast episode to delete.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeletePodcastEpisodeBody {
        ///The ID of the Podcast episode to delete.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeletePodcastEpisodeBody> for PostDeletePodcastEpisodeBody {
        fn from(value: &PostDeletePodcastEpisodeBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeleteShareBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "ID of the share to delete.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeleteShareBody {
        ///ID of the share to delete.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeleteShareBody> for PostDeleteShareBody {
        fn from(value: &PostDeleteShareBody) -> Self {
            value.clone()
        }
    }
    ///`PostDeleteUserBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "username": {
    ///      "description": "The name of the user to delete.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDeleteUserBody {
        ///The name of the user to delete.
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&PostDeleteUserBody> for PostDeleteUserBody {
        fn from(value: &PostDeleteUserBody) -> Self {
            value.clone()
        }
    }
    ///`PostDownloadBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "A string which uniquely identifies the file to download. Obtained by calls to getMusicDirectory.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDownloadBody {
        ///A string which uniquely identifies the file to download. Obtained by calls to getMusicDirectory.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDownloadBody> for PostDownloadBody {
        fn from(value: &PostDownloadBody) -> Self {
            value.clone()
        }
    }
    ///`PostDownloadPodcastEpisodeBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The ID of the Podcast episode to download.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostDownloadPodcastEpisodeBody {
        ///The ID of the Podcast episode to download.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostDownloadPodcastEpisodeBody> for PostDownloadPodcastEpisodeBody {
        fn from(value: &PostDownloadPodcastEpisodeBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetAlbumBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The album ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetAlbumBody {
        ///The album ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetAlbumBody> for PostGetAlbumBody {
        fn from(value: &PostGetAlbumBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetAlbumInfo2Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The album ID or song ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetAlbumInfo2Body {
        ///The album ID or song ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetAlbumInfo2Body> for PostGetAlbumInfo2Body {
        fn from(value: &PostGetAlbumInfo2Body) -> Self {
            value.clone()
        }
    }
    ///`PostGetAlbumInfoBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The album ID or song ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetAlbumInfoBody {
        ///The album ID or song ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetAlbumInfoBody> for PostGetAlbumInfoBody {
        fn from(value: &PostGetAlbumInfoBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetAlbumList2Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "fromYear": {
    ///      "description": "Required if `type=='byYear'`. The first year in the range. If `fromYear` > `toYear` a reverse chronological list is returned.",
    ///      "type": "integer"
    ///    },
    ///    "genre": {
    ///      "description": "Required if `type=='byGenre'`. The name of the genre, e.g., “Rock”.",
    ///      "type": "string"
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.11.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    },
    ///    "offset": {
    ///      "description": "The list offset. Useful if you for example want to page through the list of newest albums.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "size": {
    ///      "description": "The number of albums to return. Max 500.",
    ///      "default": 10,
    ///      "type": "integer",
    ///      "maximum": 500.0,
    ///      "minimum": 1.0
    ///    },
    ///    "toYear": {
    ///      "description": "Required if `type=='byYear'`. The last year in the range.",
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "$ref": "#/components/schemas/AlbumListType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetAlbumList2Body {
        ///Required if `type=='byYear'`. The first year in the range. If `fromYear` > `toYear` a reverse chronological list is returned.
        #[serde(
            rename = "fromYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub from_year: ::std::option::Option<i64>,
        ///Required if `type=='byGenre'`. The name of the genre, e.g., “Rock”.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///(Since 1.11.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
        ///The list offset. Useful if you for example want to page through the list of newest albums.
        #[serde(default)]
        pub offset: u64,
        ///The number of albums to return. Max 500.
        #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU64, 10>")]
        pub size: ::std::num::NonZeroU64,
        ///Required if `type=='byYear'`. The last year in the range.
        #[serde(
            rename = "toYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub to_year: ::std::option::Option<i64>,
        #[serde(rename = "type")]
        pub type_: AlbumListType,
    }
    impl ::std::convert::From<&PostGetAlbumList2Body> for PostGetAlbumList2Body {
        fn from(value: &PostGetAlbumList2Body) -> Self {
            value.clone()
        }
    }
    ///`PostGetAlbumListBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "fromYear": {
    ///      "description": "Required if `type=='byYear'`. The first year in the range. If `fromYear` > `toYear` a reverse chronological list is returned.",
    ///      "type": "integer"
    ///    },
    ///    "genre": {
    ///      "description": "Required if `type=='byGenre'`. The name of the genre, e.g., “Rock”.",
    ///      "type": "string"
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.11.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    },
    ///    "offset": {
    ///      "description": "The list offset. Useful if you for example want to page through the list of newest albums.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "size": {
    ///      "description": "The number of albums to return. Max 500.",
    ///      "default": 10,
    ///      "type": "integer",
    ///      "maximum": 500.0,
    ///      "minimum": 1.0
    ///    },
    ///    "toYear": {
    ///      "description": "Required if `type=='byYear'`. The last year in the range.",
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "$ref": "#/components/schemas/AlbumListType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetAlbumListBody {
        ///Required if `type=='byYear'`. The first year in the range. If `fromYear` > `toYear` a reverse chronological list is returned.
        #[serde(
            rename = "fromYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub from_year: ::std::option::Option<i64>,
        ///Required if `type=='byGenre'`. The name of the genre, e.g., “Rock”.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///(Since 1.11.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
        ///The list offset. Useful if you for example want to page through the list of newest albums.
        #[serde(default)]
        pub offset: u64,
        ///The number of albums to return. Max 500.
        #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU64, 10>")]
        pub size: ::std::num::NonZeroU64,
        ///Required if `type=='byYear'`. The last year in the range.
        #[serde(
            rename = "toYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub to_year: ::std::option::Option<i64>,
        #[serde(rename = "type")]
        pub type_: AlbumListType,
    }
    impl ::std::convert::From<&PostGetAlbumListBody> for PostGetAlbumListBody {
        fn from(value: &PostGetAlbumListBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetArtistBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The artist ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetArtistBody {
        ///The artist ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetArtistBody> for PostGetArtistBody {
        fn from(value: &PostGetArtistBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetArtistInfo2Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "Max number of similar artists to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "description": "The artist, album or song ID.",
    ///      "type": "string"
    ///    },
    ///    "includeNotPresent": {
    ///      "description": "Whether to return artists that are not present in the media library.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetArtistInfo2Body {
        ///Max number of similar artists to return.
        #[serde(default = "defaults::default_u64::<u64, 20>")]
        pub count: u64,
        ///The artist, album or song ID.
        pub id: ::std::string::String,
        ///Whether to return artists that are not present in the media library.
        #[serde(rename = "includeNotPresent", default)]
        pub include_not_present: bool,
    }
    impl ::std::convert::From<&PostGetArtistInfo2Body> for PostGetArtistInfo2Body {
        fn from(value: &PostGetArtistInfo2Body) -> Self {
            value.clone()
        }
    }
    ///`PostGetArtistInfoBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "Max number of similar artists to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "description": "The artist, album or song ID.",
    ///      "type": "string"
    ///    },
    ///    "includeNotPresent": {
    ///      "description": "Whether to return artists that are not present in the media library.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetArtistInfoBody {
        ///Max number of similar artists to return.
        #[serde(default = "defaults::default_u64::<u64, 20>")]
        pub count: u64,
        ///The artist, album or song ID.
        pub id: ::std::string::String,
        ///Whether to return artists that are not present in the media library.
        #[serde(rename = "includeNotPresent", default)]
        pub include_not_present: bool,
    }
    impl ::std::convert::From<&PostGetArtistInfoBody> for PostGetArtistInfoBody {
        fn from(value: &PostGetArtistInfoBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetArtistsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "musicFolderId": {
    ///      "description": "If specified, only return artists in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetArtistsBody {
        ///If specified, only return artists in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PostGetArtistsBody> for PostGetArtistsBody {
        fn from(value: &PostGetArtistsBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetArtistsBody {
        fn default() -> Self {
            Self {
                music_folder_id: Default::default(),
            }
        }
    }
    ///`PostGetAvatarBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "username": {
    ///      "description": "The username for which to retrieve the avatar.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetAvatarBody {
        ///The username for which to retrieve the avatar.
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetAvatarBody> for PostGetAvatarBody {
        fn from(value: &PostGetAvatarBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetCaptionsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "format": {
    ///      "description": "Preferred captions format (“srt” or “vtt”).",
    ///      "type": "string",
    ///      "enum": [
    ///        "srt",
    ///        "vtt"
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "The ID of the video.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetCaptionsBody {
        ///Preferred captions format (“srt” or “vtt”).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub format: ::std::option::Option<PostGetCaptionsBodyFormat>,
        ///The ID of the video.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetCaptionsBody> for PostGetCaptionsBody {
        fn from(value: &PostGetCaptionsBody) -> Self {
            value.clone()
        }
    }
    ///Preferred captions format (“srt” or “vtt”).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Preferred captions format (“srt” or “vtt”).",
    ///  "type": "string",
    ///  "enum": [
    ///    "srt",
    ///    "vtt"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostGetCaptionsBodyFormat {
        #[serde(rename = "srt")]
        Srt,
        #[serde(rename = "vtt")]
        Vtt,
    }
    impl ::std::convert::From<&Self> for PostGetCaptionsBodyFormat {
        fn from(value: &PostGetCaptionsBodyFormat) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostGetCaptionsBodyFormat {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Srt => f.write_str("srt"),
                Self::Vtt => f.write_str("vtt"),
            }
        }
    }
    impl ::std::str::FromStr for PostGetCaptionsBodyFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "srt" => Ok(Self::Srt),
                "vtt" => Ok(Self::Vtt),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostGetCaptionsBodyFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostGetCaptionsBodyFormat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostGetCaptionsBodyFormat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PostGetCoverArtBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The coverArt ID. Returned by most entities likes `Child` or `AlbumID3`",
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "description": "If specified, scale image to this size.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetCoverArtBody {
        ///The coverArt ID. Returned by most entities likes `Child` or `AlbumID3`
        pub id: ::std::string::String,
        ///If specified, scale image to this size.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&PostGetCoverArtBody> for PostGetCoverArtBody {
        fn from(value: &PostGetCoverArtBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetIndexesBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ifModifiedSince": {
    ///      "description": "If specified, only return a result if the artist collection has changed since the given time (in milliseconds since 1 Jan 1970).",
    ///      "type": "integer"
    ///    },
    ///    "musicFolderId": {
    ///      "description": "If specified, only return artists in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetIndexesBody {
        ///If specified, only return a result if the artist collection has changed since the given time (in milliseconds since 1 Jan 1970).
        #[serde(
            rename = "ifModifiedSince",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub if_modified_since: ::std::option::Option<i64>,
        ///If specified, only return artists in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PostGetIndexesBody> for PostGetIndexesBody {
        fn from(value: &PostGetIndexesBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetIndexesBody {
        fn default() -> Self {
            Self {
                if_modified_since: Default::default(),
                music_folder_id: Default::default(),
            }
        }
    }
    ///`PostGetLyricsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "artist": {
    ///      "description": "The artist name.",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "The song title.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetLyricsBody {
        ///The artist name.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///The song title.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PostGetLyricsBody> for PostGetLyricsBody {
        fn from(value: &PostGetLyricsBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetLyricsBody {
        fn default() -> Self {
            Self {
                artist: Default::default(),
                title: Default::default(),
            }
        }
    }
    ///`PostGetLyricsBySongIdBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The track ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetLyricsBySongIdBody {
        ///The track ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetLyricsBySongIdBody> for PostGetLyricsBySongIdBody {
        fn from(value: &PostGetLyricsBySongIdBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetMusicDirectoryBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "A string which uniquely identifies the music folder. Obtained by calls to `getIndexes` or `getMusicDirectory`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetMusicDirectoryBody {
        ///A string which uniquely identifies the music folder. Obtained by calls to `getIndexes` or `getMusicDirectory`.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetMusicDirectoryBody> for PostGetMusicDirectoryBody {
        fn from(value: &PostGetMusicDirectoryBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetNewestPodcastsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "description": "The maximum number of episodes to return.",
    ///      "default": 20,
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetNewestPodcastsBody {
        ///The maximum number of episodes to return.
        #[serde(default = "defaults::default_u64::<i64, 20>")]
        pub count: i64,
    }
    impl ::std::convert::From<&PostGetNewestPodcastsBody> for PostGetNewestPodcastsBody {
        fn from(value: &PostGetNewestPodcastsBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetNewestPodcastsBody {
        fn default() -> Self {
            Self {
                count: defaults::default_u64::<i64, 20>(),
            }
        }
    }
    ///`PostGetPlaylistBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "ID of the playlist to return, as obtained by `getPlaylists`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetPlaylistBody {
        ///ID of the playlist to return, as obtained by `getPlaylists`.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetPlaylistBody> for PostGetPlaylistBody {
        fn from(value: &PostGetPlaylistBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetPlaylistsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "username": {
    ///      "description": "(Since 1.8.0) If specified, return playlists for this user rather than for the authenticated user. The authenticated user must have admin role if this parameter is used.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetPlaylistsBody {
        ///(Since 1.8.0) If specified, return playlists for this user rather than for the authenticated user. The authenticated user must have admin role if this parameter is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PostGetPlaylistsBody> for PostGetPlaylistsBody {
        fn from(value: &PostGetPlaylistsBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetPlaylistsBody {
        fn default() -> Self {
            Self {
                username: Default::default(),
            }
        }
    }
    ///`PostGetPodcastEpisodeBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The podcast episode ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetPodcastEpisodeBody {
        ///The podcast episode ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetPodcastEpisodeBody> for PostGetPodcastEpisodeBody {
        fn from(value: &PostGetPodcastEpisodeBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetPodcastsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "description": "(Since 1.9.0) If specified, only return the Podcast channel with this ID.",
    ///      "type": "string"
    ///    },
    ///    "includeEpisodes": {
    ///      "description": "(Since 1.9.0) Whether to include Podcast episodes in the returned result.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetPodcastsBody {
        ///(Since 1.9.0) If specified, only return the Podcast channel with this ID.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///(Since 1.9.0) Whether to include Podcast episodes in the returned result.
        #[serde(rename = "includeEpisodes", default = "defaults::default_bool::<true>")]
        pub include_episodes: bool,
    }
    impl ::std::convert::From<&PostGetPodcastsBody> for PostGetPodcastsBody {
        fn from(value: &PostGetPodcastsBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetPodcastsBody {
        fn default() -> Self {
            Self {
                id: Default::default(),
                include_episodes: defaults::default_bool::<true>(),
            }
        }
    }
    ///`PostGetRandomSongsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "fromYear": {
    ///      "description": "(Since 1.9.0) Only return songs from this year or later.",
    ///      "type": "integer"
    ///    },
    ///    "genre": {
    ///      "description": "Only returns songs belonging to this genre.",
    ///      "type": "string"
    ///    },
    ///    "musicFolderId": {
    ///      "description": "Only return songs in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "description": "The maximum number of songs to return. Max 500.",
    ///      "default": 10,
    ///      "type": "integer",
    ///      "maximum": 500.0,
    ///      "minimum": 0.0
    ///    },
    ///    "toYear": {
    ///      "description": "Only return songs published before or in this year.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetRandomSongsBody {
        ///(Since 1.9.0) Only return songs from this year or later.
        #[serde(
            rename = "fromYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub from_year: ::std::option::Option<i64>,
        ///Only returns songs belonging to this genre.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genre: ::std::option::Option<::std::string::String>,
        ///Only return songs in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
        ///The maximum number of songs to return. Max 500.
        #[serde(default = "defaults::default_u64::<i64, 10>")]
        pub size: i64,
        ///Only return songs published before or in this year.
        #[serde(
            rename = "toYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub to_year: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&PostGetRandomSongsBody> for PostGetRandomSongsBody {
        fn from(value: &PostGetRandomSongsBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostGetRandomSongsBody {
        fn default() -> Self {
            Self {
                from_year: Default::default(),
                genre: Default::default(),
                music_folder_id: Default::default(),
                size: defaults::default_u64::<i64, 10>(),
                to_year: Default::default(),
            }
        }
    }
    ///`PostGetSimilarSongs2Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "Max number of songs to return.",
    ///      "default": 50,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "description": "The artist, album or song ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetSimilarSongs2Body {
        ///Max number of songs to return.
        #[serde(default = "defaults::default_u64::<u64, 50>")]
        pub count: u64,
        ///The artist, album or song ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetSimilarSongs2Body> for PostGetSimilarSongs2Body {
        fn from(value: &PostGetSimilarSongs2Body) -> Self {
            value.clone()
        }
    }
    ///`PostGetSimilarSongsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "Max number of songs to return.",
    ///      "default": 50,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "description": "The artist, album or song ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetSimilarSongsBody {
        ///Max number of songs to return.
        #[serde(default = "defaults::default_u64::<u64, 50>")]
        pub count: u64,
        ///The artist, album or song ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetSimilarSongsBody> for PostGetSimilarSongsBody {
        fn from(value: &PostGetSimilarSongsBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetSongBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The song ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetSongBody {
        ///The song ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetSongBody> for PostGetSongBody {
        fn from(value: &PostGetSongBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetSongsByGenreBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "genre"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "The maximum number of songs to return. Max 500.",
    ///      "default": 10,
    ///      "type": "integer",
    ///      "maximum": 500.0,
    ///      "minimum": 0.0
    ///    },
    ///    "genre": {
    ///      "description": "The genre, as returned by `getGenres`.",
    ///      "type": "string"
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    },
    ///    "offset": {
    ///      "description": "The offset. Useful if you want to page through the songs in a genre.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetSongsByGenreBody {
        ///The maximum number of songs to return. Max 500.
        #[serde(default = "defaults::default_u64::<i64, 10>")]
        pub count: i64,
        ///The genre, as returned by `getGenres`.
        pub genre: ::std::string::String,
        ///(Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
        ///The offset. Useful if you want to page through the songs in a genre.
        #[serde(default)]
        pub offset: u64,
    }
    impl ::std::convert::From<&PostGetSongsByGenreBody> for PostGetSongsByGenreBody {
        fn from(value: &PostGetSongsByGenreBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetTopSongsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "The maximum number of songs to return.",
    ///      "default": 50,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "description": "The artist name.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetTopSongsBody {
        ///The maximum number of songs to return.
        #[serde(default = "defaults::default_u64::<u64, 50>")]
        pub count: u64,
        ///The artist name.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetTopSongsBody> for PostGetTopSongsBody {
        fn from(value: &PostGetTopSongsBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetUserBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "username": {
    ///      "description": "The name of the user to retrieve. You can only retrieve your own user unless you have admin privileges.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetUserBody {
        ///The name of the user to retrieve. You can only retrieve your own user unless you have admin privileges.
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetUserBody> for PostGetUserBody {
        fn from(value: &PostGetUserBody) -> Self {
            value.clone()
        }
    }
    ///`PostGetVideoInfoBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The video ID.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostGetVideoInfoBody {
        ///The video ID.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostGetVideoInfoBody> for PostGetVideoInfoBody {
        fn from(value: &PostGetVideoInfoBody) -> Self {
            value.clone()
        }
    }
    ///`PostHlsM3u8Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "audioTrack": {
    ///      "description": "The ID of the audio track to use. See `getVideoInfo` for how to get the list of available audio tracks for a video.",
    ///      "type": "string"
    ///    },
    ///    "bitRate": {
    ///      "description": "If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If set to zero, no limit is imposed.",
    ///      "type": "integer"
    ///    },
    ///    "id": {
    ///      "description": "A string which uniquely identifies the media file to stream.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostHlsM3u8Body {
        ///The ID of the audio track to use. See `getVideoInfo` for how to get the list of available audio tracks for a video.
        #[serde(
            rename = "audioTrack",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub audio_track: ::std::option::Option<::std::string::String>,
        ///If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If set to zero, no limit is imposed.
        #[serde(
            rename = "bitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bit_rate: ::std::option::Option<i64>,
        ///A string which uniquely identifies the media file to stream.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostHlsM3u8Body> for PostHlsM3u8Body {
        fn from(value: &PostHlsM3u8Body) -> Self {
            value.clone()
        }
    }
    ///`PostJukeboxControlBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "description": "The operation to perform. Must be one of: get, status (since 1.7.0), set (since 1.7.0), start, stop, skip, add, clear, remove, shuffle, setGain",
    ///          "enum": [
    ///            "get",
    ///            "status",
    ///            "start",
    ///            "stop",
    ///            "clear",
    ///            "shuffle"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "index"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "description": "'remove' action.",
    ///          "enum": [
    ///            "remove"
    ///          ]
    ///        },
    ///        "index": {
    ///          "description": "Zero-based index of the song to remove.",
    ///          "type": "integer",
    ///          "minimum": 0.0
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "index"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "description": "'skip' action.",
    ///          "enum": [
    ///            "skip"
    ///          ]
    ///        },
    ///        "index": {
    ///          "description": "Zero-based index of the song to skip.",
    ///          "type": "integer",
    ///          "minimum": 0.0
    ///        },
    ///        "offset": {
    ///          "description": "(Since 1.7.0) Used by `skip`. Start playing this many seconds into the track.",
    ///          "type": "integer",
    ///          "minimum": 0.0
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "id"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "description": "'add' action.",
    ///          "enum": [
    ///            "add"
    ///          ]
    ///        },
    ///        "id": {
    ///          "description": "ID of song to add to the jukebox playlist. Use multiple id parameters to add many songs in the same request. (set is similar to a clear followed by an add, but will not change the currently playing track.)",
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "id"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "description": "'set' action.",
    ///          "enum": [
    ///            "set"
    ///          ]
    ///        },
    ///        "id": {
    ///          "description": "ID of song to add to the jukebox playlist. Use multiple id parameters to add many songs in the same request. (set is similar to a clear followed by an add, but will not change the currently playing track.)",
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "action",
    ///        "gain"
    ///      ],
    ///      "properties": {
    ///        "action": {
    ///          "description": "'setGain' action.",
    ///          "enum": [
    ///            "setGain"
    ///          ]
    ///        },
    ///        "gain": {
    ///          "description": "Used by `setGain` to control the playback volume. A float value between 0.0 and 1.0.",
    ///          "type": "number",
    ///          "format": "float",
    ///          "maximum": 1.0,
    ///          "minimum": 0.0
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PostJukeboxControlBody {
        Variant0 {
            ///The operation to perform. Must be one of: get, status (since 1.7.0), set (since 1.7.0), start, stop, skip, add, clear, remove, shuffle, setGain
            action: PostJukeboxControlBodyVariant0Action,
        },
        Variant1 {
            ///'remove' action.
            action: PostJukeboxControlBodyVariant1Action,
            ///Zero-based index of the song to remove.
            index: u64,
        },
        Variant2 {
            ///'skip' action.
            action: PostJukeboxControlBodyVariant2Action,
            ///Zero-based index of the song to skip.
            index: u64,
            ///(Since 1.7.0) Used by `skip`. Start playing this many seconds into the track.
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            offset: ::std::option::Option<u64>,
        },
        Variant3 {
            ///'add' action.
            action: PostJukeboxControlBodyVariant3Action,
            ///ID of song to add to the jukebox playlist. Use multiple id parameters to add many songs in the same request. (set is similar to a clear followed by an add, but will not change the currently playing track.)
            id: ::std::vec::Vec<::std::string::String>,
        },
        Variant4 {
            ///'set' action.
            action: PostJukeboxControlBodyVariant4Action,
            ///ID of song to add to the jukebox playlist. Use multiple id parameters to add many songs in the same request. (set is similar to a clear followed by an add, but will not change the currently playing track.)
            id: ::std::vec::Vec<::std::string::String>,
        },
        Variant5 {
            ///'setGain' action.
            action: PostJukeboxControlBodyVariant5Action,
            gain: f32,
        },
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBody {
        fn from(value: &PostJukeboxControlBody) -> Self {
            value.clone()
        }
    }
    ///The operation to perform. Must be one of: get, status (since 1.7.0), set (since 1.7.0), start, stop, skip, add, clear, remove, shuffle, setGain
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The operation to perform. Must be one of: get, status (since 1.7.0), set (since 1.7.0), start, stop, skip, add, clear, remove, shuffle, setGain",
    ///  "enum": [
    ///    "get",
    ///    "status",
    ///    "start",
    ///    "stop",
    ///    "clear",
    ///    "shuffle"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostJukeboxControlBodyVariant0Action {
        #[serde(rename = "get")]
        Get,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "start")]
        Start,
        #[serde(rename = "stop")]
        Stop,
        #[serde(rename = "clear")]
        Clear,
        #[serde(rename = "shuffle")]
        Shuffle,
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBodyVariant0Action {
        fn from(value: &PostJukeboxControlBodyVariant0Action) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostJukeboxControlBodyVariant0Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Get => f.write_str("get"),
                Self::Status => f.write_str("status"),
                Self::Start => f.write_str("start"),
                Self::Stop => f.write_str("stop"),
                Self::Clear => f.write_str("clear"),
                Self::Shuffle => f.write_str("shuffle"),
            }
        }
    }
    impl ::std::str::FromStr for PostJukeboxControlBodyVariant0Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "get" => Ok(Self::Get),
                "status" => Ok(Self::Status),
                "start" => Ok(Self::Start),
                "stop" => Ok(Self::Stop),
                "clear" => Ok(Self::Clear),
                "shuffle" => Ok(Self::Shuffle),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostJukeboxControlBodyVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostJukeboxControlBodyVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostJukeboxControlBodyVariant0Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///'remove' action.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "'remove' action.",
    ///  "enum": [
    ///    "remove"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostJukeboxControlBodyVariant1Action {
        #[serde(rename = "remove")]
        Remove,
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBodyVariant1Action {
        fn from(value: &PostJukeboxControlBodyVariant1Action) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostJukeboxControlBodyVariant1Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Remove => f.write_str("remove"),
            }
        }
    }
    impl ::std::str::FromStr for PostJukeboxControlBodyVariant1Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "remove" => Ok(Self::Remove),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostJukeboxControlBodyVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostJukeboxControlBodyVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostJukeboxControlBodyVariant1Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///'skip' action.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "'skip' action.",
    ///  "enum": [
    ///    "skip"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostJukeboxControlBodyVariant2Action {
        #[serde(rename = "skip")]
        Skip,
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBodyVariant2Action {
        fn from(value: &PostJukeboxControlBodyVariant2Action) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostJukeboxControlBodyVariant2Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Skip => f.write_str("skip"),
            }
        }
    }
    impl ::std::str::FromStr for PostJukeboxControlBodyVariant2Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "skip" => Ok(Self::Skip),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostJukeboxControlBodyVariant2Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostJukeboxControlBodyVariant2Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostJukeboxControlBodyVariant2Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///'add' action.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "'add' action.",
    ///  "enum": [
    ///    "add"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostJukeboxControlBodyVariant3Action {
        #[serde(rename = "add")]
        Add,
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBodyVariant3Action {
        fn from(value: &PostJukeboxControlBodyVariant3Action) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostJukeboxControlBodyVariant3Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Add => f.write_str("add"),
            }
        }
    }
    impl ::std::str::FromStr for PostJukeboxControlBodyVariant3Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "add" => Ok(Self::Add),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostJukeboxControlBodyVariant3Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostJukeboxControlBodyVariant3Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostJukeboxControlBodyVariant3Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///'set' action.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "'set' action.",
    ///  "enum": [
    ///    "set"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostJukeboxControlBodyVariant4Action {
        #[serde(rename = "set")]
        Set,
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBodyVariant4Action {
        fn from(value: &PostJukeboxControlBodyVariant4Action) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostJukeboxControlBodyVariant4Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Set => f.write_str("set"),
            }
        }
    }
    impl ::std::str::FromStr for PostJukeboxControlBodyVariant4Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "set" => Ok(Self::Set),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostJukeboxControlBodyVariant4Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostJukeboxControlBodyVariant4Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostJukeboxControlBodyVariant4Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///'setGain' action.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "'setGain' action.",
    ///  "enum": [
    ///    "setGain"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostJukeboxControlBodyVariant5Action {
        #[serde(rename = "setGain")]
        SetGain,
    }
    impl ::std::convert::From<&Self> for PostJukeboxControlBodyVariant5Action {
        fn from(value: &PostJukeboxControlBodyVariant5Action) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostJukeboxControlBodyVariant5Action {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SetGain => f.write_str("setGain"),
            }
        }
    }
    impl ::std::str::FromStr for PostJukeboxControlBodyVariant5Action {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "setGain" => Ok(Self::SetGain),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostJukeboxControlBodyVariant5Action {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostJukeboxControlBodyVariant5Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostJukeboxControlBodyVariant5Action {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PostSavePlayQueueBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "current": {
    ///      "description": "The ID of the current playing song. This is required if one or mode IDs is provided",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "ID of a song in the play queue. Use one id parameter for each song in the play queue. Specify no IDs to clear",
    ///      "type": "string"
    ///    },
    ///    "position": {
    ///      "description": "The position in milliseconds within the currently playing song.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSavePlayQueueBody {
        ///The ID of the current playing song. This is required if one or mode IDs is provided
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub current: ::std::option::Option<::std::string::String>,
        ///ID of a song in the play queue. Use one id parameter for each song in the play queue. Specify no IDs to clear
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///The position in milliseconds within the currently playing song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<u64>,
    }
    impl ::std::convert::From<&PostSavePlayQueueBody> for PostSavePlayQueueBody {
        fn from(value: &PostSavePlayQueueBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostSavePlayQueueBody {
        fn default() -> Self {
            Self {
                current: Default::default(),
                id: Default::default(),
                position: Default::default(),
            }
        }
    }
    ///`PostSavePlayQueueByIndexBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "currentIndex": {
    ///      "description": "The index of the current playing song. This is required if one or more IDs is provided.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "ID of a song in the play queue. Use one id parameter for each song in the play queue. Specify no IDs to clear",
    ///      "type": "string"
    ///    },
    ///    "position": {
    ///      "description": "The position in milliseconds within the currently playing song.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSavePlayQueueByIndexBody {
        ///The index of the current playing song. This is required if one or more IDs is provided.
        #[serde(
            rename = "currentIndex",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_index: ::std::option::Option<::std::string::String>,
        ///ID of a song in the play queue. Use one id parameter for each song in the play queue. Specify no IDs to clear
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///The position in milliseconds within the currently playing song.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<u64>,
    }
    impl ::std::convert::From<&PostSavePlayQueueByIndexBody> for PostSavePlayQueueByIndexBody {
        fn from(value: &PostSavePlayQueueByIndexBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostSavePlayQueueByIndexBody {
        fn default() -> Self {
            Self {
                current_index: Default::default(),
                id: Default::default(),
                position: Default::default(),
            }
        }
    }
    ///`PostScrobbleBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "A string which uniquely identifies the file to scrobble.",
    ///      "type": "string"
    ///    },
    ///    "submission": {
    ///      "description": "Whether this is a “submission” or a “now playing” notification.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "time": {
    ///      "description": "(Since 1.8.0) The time (in milliseconds since 1 Jan 1970) at which the song was listened to.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostScrobbleBody {
        ///A string which uniquely identifies the file to scrobble.
        pub id: ::std::string::String,
        ///Whether this is a “submission” or a “now playing” notification.
        #[serde(default = "defaults::default_bool::<true>")]
        pub submission: bool,
        ///(Since 1.8.0) The time (in milliseconds since 1 Jan 1970) at which the song was listened to.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<u64>,
    }
    impl ::std::convert::From<&PostScrobbleBody> for PostScrobbleBody {
        fn from(value: &PostScrobbleBody) -> Self {
            value.clone()
        }
    }
    ///`PostSearch2Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "query"
    ///  ],
    ///  "properties": {
    ///    "albumCount": {
    ///      "description": "Maximum number of albums to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "albumOffset": {
    ///      "description": "Search result offset for albums. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "artistCount": {
    ///      "description": "Maximum number of artists to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "artistOffset": {
    ///      "description": "Search result offset for artists. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    },
    ///    "query": {
    ///      "description": "Search query.",
    ///      "type": "string"
    ///    },
    ///    "songCount": {
    ///      "description": "Maximum number of songs to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "songOffset": {
    ///      "description": "Search result offset for songs. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSearch2Body {
        ///Maximum number of albums to return.
        #[serde(rename = "albumCount", default = "defaults::default_u64::<u64, 20>")]
        pub album_count: u64,
        ///Search result offset for albums. Used for paging.
        #[serde(rename = "albumOffset", default)]
        pub album_offset: u64,
        ///Maximum number of artists to return.
        #[serde(rename = "artistCount", default = "defaults::default_u64::<u64, 20>")]
        pub artist_count: u64,
        ///Search result offset for artists. Used for paging.
        #[serde(rename = "artistOffset", default)]
        pub artist_offset: u64,
        ///(Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
        ///Search query.
        pub query: ::std::string::String,
        ///Maximum number of songs to return.
        #[serde(rename = "songCount", default = "defaults::default_u64::<u64, 20>")]
        pub song_count: u64,
        ///Search result offset for songs. Used for paging.
        #[serde(rename = "songOffset", default)]
        pub song_offset: u64,
    }
    impl ::std::convert::From<&PostSearch2Body> for PostSearch2Body {
        fn from(value: &PostSearch2Body) -> Self {
            value.clone()
        }
    }
    ///`PostSearch3Body`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "query"
    ///  ],
    ///  "properties": {
    ///    "albumCount": {
    ///      "description": "Maximum number of albums to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "albumOffset": {
    ///      "description": "Search result offset for albums. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "artistCount": {
    ///      "description": "Maximum number of artists to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "artistOffset": {
    ///      "description": "Search result offset for artists. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.",
    ///      "type": "string"
    ///    },
    ///    "query": {
    ///      "description": "Search query. Servers must support an empty query and return all the data to allow clients to properly access all the media information for offline sync.",
    ///      "type": "string"
    ///    },
    ///    "songCount": {
    ///      "description": "Maximum number of songs to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "songOffset": {
    ///      "description": "Search result offset for songs. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSearch3Body {
        ///Maximum number of albums to return.
        #[serde(rename = "albumCount", default = "defaults::default_u64::<u64, 20>")]
        pub album_count: u64,
        ///Search result offset for albums. Used for paging.
        #[serde(rename = "albumOffset", default)]
        pub album_offset: u64,
        ///Maximum number of artists to return.
        #[serde(rename = "artistCount", default = "defaults::default_u64::<u64, 20>")]
        pub artist_count: u64,
        ///Search result offset for artists. Used for paging.
        #[serde(rename = "artistOffset", default)]
        pub artist_offset: u64,
        ///(Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub music_folder_id: ::std::option::Option<::std::string::String>,
        ///Search query. Servers must support an empty query and return all the data to allow clients to properly access all the media information for offline sync.
        pub query: ::std::string::String,
        ///Maximum number of songs to return.
        #[serde(rename = "songCount", default = "defaults::default_u64::<u64, 20>")]
        pub song_count: u64,
        ///Search result offset for songs. Used for paging.
        #[serde(rename = "songOffset", default)]
        pub song_offset: u64,
    }
    impl ::std::convert::From<&PostSearch3Body> for PostSearch3Body {
        fn from(value: &PostSearch3Body) -> Self {
            value.clone()
        }
    }
    ///`PostSearchBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Album to search for.",
    ///      "type": "string"
    ///    },
    ///    "any": {
    ///      "description": "Searches all fields.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "artist": {
    ///      "description": "Artist to search for.",
    ///      "type": "string"
    ///    },
    ///    "count": {
    ///      "description": "Maximum number of results to return.",
    ///      "default": 20,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "newerThan": {
    ///      "description": "Only return matches that are newer than this. Given as milliseconds since 1970.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "offset": {
    ///      "description": "Search result offset. Used for paging.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "title": {
    ///      "description": "Song title to search for.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSearchBody {
        ///Album to search for.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub album: ::std::option::Option<::std::string::String>,
        ///Searches all fields.
        #[serde(default)]
        pub any: bool,
        ///Artist to search for.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artist: ::std::option::Option<::std::string::String>,
        ///Maximum number of results to return.
        #[serde(default = "defaults::default_u64::<u64, 20>")]
        pub count: u64,
        ///Only return matches that are newer than this. Given as milliseconds since 1970.
        #[serde(
            rename = "newerThan",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub newer_than: ::std::option::Option<u64>,
        ///Search result offset. Used for paging.
        #[serde(default)]
        pub offset: u64,
        ///Song title to search for.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PostSearchBody> for PostSearchBody {
        fn from(value: &PostSearchBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostSearchBody {
        fn default() -> Self {
            Self {
                album: Default::default(),
                any: Default::default(),
                artist: Default::default(),
                count: defaults::default_u64::<u64, 20>(),
                newer_than: Default::default(),
                offset: Default::default(),
                title: Default::default(),
            }
        }
    }
    ///`PostSetRatingBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "rating"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "A string which uniquely identifies the file (song) or folder (album/artist) to rate.",
    ///      "type": "string"
    ///    },
    ///    "rating": {
    ///      "description": "The rating between 1 and 5 (inclusive), or 0 to remove the rating.",
    ///      "type": "integer",
    ///      "maximum": 5.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSetRatingBody {
        ///A string which uniquely identifies the file (song) or folder (album/artist) to rate.
        pub id: ::std::string::String,
        ///The rating between 1 and 5 (inclusive), or 0 to remove the rating.
        pub rating: i64,
    }
    impl ::std::convert::From<&PostSetRatingBody> for PostSetRatingBody {
        fn from(value: &PostSetRatingBody) -> Self {
            value.clone()
        }
    }
    ///`PostStarBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "albumId": {
    ///      "description": "The ID of an album to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "artistId": {
    ///      "description": "The ID of an artist to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "The ID of the file (song) or folder (album/artist) to star. Multiple parameters allowed.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostStarBody {
        ///The ID of an album to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
        #[serde(
            rename = "albumId",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub album_id: ::std::vec::Vec<::std::string::String>,
        ///The ID of an artist to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub artist_id: ::std::vec::Vec<::std::string::String>,
        ///The ID of the file (song) or folder (album/artist) to star. Multiple parameters allowed.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub id: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::convert::From<&PostStarBody> for PostStarBody {
        fn from(value: &PostStarBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostStarBody {
        fn default() -> Self {
            Self {
                album_id: Default::default(),
                artist_id: Default::default(),
                id: Default::default(),
            }
        }
    }
    ///`PostStreamBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "converted": {
    ///      "description": "(Since 1.14.0) Only applicable to video streaming. Servers can optimize videos for streaming by converting them to MP4. If a conversion exists for the video in question, then setting this parameter to “true” will cause the converted video to be returned instead of the original.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "estimateContentLength": {
    ///      "description": "(Since 1.8.0). If set to “true”, the Content-Length HTTP header will be set to an estimated value for transcoded or downsampled media.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "format": {
    ///      "description": "(Since 1.6.0) Specifies the preferred target format (e.g., “mp3” or “flv”) in case there are multiple applicable transcodings. Starting with 1.9.0 you can use the special value “raw” to disable transcoding.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "A string which uniquely identifies the file to stream. Obtained by calls to getMusicDirectory.",
    ///      "type": "string"
    ///    },
    ///    "maxBitRate": {
    ///      "description": "(Since 1.2.0) If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If set to zero, no limit is imposed.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "size": {
    ///      "description": "(Since 1.6.0) Only applicable to video streaming. Requested video size specified as WxH, for instance “640x480”.",
    ///      "type": "string",
    ///      "pattern": "^[0-9]+x[0-9]+$"
    ///    },
    ///    "timeOffset": {
    ///      "description": "By default only applicable to video streaming. If specified, start streaming at the given offset (in seconds) into the media. The `Transcode Offset` extension enables the parameter to music too.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostStreamBody {
        ///(Since 1.14.0) Only applicable to video streaming. Servers can optimize videos for streaming by converting them to MP4. If a conversion exists for the video in question, then setting this parameter to “true” will cause the converted video to be returned instead of the original.
        #[serde(default)]
        pub converted: bool,
        ///(Since 1.8.0). If set to “true”, the Content-Length HTTP header will be set to an estimated value for transcoded or downsampled media.
        #[serde(rename = "estimateContentLength", default)]
        pub estimate_content_length: bool,
        ///(Since 1.6.0) Specifies the preferred target format (e.g., “mp3” or “flv”) in case there are multiple applicable transcodings. Starting with 1.9.0 you can use the special value “raw” to disable transcoding.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub format: ::std::option::Option<::std::string::String>,
        ///A string which uniquely identifies the file to stream. Obtained by calls to getMusicDirectory.
        pub id: ::std::string::String,
        ///(Since 1.2.0) If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If set to zero, no limit is imposed.
        #[serde(
            rename = "maxBitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_bit_rate: ::std::option::Option<u64>,
        ///(Since 1.6.0) Only applicable to video streaming. Requested video size specified as WxH, for instance “640x480”.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<PostStreamBodySize>,
        ///By default only applicable to video streaming. If specified, start streaming at the given offset (in seconds) into the media. The `Transcode Offset` extension enables the parameter to music too.
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<u64>,
    }
    impl ::std::convert::From<&PostStreamBody> for PostStreamBody {
        fn from(value: &PostStreamBody) -> Self {
            value.clone()
        }
    }
    ///(Since 1.6.0) Only applicable to video streaming. Requested video size specified as WxH, for instance “640x480”.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "(Since 1.6.0) Only applicable to video streaming. Requested video size specified as WxH, for instance “640x480”.",
    ///  "type": "string",
    ///  "pattern": "^[0-9]+x[0-9]+$"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostStreamBodySize(::std::string::String);
    impl ::std::ops::Deref for PostStreamBodySize {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PostStreamBodySize> for ::std::string::String {
        fn from(value: PostStreamBodySize) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PostStreamBodySize> for PostStreamBodySize {
        fn from(value: &PostStreamBodySize) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for PostStreamBodySize {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9]+x[0-9]+$").unwrap());
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[0-9]+x[0-9]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PostStreamBodySize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PostStreamBodySize {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PostStreamBodySize {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PostStreamBodySize {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`PostUnstarBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "albumId": {
    ///      "description": "The ID of an album to unstar. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed."
    ///    },
    ///    "artistId": {
    ///      "description": "The ID of an artist to unstar. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed."
    ///    },
    ///    "id": {
    ///      "description": "The ID of the file (song) or folder (album/artist) to unstar. Multiple parameters allowed."
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostUnstarBody {
        ///The ID of an album to unstar. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
        #[serde(
            rename = "albumId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_id: ::std::option::Option<::serde_json::Value>,
        ///The ID of an artist to unstar. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
        #[serde(
            rename = "artistId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub artist_id: ::std::option::Option<::serde_json::Value>,
        ///The ID of the file (song) or folder (album/artist) to unstar. Multiple parameters allowed.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::serde_json::Value>,
    }
    impl ::std::convert::From<&PostUnstarBody> for PostUnstarBody {
        fn from(value: &PostUnstarBody) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostUnstarBody {
        fn default() -> Self {
            Self {
                album_id: Default::default(),
                artist_id: Default::default(),
                id: Default::default(),
            }
        }
    }
    ///`PostUpdateInternetRadioStationBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "streamUrl"
    ///  ],
    ///  "properties": {
    ///    "homepageUrl": {
    ///      "description": "The home page URL for the station.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "The ID of the station.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The user-defined name for the station.",
    ///      "type": "string"
    ///    },
    ///    "streamUrl": {
    ///      "description": "The stream URL for the station.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostUpdateInternetRadioStationBody {
        ///The home page URL for the station.
        #[serde(
            rename = "homepageUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub homepage_url: ::std::option::Option<::std::string::String>,
        ///The ID of the station.
        pub id: ::std::string::String,
        ///The user-defined name for the station.
        pub name: ::std::string::String,
        ///The stream URL for the station.
        #[serde(rename = "streamUrl")]
        pub stream_url: ::std::string::String,
    }
    impl ::std::convert::From<&PostUpdateInternetRadioStationBody>
        for PostUpdateInternetRadioStationBody
    {
        fn from(value: &PostUpdateInternetRadioStationBody) -> Self {
            value.clone()
        }
    }
    ///`PostUpdatePlaylistBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "playlistId"
    ///  ],
    ///  "properties": {
    ///    "comment": {
    ///      "description": "The playlist comment.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The human-readable name of the playlist.",
    ///      "type": "string"
    ///    },
    ///    "playlistId": {
    ///      "description": "The playlist ID.",
    ///      "type": "string"
    ///    },
    ///    "public": {
    ///      "description": "`true` if the playlist should be visible to all users, `false` otherwise.",
    ///      "type": "boolean"
    ///    },
    ///    "songIdToAdd": {
    ///      "description": "Add this song with this ID to the playlist. Multiple parameters allowed.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "songIndexToRemove": {
    ///      "description": "Remove the song at this position in the playlist. Multiple parameters allowed.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostUpdatePlaylistBody {
        ///The playlist comment.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        ///The human-readable name of the playlist.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The playlist ID.
        #[serde(rename = "playlistId")]
        pub playlist_id: ::std::string::String,
        ///`true` if the playlist should be visible to all users, `false` otherwise.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public: ::std::option::Option<bool>,
        ///Add this song with this ID to the playlist. Multiple parameters allowed.
        #[serde(
            rename = "songIdToAdd",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub song_id_to_add: ::std::vec::Vec<::std::string::String>,
        ///Remove the song at this position in the playlist. Multiple parameters allowed.
        #[serde(
            rename = "songIndexToRemove",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub song_index_to_remove: ::std::vec::Vec<i64>,
    }
    impl ::std::convert::From<&PostUpdatePlaylistBody> for PostUpdatePlaylistBody {
        fn from(value: &PostUpdatePlaylistBody) -> Self {
            value.clone()
        }
    }
    ///`PostUpdateShareBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "A user-defined description that will be displayed to people visiting the shared media.",
    ///      "type": "string"
    ///    },
    ///    "expires": {
    ///      "description": "The time at which the share expires. Given as milliseconds since 1970, or zero to remove the expiration.",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "description": "ID of the share to update.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostUpdateShareBody {
        ///A user-defined description that will be displayed to people visiting the shared media.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///The time at which the share expires. Given as milliseconds since 1970, or zero to remove the expiration.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires: ::std::option::Option<u64>,
        ///ID of the share to update.
        pub id: ::std::string::String,
    }
    impl ::std::convert::From<&PostUpdateShareBody> for PostUpdateShareBody {
        fn from(value: &PostUpdateShareBody) -> Self {
            value.clone()
        }
    }
    ///`PostUpdateUserBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "password",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "adminRole": {
    ///      "description": "Whether the user is an administrator.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "commentRole": {
    ///      "description": "Whether the user is allowed to create and edit comments and ratings.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "coverArtRole": {
    ///      "description": "Whether the user is allowed to change cover art and tags.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "downloadRole": {
    ///      "description": "Whether the user is allowed to download files.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "email": {
    ///      "description": "The email address of the user.",
    ///      "type": "string"
    ///    },
    ///    "jukeboxRole": {
    ///      "description": "Whether the user is allowed to play files in jukebox mode.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "ldapAuthenticated": {
    ///      "description": "Whether the user is authenticated in LDAP.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "maxBitRate": {
    ///      "description": "(Since 1.13.0) The maximum bit rate (in Kbps) for the user. Audio streams of higher bit rates are automatically downsampled to this bit rate. Legal values: 0 (no limit), 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320.",
    ///      "type": "integer",
    ///      "enum": [
    ///        0,
    ///        32,
    ///        40,
    ///        48,
    ///        56,
    ///        64,
    ///        80,
    ///        96,
    ///        112,
    ///        128,
    ///        160,
    ///        192,
    ///        224,
    ///        256,
    ///        320
    ///      ]
    ///    },
    ///    "musicFolderId": {
    ///      "description": "(Since 1.12.0) IDs of the music folders the user is allowed access to. Include the parameter once for each folder.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "password": {
    ///      "description": "The password of the user, either in clear text or hex-encoded.",
    ///      "type": "string"
    ///    },
    ///    "podcastRole": {
    ///      "description": "Whether the user is allowed to administrate Podcasts.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "settingsRole": {
    ///      "description": "Whether the user is allowed to change personal settings and password.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "shareRole": {
    ///      "description": "Whether the user is allowed to share files with anyone.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "streamRole": {
    ///      "description": "Whether the user is allowed to play files.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "uploadRole": {
    ///      "description": "Whether the user is allowed to upload files.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "username": {
    ///      "description": "The name of the user.",
    ///      "type": "string"
    ///    },
    ///    "videoConversionRole": {
    ///      "description": "(Since 1.15.0) Whether the user is allowed to start video conversions.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostUpdateUserBody {
        ///Whether the user is an administrator.
        #[serde(rename = "adminRole", default)]
        pub admin_role: bool,
        ///Whether the user is allowed to create and edit comments and ratings.
        #[serde(rename = "commentRole", default)]
        pub comment_role: bool,
        ///Whether the user is allowed to change cover art and tags.
        #[serde(rename = "coverArtRole", default)]
        pub cover_art_role: bool,
        ///Whether the user is allowed to download files.
        #[serde(rename = "downloadRole", default)]
        pub download_role: bool,
        ///The email address of the user.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        ///Whether the user is allowed to play files in jukebox mode.
        #[serde(rename = "jukeboxRole", default)]
        pub jukebox_role: bool,
        ///Whether the user is authenticated in LDAP.
        #[serde(rename = "ldapAuthenticated", default)]
        pub ldap_authenticated: bool,
        ///(Since 1.13.0) The maximum bit rate (in Kbps) for the user. Audio streams of higher bit rates are automatically downsampled to this bit rate. Legal values: 0 (no limit), 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320.
        #[serde(
            rename = "maxBitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_bit_rate: ::std::option::Option<PostUpdateUserBodyMaxBitRate>,
        ///(Since 1.12.0) IDs of the music folders the user is allowed access to. Include the parameter once for each folder.
        #[serde(
            rename = "musicFolderId",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub music_folder_id: ::std::vec::Vec<::std::string::String>,
        ///The password of the user, either in clear text or hex-encoded.
        pub password: ::std::string::String,
        ///Whether the user is allowed to administrate Podcasts.
        #[serde(rename = "podcastRole", default)]
        pub podcast_role: bool,
        ///Whether the user is allowed to change personal settings and password.
        #[serde(rename = "settingsRole", default = "defaults::default_bool::<true>")]
        pub settings_role: bool,
        ///Whether the user is allowed to share files with anyone.
        #[serde(rename = "shareRole", default)]
        pub share_role: bool,
        ///Whether the user is allowed to play files.
        #[serde(rename = "streamRole", default = "defaults::default_bool::<true>")]
        pub stream_role: bool,
        ///Whether the user is allowed to upload files.
        #[serde(rename = "uploadRole", default)]
        pub upload_role: bool,
        ///The name of the user.
        pub username: ::std::string::String,
        ///(Since 1.15.0) Whether the user is allowed to start video conversions.
        #[serde(rename = "videoConversionRole", default)]
        pub video_conversion_role: bool,
    }
    impl ::std::convert::From<&PostUpdateUserBody> for PostUpdateUserBody {
        fn from(value: &PostUpdateUserBody) -> Self {
            value.clone()
        }
    }
    ///(Since 1.13.0) The maximum bit rate (in Kbps) for the user. Audio streams of higher bit rates are automatically downsampled to this bit rate. Legal values: 0 (no limit), 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "(Since 1.13.0) The maximum bit rate (in Kbps) for the user. Audio streams of higher bit rates are automatically downsampled to this bit rate. Legal values: 0 (no limit), 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320.",
    ///  "type": "integer",
    ///  "enum": [
    ///    0,
    ///    32,
    ///    40,
    ///    48,
    ///    56,
    ///    64,
    ///    80,
    ///    96,
    ///    112,
    ///    128,
    ///    160,
    ///    192,
    ///    224,
    ///    256,
    ///    320
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct PostUpdateUserBodyMaxBitRate(i64);
    impl ::std::ops::Deref for PostUpdateUserBodyMaxBitRate {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<PostUpdateUserBodyMaxBitRate> for i64 {
        fn from(value: PostUpdateUserBodyMaxBitRate) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PostUpdateUserBodyMaxBitRate> for PostUpdateUserBodyMaxBitRate {
        fn from(value: &PostUpdateUserBodyMaxBitRate) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for PostUpdateUserBodyMaxBitRate {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![
                0_i64, 32_i64, 40_i64, 48_i64, 56_i64, 64_i64, 80_i64, 96_i64, 112_i64, 128_i64,
                160_i64, 192_i64, 224_i64, 256_i64, 320_i64,
            ]
            .contains(&value)
            {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PostUpdateUserBodyMaxBitRate {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///A record label for an album.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A record label for an album.",
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RecordLabel {
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&RecordLabel> for RecordLabel {
        fn from(value: &RecordLabel) -> Self {
            value.clone()
        }
    }
    ///The replay gain data of a song. Note: If the data is not present the field must be ommited in the answer. (But the replayGain field on Child must always be present)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The replay gain data of a song. Note: If the data is not present the field must be ommited in the answer. (But the replayGain field on Child must always be present)",
    ///  "type": "object",
    ///  "properties": {
    ///    "albumGain": {
    ///      "description": "The album replay gain value. (In Db)",
    ///      "type": "number"
    ///    },
    ///    "albumPeak": {
    ///      "description": "The album peak value. (Must be positive)",
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "baseGain": {
    ///      "description": "The base gain value. (In Db) (Ogg Opus Output Gain for example)",
    ///      "type": "number"
    ///    },
    ///    "fallbackGain": {
    ///      "description": "An optional fallback gain that clients should apply when the corresponding gain value is missing. (Can be computed from the tracks or exposed as an user setting.)",
    ///      "type": "number"
    ///    },
    ///    "trackGain": {
    ///      "description": "The track replay gain value. (In Db)",
    ///      "type": "number"
    ///    },
    ///    "trackPeak": {
    ///      "description": "The track peak value. (Must be positive)",
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ReplayGain {
        #[serde(
            rename = "albumGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_gain: ::std::option::Option<f64>,
        #[serde(
            rename = "albumPeak",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub album_peak: ::std::option::Option<f64>,
        #[serde(
            rename = "baseGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub base_gain: ::std::option::Option<f64>,
        #[serde(
            rename = "fallbackGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fallback_gain: ::std::option::Option<f64>,
        #[serde(
            rename = "trackGain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub track_gain: ::std::option::Option<f64>,
        #[serde(
            rename = "trackPeak",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub track_peak: ::std::option::Option<f64>,
    }
    impl ::std::convert::From<&ReplayGain> for ReplayGain {
        fn from(value: &ReplayGain) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ReplayGain {
        fn default() -> Self {
            Self {
                album_gain: Default::default(),
                album_peak: Default::default(),
                base_gain: Default::default(),
                fallback_gain: Default::default(),
                track_gain: Default::default(),
                track_peak: Default::default(),
            }
        }
    }
    ///Scan status information.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Scan status information.",
    ///  "type": "object",
    ///  "required": [
    ///    "scanning"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "Scanned item count",
    ///      "type": "integer"
    ///    },
    ///    "scanning": {
    ///      "description": "The status of the scan",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ScanStatus {
        ///Scanned item count
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub count: ::std::option::Option<i64>,
        ///The status of the scan
        pub scanning: bool,
    }
    impl ::std::convert::From<&ScanStatus> for ScanStatus {
        fn from(value: &ScanStatus) -> Self {
            value.clone()
        }
    }
    ///A subsonic-response element with a nested `searchResult2` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `searchResult2` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Search2SuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Search2Response {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<Search2ResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&Search2Response> for Search2Response {
        fn from(value: &Search2Response) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Search2Response {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`Search2ResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Search2SuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum Search2ResponseSubsonicResponse {
        Search2SuccessResponse(Search2SuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for Search2ResponseSubsonicResponse {
        fn from(value: &Search2ResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<Search2SuccessResponse> for Search2ResponseSubsonicResponse {
        fn from(value: Search2SuccessResponse) -> Self {
            Self::Search2SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for Search2ResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`Search2SuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "searchResult2"
    ///      ],
    ///      "properties": {
    ///        "searchResult2": {
    ///          "$ref": "#/components/schemas/SearchResult2"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Search2SuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "searchResult2")]
        pub search_result2: SearchResult2,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: Search2SuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&Search2SuccessResponse> for Search2SuccessResponse {
        fn from(value: &Search2SuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum Search2SuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for Search2SuccessResponseStatus {
        fn from(value: &Search2SuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for Search2SuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for Search2SuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for Search2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for Search2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for Search2SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `searchResult3` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `searchResult3` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Search3SuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Search3Response {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<Search3ResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&Search3Response> for Search3Response {
        fn from(value: &Search3Response) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Search3Response {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`Search3ResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Search3SuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum Search3ResponseSubsonicResponse {
        Search3SuccessResponse(Search3SuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for Search3ResponseSubsonicResponse {
        fn from(value: &Search3ResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<Search3SuccessResponse> for Search3ResponseSubsonicResponse {
        fn from(value: Search3SuccessResponse) -> Self {
            Self::Search3SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for Search3ResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`Search3SuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "searchResult3"
    ///      ],
    ///      "properties": {
    ///        "searchResult3": {
    ///          "$ref": "#/components/schemas/SearchResult3"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Search3SuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "searchResult3")]
        pub search_result3: SearchResult3,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: Search3SuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&Search3SuccessResponse> for Search3SuccessResponse {
        fn from(value: &Search3SuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum Search3SuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for Search3SuccessResponseStatus {
        fn from(value: &Search3SuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for Search3SuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for Search3SuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for Search3SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for Search3SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for Search3SuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A subsonic-response element with a nested `searchResult` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `searchResult` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SearchSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<SearchResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&SearchResponse> for SearchResponse {
        fn from(value: &SearchResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for SearchResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`SearchResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SearchSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SearchResponseSubsonicResponse {
        SearchSuccessResponse(SearchSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for SearchResponseSubsonicResponse {
        fn from(value: &SearchResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<SearchSuccessResponse> for SearchResponseSubsonicResponse {
        fn from(value: SearchSuccessResponse) -> Self {
            Self::SearchSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for SearchResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///searchResult. TODO
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "searchResult. TODO",
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct SearchResult(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for SearchResult {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<SearchResult>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: SearchResult) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&SearchResult> for SearchResult {
        fn from(value: &SearchResult) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for SearchResult
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///searchResult2
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "searchResult2",
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Starred albums",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "artist": {
    ///      "description": "Starred artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Artist"
    ///      }
    ///    },
    ///    "song": {
    ///      "description": "Starred songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchResult2 {
        ///Starred albums
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub album: ::std::vec::Vec<Child>,
        ///Starred artists
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artist: ::std::vec::Vec<Artist>,
        ///Starred songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&SearchResult2> for SearchResult2 {
        fn from(value: &SearchResult2) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for SearchResult2 {
        fn default() -> Self {
            Self {
                album: Default::default(),
                artist: Default::default(),
                song: Default::default(),
            }
        }
    }
    ///searchResult3
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "searchResult3",
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Matching albums",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AlbumID3"
    ///      }
    ///    },
    ///    "artist": {
    ///      "description": "Matching artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    },
    ///    "song": {
    ///      "description": "Matching songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchResult3 {
        ///Matching albums
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub album: ::std::vec::Vec<AlbumId3>,
        ///Matching artists
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artist: ::std::vec::Vec<ArtistId3>,
        ///Matching songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&SearchResult3> for SearchResult3 {
        fn from(value: &SearchResult3) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for SearchResult3 {
        fn default() -> Self {
            Self {
                album: Default::default(),
                artist: Default::default(),
                song: Default::default(),
            }
        }
    }
    ///`SearchSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "searchResult"
    ///      ],
    ///      "properties": {
    ///        "searchResult": {
    ///          "$ref": "#/components/schemas/SearchResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "searchResult")]
        pub search_result: SearchResult,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: SearchSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&SearchSuccessResponse> for SearchSuccessResponse {
        fn from(value: &SearchSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SearchSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for SearchSuccessResponseStatus {
        fn from(value: &SearchSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for SearchSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for SearchSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SearchSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SearchSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SearchSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Share.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Share.",
    ///  "type": "object",
    ///  "required": [
    ///    "created",
    ///    "id",
    ///    "url",
    ///    "username",
    ///    "visitCount"
    ///  ],
    ///  "properties": {
    ///    "created": {
    ///      "description": "Creation date [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "description": {
    ///      "description": "A description",
    ///      "type": "string"
    ///    },
    ///    "entry": {
    ///      "description": "A list of share",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "expires": {
    ///      "description": "Share expiration [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "description": "The share Id",
    ///      "type": "string"
    ///    },
    ///    "lastVisited": {
    ///      "description": "Last visit [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "url": {
    ///      "description": "The share url",
    ///      "type": "string"
    ///    },
    ///    "username": {
    ///      "description": "The username",
    ///      "type": "string"
    ///    },
    ///    "visitCount": {
    ///      "description": "Visit count",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Share {
        ///Creation date [ISO 8601]
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        ///A description
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///A list of share
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub entry: ::std::vec::Vec<Child>,
        ///Share expiration [ISO 8601]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The share Id
        pub id: ::std::string::String,
        ///Last visit [ISO 8601]
        #[serde(
            rename = "lastVisited",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_visited: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///The share url
        pub url: ::std::string::String,
        ///The username
        pub username: ::std::string::String,
        ///Visit count
        #[serde(rename = "visitCount")]
        pub visit_count: i64,
    }
    impl ::std::convert::From<&Share> for Share {
        fn from(value: &Share) -> Self {
            value.clone()
        }
    }
    ///Shares.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Shares.",
    ///  "type": "object",
    ///  "properties": {
    ///    "share": {
    ///      "description": "A list of share",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Share"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Shares {
        ///A list of share
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub share: ::std::vec::Vec<Share>,
    }
    impl ::std::convert::From<&Shares> for Shares {
        fn from(value: &Shares) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Shares {
        fn default() -> Self {
            Self {
                share: Default::default(),
            }
        }
    }
    ///SimilarSongs list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "SimilarSongs list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "song": {
    ///      "description": "List of songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SimilarSongs {
        ///List of songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&SimilarSongs> for SimilarSongs {
        fn from(value: &SimilarSongs) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for SimilarSongs {
        fn default() -> Self {
            Self {
                song: Default::default(),
            }
        }
    }
    ///SimilarSongs2 list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "SimilarSongs2 list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "song": {
    ///      "description": "List of songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SimilarSongs2 {
        ///List of songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&SimilarSongs2> for SimilarSongs2 {
        fn from(value: &SimilarSongs2) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for SimilarSongs2 {
        fn default() -> Self {
            Self {
                song: Default::default(),
            }
        }
    }
    ///Songs list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Songs list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "song": {
    ///      "description": "List of songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Songs {
        ///List of songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&Songs> for Songs {
        fn from(value: &Songs) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Songs {
        fn default() -> Self {
            Self {
                song: Default::default(),
            }
        }
    }
    ///starred.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "starred.",
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Starred albums",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    },
    ///    "artist": {
    ///      "description": "Starred artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Artist"
    ///      }
    ///    },
    ///    "song": {
    ///      "description": "Starred songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Starred {
        ///Starred albums
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub album: ::std::vec::Vec<Child>,
        ///Starred artists
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artist: ::std::vec::Vec<Artist>,
        ///Starred songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&Starred> for Starred {
        fn from(value: &Starred) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Starred {
        fn default() -> Self {
            Self {
                album: Default::default(),
                artist: Default::default(),
                song: Default::default(),
            }
        }
    }
    ///Starred2.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Starred2.",
    ///  "type": "object",
    ///  "properties": {
    ///    "album": {
    ///      "description": "Starred albums",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AlbumID3"
    ///      }
    ///    },
    ///    "artist": {
    ///      "description": "Starred artists",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ArtistID3"
    ///      }
    ///    },
    ///    "song": {
    ///      "description": "Starred songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Starred2 {
        ///Starred albums
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub album: ::std::vec::Vec<AlbumId3>,
        ///Starred artists
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub artist: ::std::vec::Vec<ArtistId3>,
        ///Starred songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&Starred2> for Starred2 {
        fn from(value: &Starred2) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Starred2 {
        fn default() -> Self {
            Self {
                album: Default::default(),
                artist: Default::default(),
                song: Default::default(),
            }
        }
    }
    ///A subsonic-response element with a nested `scanStatus` element on success.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A subsonic-response element with a nested `scanStatus` element on success.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/StartScanSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct StartScanResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<StartScanResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&StartScanResponse> for StartScanResponse {
        fn from(value: &StartScanResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for StartScanResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`StartScanResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/StartScanSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum StartScanResponseSubsonicResponse {
        StartScanSuccessResponse(StartScanSuccessResponse),
        SubsonicFailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for StartScanResponseSubsonicResponse {
        fn from(value: &StartScanResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<StartScanSuccessResponse> for StartScanResponseSubsonicResponse {
        fn from(value: StartScanSuccessResponse) -> Self {
            Self::StartScanSuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for StartScanResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::SubsonicFailureResponse(value)
        }
    }
    ///`StartScanSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "scanStatus"
    ///      ],
    ///      "properties": {
    ///        "scanStatus": {
    ///          "$ref": "#/components/schemas/ScanStatus"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct StartScanSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        #[serde(rename = "scanStatus")]
        pub scan_status: ScanStatus,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: StartScanSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&StartScanSuccessResponse> for StartScanSuccessResponse {
        fn from(value: &StartScanSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum StartScanSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for StartScanSuccessResponseStatus {
        fn from(value: &StartScanSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for StartScanSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for StartScanSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for StartScanSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for StartScanSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for StartScanSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`StreamSize`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]+x[0-9]+$"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct StreamSize(::std::string::String);
    impl ::std::ops::Deref for StreamSize {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<StreamSize> for ::std::string::String {
        fn from(value: StreamSize) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&StreamSize> for StreamSize {
        fn from(value: &StreamSize) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for StreamSize {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9]+x[0-9]+$").unwrap());
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[0-9]+x[0-9]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for StreamSize {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for StreamSize {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for StreamSize {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StreamSize {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Structured lyrics
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Structured lyrics",
    ///  "type": "object",
    ///  "required": [
    ///    "lang",
    ///    "line",
    ///    "synced"
    ///  ],
    ///  "properties": {
    ///    "displayArtist": {
    ///      "description": "The artist name to display. This could be the localized name, or any other value",
    ///      "type": "string"
    ///    },
    ///    "displayTitle": {
    ///      "description": "The title to display. This could be the song title (localized), or any other value",
    ///      "type": "string"
    ///    },
    ///    "lang": {
    ///      "description": "The lyrics language (ideally ISO 639). If the language is unknown (e.g. lrc file), the server must return `und` (ISO standard) or `xxx` (common value for taggers). Ideally, the server will return lang as an ISO 639 (2/3) code. However, tagged files and external lyrics can come with any value as a potential language code, so clients should take care when displaying lang.\n\nFurthermore, there is special behavior for the value xxx. While not an ISO code, it is commonly used by taggers and other parsing software. Clients should treat xxx as not having a specified language (equivalent to the und code).",
    ///      "type": "string"
    ///    },
    ///    "line": {
    ///      "description": "The actual lyrics. Ordered by start time (synced) or appearance order (unsynced)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Line"
    ///      }
    ///    },
    ///    "offset": {
    ///      "description": "The offset to apply to all lyrics, in milliseconds. Positive means lyrics appear sooner, negative means later. If not included, the offset must be assumed to be 0",
    ///      "type": "number"
    ///    },
    ///    "synced": {
    ///      "description": "True if the lyrics are synced, false otherwise",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct StructuredLyrics {
        ///The artist name to display. This could be the localized name, or any other value
        #[serde(
            rename = "displayArtist",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_artist: ::std::option::Option<::std::string::String>,
        ///The title to display. This could be the song title (localized), or any other value
        #[serde(
            rename = "displayTitle",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_title: ::std::option::Option<::std::string::String>,
        /**The lyrics language (ideally ISO 639). If the language is unknown (e.g. lrc file), the server must return `und` (ISO standard) or `xxx` (common value for taggers). Ideally, the server will return lang as an ISO 639 (2/3) code. However, tagged files and external lyrics can come with any value as a potential language code, so clients should take care when displaying lang.

        Furthermore, there is special behavior for the value xxx. While not an ISO code, it is commonly used by taggers and other parsing software. Clients should treat xxx as not having a specified language (equivalent to the und code).*/
        pub lang: ::std::string::String,
        ///The actual lyrics. Ordered by start time (synced) or appearance order (unsynced)
        pub line: ::std::vec::Vec<Line>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offset: ::std::option::Option<f64>,
        ///True if the lyrics are synced, false otherwise
        pub synced: bool,
    }
    impl ::std::convert::From<&StructuredLyrics> for StructuredLyrics {
        fn from(value: &StructuredLyrics) -> Self {
            value.clone()
        }
    }
    ///`SubsonicBaseResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "serverVersion",
    ///    "type",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "openSubsonic": {
    ///      "description": "Must return true if the server support OpenSubsonic API v1",
    ///      "type": "boolean"
    ///    },
    ///    "serverVersion": {
    ///      "description": "The server version.",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "The server actual name. [Ex: Navidrome or gonic]",
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "description": "The server supported Subsonic API version.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SubsonicBaseResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&SubsonicBaseResponse> for SubsonicBaseResponse {
        fn from(value: &SubsonicBaseResponse) -> Self {
            value.clone()
        }
    }
    ///`SubsonicError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "code": 42,
    ///      "helpUrl": "https://example.org/users/apiKey",
    ///      "message": "Authentication mechanism not supported. Use API keys"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "code"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "description": "The error code.\n* 0: A generic error.\n* 10: Required parameter is missing.\n* 20: Incompatible Subsonic REST protocol version. Client must upgrade.\n* 30: Incompatible Subsonic REST protocol version. Server must upgrade.\n* 40: Wrong username or password.\n* 41: Token authentication not supported for LDAP users.\n* 42: Provided authentication mechanism not supported.\n* 43: Multiple conflicting authentication mechanisms provided.\n* 44: Invalid API key.\n* 50: User is not authorized for the given operation.\n* 60: The trial period for the Subsonic server is over. Please upgrade to Subsonic Premium. Visit subsonic.org for details.\n* 70: The requested data was not found.",
    ///      "type": "integer",
    ///      "enum": [
    ///        0,
    ///        10,
    ///        20,
    ///        30,
    ///        40,
    ///        41,
    ///        42,
    ///        43,
    ///        44,
    ///        50,
    ///        60,
    ///        70
    ///      ]
    ///    },
    ///    "helpUrl": {
    ///      "description": "A URL (documentation, configuration, etc) which may provide additional context for the error)",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "The optional error message",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SubsonicError {
        /**The error code.
         * 0: A generic error.
         * 10: Required parameter is missing.
         * 20: Incompatible Subsonic REST protocol version. Client must upgrade.
         * 30: Incompatible Subsonic REST protocol version. Server must upgrade.
         * 40: Wrong username or password.
         * 41: Token authentication not supported for LDAP users.
         * 42: Provided authentication mechanism not supported.
         * 43: Multiple conflicting authentication mechanisms provided.
         * 44: Invalid API key.
         * 50: User is not authorized for the given operation.
         * 60: The trial period for the Subsonic server is over. Please upgrade to Subsonic Premium. Visit subsonic.org for details.
         * 70: The requested data was not found.*/
        pub code: SubsonicErrorCode,
        ///A URL (documentation, configuration, etc) which may provide additional context for the error)
        #[serde(
            rename = "helpUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help_url: ::std::option::Option<::std::string::String>,
        ///The optional error message
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&SubsonicError> for SubsonicError {
        fn from(value: &SubsonicError) -> Self {
            value.clone()
        }
    }
    /**The error code.
     * 0: A generic error.
     * 10: Required parameter is missing.
     * 20: Incompatible Subsonic REST protocol version. Client must upgrade.
     * 30: Incompatible Subsonic REST protocol version. Server must upgrade.
     * 40: Wrong username or password.
     * 41: Token authentication not supported for LDAP users.
     * 42: Provided authentication mechanism not supported.
     * 43: Multiple conflicting authentication mechanisms provided.
     * 44: Invalid API key.
     * 50: User is not authorized for the given operation.
     * 60: The trial period for the Subsonic server is over. Please upgrade to Subsonic Premium. Visit subsonic.org for details.
     * 70: The requested data was not found.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The error code.\n* 0: A generic error.\n* 10: Required parameter is missing.\n* 20: Incompatible Subsonic REST protocol version. Client must upgrade.\n* 30: Incompatible Subsonic REST protocol version. Server must upgrade.\n* 40: Wrong username or password.\n* 41: Token authentication not supported for LDAP users.\n* 42: Provided authentication mechanism not supported.\n* 43: Multiple conflicting authentication mechanisms provided.\n* 44: Invalid API key.\n* 50: User is not authorized for the given operation.\n* 60: The trial period for the Subsonic server is over. Please upgrade to Subsonic Premium. Visit subsonic.org for details.\n* 70: The requested data was not found.",
    ///  "type": "integer",
    ///  "enum": [
    ///    0,
    ///    10,
    ///    20,
    ///    30,
    ///    40,
    ///    41,
    ///    42,
    ///    43,
    ///    44,
    ///    50,
    ///    60,
    ///    70
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct SubsonicErrorCode(i64);
    impl ::std::ops::Deref for SubsonicErrorCode {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<SubsonicErrorCode> for i64 {
        fn from(value: SubsonicErrorCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&SubsonicErrorCode> for SubsonicErrorCode {
        fn from(value: &SubsonicErrorCode) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for SubsonicErrorCode {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![
                0_i64, 10_i64, 20_i64, 30_i64, 40_i64, 41_i64, 42_i64, 43_i64, 44_i64, 50_i64,
                60_i64, 70_i64,
            ]
            .contains(&value)
            {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SubsonicErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`SubsonicFailureResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicBaseResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error",
    ///        "status"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "$ref": "#/components/schemas/SubsonicError"
    ///        },
    ///        "status": {
    ///          "description": "The command result. `failed`",
    ///          "type": "string",
    ///          "enum": [
    ///            "failed"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SubsonicFailureResponse {
        pub error: SubsonicError,
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `failed`
        pub status: SubsonicFailureResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&SubsonicFailureResponse> for SubsonicFailureResponse {
        fn from(value: &SubsonicFailureResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `failed`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `failed`",
    ///  "type": "string",
    ///  "enum": [
    ///    "failed"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SubsonicFailureResponseStatus {
        #[serde(rename = "failed")]
        Failed,
    }
    impl ::std::convert::From<&Self> for SubsonicFailureResponseStatus {
        fn from(value: &SubsonicFailureResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for SubsonicFailureResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Failed => f.write_str("failed"),
            }
        }
    }
    impl ::std::str::FromStr for SubsonicFailureResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "failed" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SubsonicFailureResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SubsonicFailureResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SubsonicFailureResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Common answer wrapper.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Common answer wrapper.",
    ///  "type": "object",
    ///  "properties": {
    ///    "subsonic-response": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SubsonicResponse {
        #[serde(
            rename = "subsonic-response",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subsonic_response: ::std::option::Option<SubsonicResponseSubsonicResponse>,
    }
    impl ::std::convert::From<&SubsonicResponse> for SubsonicResponse {
        fn from(value: &SubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for SubsonicResponse {
        fn default() -> Self {
            Self {
                subsonic_response: Default::default(),
            }
        }
    }
    ///`SubsonicResponseSubsonicResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicSuccessResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicFailureResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SubsonicResponseSubsonicResponse {
        SuccessResponse(SubsonicSuccessResponse),
        FailureResponse(SubsonicFailureResponse),
    }
    impl ::std::convert::From<&Self> for SubsonicResponseSubsonicResponse {
        fn from(value: &SubsonicResponseSubsonicResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<SubsonicSuccessResponse> for SubsonicResponseSubsonicResponse {
        fn from(value: SubsonicSuccessResponse) -> Self {
            Self::SuccessResponse(value)
        }
    }
    impl ::std::convert::From<SubsonicFailureResponse> for SubsonicResponseSubsonicResponse {
        fn from(value: SubsonicFailureResponse) -> Self {
            Self::FailureResponse(value)
        }
    }
    ///`SubsonicSuccessResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SubsonicBaseResponse"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "status"
    ///      ],
    ///      "properties": {
    ///        "status": {
    ///          "description": "The command result. `ok`",
    ///          "type": "string",
    ///          "enum": [
    ///            "ok"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SubsonicSuccessResponse {
        ///Must return true if the server support OpenSubsonic API v1
        #[serde(
            rename = "openSubsonic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_subsonic: ::std::option::Option<bool>,
        ///The server version.
        #[serde(rename = "serverVersion")]
        pub server_version: ::std::string::String,
        ///The command result. `ok`
        pub status: SubsonicSuccessResponseStatus,
        ///The server actual name. [Ex: Navidrome or gonic]
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        ///The server supported Subsonic API version.
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&SubsonicSuccessResponse> for SubsonicSuccessResponse {
        fn from(value: &SubsonicSuccessResponse) -> Self {
            value.clone()
        }
    }
    ///The command result. `ok`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The command result. `ok`",
    ///  "type": "string",
    ///  "enum": [
    ///    "ok"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SubsonicSuccessResponseStatus {
        #[serde(rename = "ok")]
        Ok,
    }
    impl ::std::convert::From<&Self> for SubsonicSuccessResponseStatus {
        fn from(value: &SubsonicSuccessResponseStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for SubsonicSuccessResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ok => f.write_str("ok"),
            }
        }
    }
    impl ::std::str::FromStr for SubsonicSuccessResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SubsonicSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SubsonicSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SubsonicSuccessResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Information about an API key
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Information about an API key",
    ///  "type": "object",
    ///  "required": [
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "username": {
    ///      "description": "Username associated with token",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TokenInfo {
        ///Username associated with token
        pub username: ::std::string::String,
    }
    impl ::std::convert::From<&TokenInfo> for TokenInfo {
        fn from(value: &TokenInfo) -> Self {
            value.clone()
        }
    }
    ///TopSongs list.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "TopSongs list.",
    ///  "type": "object",
    ///  "properties": {
    ///    "song": {
    ///      "description": "List of songs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Child"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TopSongs {
        ///List of songs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub song: ::std::vec::Vec<Child>,
    }
    impl ::std::convert::From<&TopSongs> for TopSongs {
        fn from(value: &TopSongs) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for TopSongs {
        fn default() -> Self {
            Self {
                song: Default::default(),
            }
        }
    }
    ///`UpdateUserMaxBitRate`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "integer",
    ///  "enum": [
    ///    0,
    ///    32,
    ///    40,
    ///    48,
    ///    56,
    ///    64,
    ///    80,
    ///    96,
    ///    112,
    ///    128,
    ///    160,
    ///    192,
    ///    224,
    ///    256,
    ///    320
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct UpdateUserMaxBitRate(i64);
    impl ::std::ops::Deref for UpdateUserMaxBitRate {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<UpdateUserMaxBitRate> for i64 {
        fn from(value: UpdateUserMaxBitRate) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UpdateUserMaxBitRate> for UpdateUserMaxBitRate {
        fn from(value: &UpdateUserMaxBitRate) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for UpdateUserMaxBitRate {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![
                0_i64, 32_i64, 40_i64, 48_i64, 56_i64, 64_i64, 80_i64, 96_i64, 112_i64, 128_i64,
                160_i64, 192_i64, 224_i64, 256_i64, 320_i64,
            ]
            .contains(&value)
            {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateUserMaxBitRate {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///user.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "user.",
    ///  "type": "object",
    ///  "required": [
    ///    "adminRole",
    ///    "commentRole",
    ///    "coverArtRole",
    ///    "downloadRole",
    ///    "jukeboxRole",
    ///    "playlistRole",
    ///    "podcastRole",
    ///    "scrobblingEnabled",
    ///    "settingsRole",
    ///    "shareRole",
    ///    "streamRole",
    ///    "uploadRole",
    ///    "username",
    ///    "videoConversionRole"
    ///  ],
    ///  "properties": {
    ///    "adminRole": {
    ///      "description": "Whether the user is an admin",
    ///      "type": "boolean"
    ///    },
    ///    "avatarLastChanged": {
    ///      "description": "Last time the avatar was changed [ISO 8601]",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "commentRole": {
    ///      "description": "Whether the user can create comments",
    ///      "type": "boolean"
    ///    },
    ///    "coverArtRole": {
    ///      "description": "Whether the user can get cover art",
    ///      "type": "boolean"
    ///    },
    ///    "downloadRole": {
    ///      "description": "Whether the user can download",
    ///      "type": "boolean"
    ///    },
    ///    "folder": {
    ///      "description": "Folder ID(s)",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "jukeboxRole": {
    ///      "description": "Whether the user can control the jukebox",
    ///      "type": "boolean"
    ///    },
    ///    "maxBitRate": {
    ///      "type": "integer"
    ///    },
    ///    "playlistRole": {
    ///      "description": "Whether the user can create playlists",
    ///      "type": "boolean"
    ///    },
    ///    "podcastRole": {
    ///      "description": "Whether the user can create/refresh podcasts",
    ///      "type": "boolean"
    ///    },
    ///    "scrobblingEnabled": {
    ///      "description": "Scrobbling enabled",
    ///      "type": "boolean"
    ///    },
    ///    "settingsRole": {
    ///      "description": "Whether the user is can edit settings",
    ///      "type": "boolean"
    ///    },
    ///    "shareRole": {
    ///      "description": "Whether the user can create a stream",
    ///      "type": "boolean"
    ///    },
    ///    "streamRole": {
    ///      "description": "Whether the user can stream",
    ///      "type": "boolean"
    ///    },
    ///    "uploadRole": {
    ///      "description": "Whether the user can upload",
    ///      "type": "boolean"
    ///    },
    ///    "username": {
    ///      "description": "Username",
    ///      "type": "string"
    ///    },
    ///    "videoConversionRole": {
    ///      "description": "Whether the user can convert videos",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct User {
        ///Whether the user is an admin
        #[serde(rename = "adminRole")]
        pub admin_role: bool,
        ///Last time the avatar was changed [ISO 8601]
        #[serde(
            rename = "avatarLastChanged",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avatar_last_changed: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Whether the user can create comments
        #[serde(rename = "commentRole")]
        pub comment_role: bool,
        ///Whether the user can get cover art
        #[serde(rename = "coverArtRole")]
        pub cover_art_role: bool,
        ///Whether the user can download
        #[serde(rename = "downloadRole")]
        pub download_role: bool,
        ///Folder ID(s)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub folder: ::std::vec::Vec<i64>,
        ///Whether the user can control the jukebox
        #[serde(rename = "jukeboxRole")]
        pub jukebox_role: bool,
        #[serde(
            rename = "maxBitRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_bit_rate: ::std::option::Option<i64>,
        ///Whether the user can create playlists
        #[serde(rename = "playlistRole")]
        pub playlist_role: bool,
        ///Whether the user can create/refresh podcasts
        #[serde(rename = "podcastRole")]
        pub podcast_role: bool,
        ///Scrobbling enabled
        #[serde(rename = "scrobblingEnabled")]
        pub scrobbling_enabled: bool,
        ///Whether the user is can edit settings
        #[serde(rename = "settingsRole")]
        pub settings_role: bool,
        ///Whether the user can create a stream
        #[serde(rename = "shareRole")]
        pub share_role: bool,
        ///Whether the user can stream
        #[serde(rename = "streamRole")]
        pub stream_role: bool,
        ///Whether the user can upload
        #[serde(rename = "uploadRole")]
        pub upload_role: bool,
        ///Username
        pub username: ::std::string::String,
        ///Whether the user can convert videos
        #[serde(rename = "videoConversionRole")]
        pub video_conversion_role: bool,
    }
    impl ::std::convert::From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
    ///users.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "users.",
    ///  "type": "object",
    ///  "properties": {
    ///    "user": {
    ///      "description": "Array of users",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/User"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Users {
        ///Array of users
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub user: ::std::vec::Vec<User>,
    }
    impl ::std::convert::From<&Users> for Users {
        fn from(value: &Users) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Users {
        fn default() -> Self {
            Self {
                user: Default::default(),
            }
        }
    }
    ///videoInfo. TODO
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "videoInfo. TODO",
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct VideoInfo(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for VideoInfo {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<VideoInfo>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: VideoInfo) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&VideoInfo> for VideoInfo {
        fn from(value: &VideoInfo) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for VideoInfo
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///videos. TODO
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "videos. TODO",
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Videos(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for Videos {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<Videos>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: Videos) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&Videos> for Videos {
        fn from(value: &Videos) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for Videos
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }
        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: ::std::convert::TryFrom<u64>,
            <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }
        pub(super) fn default_nzu64<T, const V: u64>() -> T
        where
            T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
            <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
        {
            T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for OpenSubsonic API

OpenSubsonic API documentation.

Version: 1.16.1*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.16.1"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**Adds a message to the chat log

    Adds a message to the chat log.

    Sends a `GET` request to `/rest/addChatMessage`

    Arguments:
    - `message`: The chat message.
    */
    pub async fn get_add_chat_message<'a>(
        &'a self,
        message: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/addChatMessage", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("message", &message))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_add_chat_message",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Adds a message to the chat log

    Adds a message to the chat log.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/addChatMessage`

    */
    pub async fn post_add_chat_message<'a>(
        &'a self,
        body: &'a types::PostAddChatMessageBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/addChatMessage", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_add_chat_message",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Changes the password of an existing user on the server

    Changes the password of an existing user on the server, using the following parameters. You can only change your own password unless you have admin privileges.

    Sends a `GET` request to `/rest/changePassword`

    Arguments:
    - `password`: The new password of the new user, either in clear text of hex-encoded (see above).
    - `username`: The name of the user which should change its password.
    */
    pub async fn change_password<'a>(
        &'a self,
        password: &'a str,
        username: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/changePassword", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("password", &password))
            .query(&progenitor_client::QueryParam::new("username", &username))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "change_password",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Changes the password of an existing user on the server

    Changes the password of an existing user on the server, using the following parameters. You can only change your own password unless you have admin privileges.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/changePassword`

    */
    pub async fn post_change_password<'a>(
        &'a self,
        body: &'a types::PostChangePasswordBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/changePassword", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_change_password",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates or updates a bookmark

    Creates or updates a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.

    Sends a `GET` request to `/rest/createBookmark`

    Arguments:
    - `comment`: A user-defined comment.
    - `id`: ID of the media file to bookmark. If a bookmark already exists for this file it will be overwritten.
    - `position`: The position (in milliseconds) within the media file.
    */
    pub async fn create_bookmark<'a>(
        &'a self,
        comment: Option<&'a str>,
        id: &'a str,
        position: i64,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createBookmark", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("comment", &comment))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("position", &position))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_bookmark",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates or updates a bookmark

    Creates or updates a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/createBookmark`

    */
    pub async fn post_create_bookmark<'a>(
        &'a self,
        body: &'a types::PostCreateBookmarkBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createBookmark", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_create_bookmark",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Adds a new internet radio station

    Adds a new internet radio station. Only users with admin privileges are allowed to call this method.

    Sends a `GET` request to `/rest/createInternetRadioStation`

    Arguments:
    - `homepage_url`: The home page URL for the station.
    - `name`: The station name.
    - `stream_url`: The stream URL for the station.
    */
    pub async fn create_internet_radio_station<'a>(
        &'a self,
        homepage_url: Option<&'a str>,
        name: &'a str,
        stream_url: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createInternetRadioStation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "homepageUrl",
                &homepage_url,
            ))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new(
                "streamUrl",
                &stream_url,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_internet_radio_station",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Adds a new internet radio station

    Adds a new internet radio station. Only users with admin privileges are allowed to call this method.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/createInternetRadioStation`

    */
    pub async fn post_create_internet_radio_station<'a>(
        &'a self,
        body: &'a types::PostCreateInternetRadioStationBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createInternetRadioStation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_create_internet_radio_station",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates (or updates) a playlist

    Creates (or updates) a playlist.

    Sends a `GET` request to `/rest/createPlaylist`

    Arguments:
    - `name`: The human-readable name of the playlist. Required if creating a new playlist.
    - `playlist_id`: The playlist ID. Required if updating an existing playlist.
    - `song_id`: ID of a song in the playlist. Use one `songId` parameter for each song in the playlist.
    */
    pub async fn create_playlist<'a>(
        &'a self,
        name: Option<&'a str>,
        playlist_id: Option<&'a str>,
        song_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
    ) -> Result<ResponseValue<types::CreatePlaylistResponse>, Error<()>> {
        let url = format!("{}/rest/createPlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new(
                "playlistId",
                &playlist_id,
            ))
            .query(&progenitor_client::QueryParam::new("songId", &song_id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates (or updates) a playlist

    Creates (or updates) a playlist.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/createPlaylist`

    */
    pub async fn post_create_playlist<'a>(
        &'a self,
        body: &'a types::PostCreatePlaylistBody,
    ) -> Result<ResponseValue<types::CreatePlaylistResponse>, Error<()>> {
        let url = format!("{}/rest/createPlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_create_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Adds a new Podcast channel

    Adds a new Podcast channel. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Sends a `GET` request to `/rest/createPodcastChannel`

    Arguments:
    - `url`: The URL of the Podcast to add.
    */
    pub async fn create_podcast_channel<'a>(
        &'a self,
        url: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let _url = format!("{}/rest/createPodcastChannel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(_url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("url", &url))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_podcast_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Adds a new Podcast channel

    Adds a new Podcast channel. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/createPodcastChannel`

    */
    pub async fn post_create_podcast_channel<'a>(
        &'a self,
        body: &'a types::PostCreatePodcastChannelBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createPodcastChannel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_create_podcast_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates a public URL that can be used by anyone to stream music or video from the server

    Creates a public URL that can be used by anyone to stream music or video from the server. The URL is short and suitable for posting on Facebook, Twitter etc. Note: The user must be authorized to share (see Settings > Users > User is allowed to share files with anyone). Since 1.6.0.

    Sends a `GET` request to `/rest/createShare`

    Arguments:
    - `description`: A user-defined description that will be displayed to people visiting the shared media.
    - `expires`: The time at which the share expires. Given as milliseconds since 1970.
    - `id`: ID of a song, album or video to share. Use one id parameter for each entry to share.
    */
    pub async fn create_share<'a>(
        &'a self,
        description: Option<&'a str>,
        expires: Option<i64>,
        id: &'a ::std::vec::Vec<::std::string::String>,
    ) -> Result<ResponseValue<types::CreateSharesResponse>, Error<()>> {
        let url = format!("{}/rest/createShare", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "description",
                &description,
            ))
            .query(&progenitor_client::QueryParam::new("expires", &expires))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates a public URL that can be used by anyone to stream music or video from the server

    Creates a public URL that can be used by anyone to stream music or video from the server. The URL is short and suitable for posting on Facebook, Twitter etc. Note: The user must be authorized to share (see Settings > Users > User is allowed to share files with anyone). Since 1.6.0.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`).

    Sends a `POST` request to `/rest/createShare`

    */
    pub async fn post_create_share<'a>(
        &'a self,
        body: &'a types::PostCreateShareBody,
    ) -> Result<ResponseValue<types::CreateSharesResponse>, Error<()>> {
        let url = format!("{}/rest/createShare", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_create_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates a new user on the server

    Creates a new user on the server.

    Sends a `GET` request to `/rest/createUser`

    Arguments:
    - `admin_role`: Whether the user is administrator.
    - `comment_role`: Whether the user is allowed to create and edit comments and ratings.
    - `cover_art_role`: Whether the user is allowed to change cover art and tags.
    - `download_role`: Whether the user is allowed to download files.
    - `email`: The email address of the new user.
    - `jukebox_role`: Whether the user is allowed to play files in jukebox mode.
    - `ldap_authenticated`: Whether the user is authenticated in LDAP.
    - `music_folder_id`: (Since 1.12.0) IDs of the music folders the user is allowed access to. Include the parameter once for each folder. Default all folders.
    - `password`: The password of the new user, either in clear text of hex-encoded (see above).
    - `playlist_role`: Whether the user is allowed to create and delete playlists. Since 1.8.0, changing this role has no effect.
    - `podcast_role`: Whether the user is allowed to administrate Podcasts.
    - `settings_role`: Whether the user is allowed to change personal settings and password.
    - `share_role`: (Since 1.8.0) Whether the user is allowed to share files with anyone.
    - `stream_role`: Whether the user is allowed to play files.
    - `upload_role`: Whether the user is allowed to upload files.
    - `username`: The name of the new user.
    - `video_conversion_role`: (Since 1.15.0) Whether the user is allowed to start video conversions.
    */
    pub async fn create_user<'a>(
        &'a self,
        admin_role: Option<bool>,
        comment_role: Option<bool>,
        cover_art_role: Option<bool>,
        download_role: Option<bool>,
        email: &'a str,
        jukebox_role: Option<bool>,
        ldap_authenticated: Option<bool>,
        music_folder_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        password: &'a str,
        playlist_role: Option<bool>,
        podcast_role: Option<bool>,
        settings_role: Option<bool>,
        share_role: Option<bool>,
        stream_role: Option<bool>,
        upload_role: Option<bool>,
        username: &'a str,
        video_conversion_role: Option<bool>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "adminRole",
                &admin_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "commentRole",
                &comment_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "coverArtRole",
                &cover_art_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "downloadRole",
                &download_role,
            ))
            .query(&progenitor_client::QueryParam::new("email", &email))
            .query(&progenitor_client::QueryParam::new(
                "jukeboxRole",
                &jukebox_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "ldapAuthenticated",
                &ldap_authenticated,
            ))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("password", &password))
            .query(&progenitor_client::QueryParam::new(
                "playlistRole",
                &playlist_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "podcastRole",
                &podcast_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "settingsRole",
                &settings_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "shareRole",
                &share_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "streamRole",
                &stream_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadRole",
                &upload_role,
            ))
            .query(&progenitor_client::QueryParam::new("username", &username))
            .query(&progenitor_client::QueryParam::new(
                "videoConversionRole",
                &video_conversion_role,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates a new user on the server

    Creates a new user on the server.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/createUser`

    */
    pub async fn post_create_user<'a>(
        &'a self,
        body: &'a types::PostCreateUserBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/createUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_create_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a bookmark

    Deletes a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.

    Sends a `GET` request to `/rest/deleteBookmark`

    Arguments:
    - `id`: ID of the media file for which to delete the bookmark. Other users’ bookmarks are not affected.
    */
    pub async fn delete_bookmark<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteBookmark", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_bookmark",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a bookmark

    Deletes a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deleteBookmark`

    */
    pub async fn post_delete_bookmark<'a>(
        &'a self,
        body: &'a types::PostDeleteBookmarkBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteBookmark", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_bookmark",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes an existing internet radio station

    Deletes an existing internet radio station. Only users with admin privileges are allowed to call this method.

    Sends a `GET` request to `/rest/deleteInternetRadioStation`

    Arguments:
    - `id`: The ID for the station.
    */
    pub async fn delete_internet_radio_station<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteInternetRadioStation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_internet_radio_station",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes an existing internet radio station

    Deletes an existing internet radio station. Only users with admin privileges are allowed to call this method.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deleteInternetRadioStation`

    */
    pub async fn post_delete_internet_radio_station<'a>(
        &'a self,
        body: &'a types::PostDeleteInternetRadioStationBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteInternetRadioStation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_internet_radio_station",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a saved playlist

    Deletes a saved playlist.

    Sends a `GET` request to `/rest/deletePlaylist`

    Arguments:
    - `id`: ID of the playlist to delete, as obtained by `getPlaylists`.
    */
    pub async fn delete_playlist<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deletePlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a saved playlist

    Deletes a saved playlist.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deletePlaylist`

    */
    pub async fn post_delete_playlist<'a>(
        &'a self,
        body: &'a types::PostDeletePlaylistBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deletePlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a Podcast channel

    Deletes a Podcast channel. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Sends a `GET` request to `/rest/deletePodcastChannel`

    Arguments:
    - `id`: The ID of the Podcast channel to delete.
    */
    pub async fn delete_podcast_channel<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deletePodcastChannel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_podcast_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a Podcast channel

    Deletes a Podcast channel. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deletePodcastChannel`

    */
    pub async fn post_delete_podcast_channel<'a>(
        &'a self,
        body: &'a types::PostDeletePodcastChannelBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deletePodcastChannel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_podcast_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a Podcast episode

    Deletes a Podcast episode. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Sends a `GET` request to `/rest/deletePodcastEpisode`

    Arguments:
    - `id`: The ID of the Podcast episode to delete.
    */
    pub async fn delete_podcast_episode<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deletePodcastEpisode", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_podcast_episode",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes a Podcast episode

    Deletes a Podcast episode. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deletePodcastEpisode`

    */
    pub async fn post_delete_podcast_episode<'a>(
        &'a self,
        body: &'a types::PostDeletePodcastEpisodeBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deletePodcastEpisode", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_podcast_episode",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes an existing share

    Deletes an existing share

    Sends a `GET` request to `/rest/deleteShare`

    Arguments:
    - `id`: ID of the share to delete.
    */
    pub async fn delete_share<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteShare", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes an existing share

    Deletes an existing share.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deleteShare`

    */
    pub async fn post_delete_share<'a>(
        &'a self,
        body: &'a types::PostDeleteShareBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteShare", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes an existing user on the server

    Deletes an existing user on the server.

    Sends a `GET` request to `/rest/deleteUser`

    Arguments:
    - `username`: The name of the user to delete.
    */
    pub async fn delete_user<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("username", &username))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Deletes an existing user on the server

    Deletes an existing user on the server.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/deleteUser`

    */
    pub async fn post_delete_user<'a>(
        &'a self,
        body: &'a types::PostDeleteUserBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/deleteUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_delete_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Downloads a given media file

    Downloads a given media file. Similar to stream, but this method returns the original media data without transcoding or downsampling.

    Sends a `GET` request to `/rest/download`

    Arguments:
    - `id`: A string which uniquely identifies the file to stream. Obtained by calls to getMusicDirectory.
    */
    pub async fn download<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/download", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "download",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Downloads a given media file

    Downloads a given media file. Similar to stream, but this method returns the original media data without transcoding or downsampling.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/download`

    */
    pub async fn post_download<'a>(
        &'a self,
        body: &'a types::PostDownloadBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/download", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_download",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Request the server to start downloading a given Podcast episode

    Request the server to start downloading a given Podcast episode. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Sends a `GET` request to `/rest/downloadPodcastEpisode`

    Arguments:
    - `id`: The ID of the Podcast episode to download
    */
    pub async fn download_podcast_episode<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/downloadPodcastEpisode", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "download_podcast_episode",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Request the server to start downloading a given Podcast episode

    Request the server to start downloading a given Podcast episode. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/downloadPodcastEpisode`

    */
    pub async fn post_download_podcast_episode<'a>(
        &'a self,
        body: &'a types::PostDownloadPodcastEpisodeBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/downloadPodcastEpisode", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_download_podcast_episode",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for an album

    Returns details for an album, including a list of songs. This method organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getAlbum`

    Arguments:
    - `id`: The album ID.
    */
    pub async fn get_album<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetAlbumResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbum", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_album",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for an album

    Returns details for an album, including a list of songs. This method organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getAlbum`

    */
    pub async fn post_get_album<'a>(
        &'a self,
        body: &'a types::PostGetAlbumBody,
    ) -> Result<ResponseValue<types::GetAlbumResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbum", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_album",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns album info

    Returns album notes, image URLs etc, using data from last.fm.

    Sends a `GET` request to `/rest/getAlbumInfo`

    Arguments:
    - `id`: The album ID or song ID.
    */
    pub async fn get_album_info<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetAlbumInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbumInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_album_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns album info

    Returns album notes, image URLs etc, using data from last.fm.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getAlbumInfo`

    */
    pub async fn post_get_album_info<'a>(
        &'a self,
        body: &'a types::PostGetAlbumInfoBody,
    ) -> Result<ResponseValue<types::GetAlbumInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbumInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_album_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns album info (v2)

    Similar to getAlbumInfo, but organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getAlbumInfo2`

    Arguments:
    - `id`: The album ID or song ID.
    */
    pub async fn get_album_info2<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetAlbumInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbumInfo2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_album_info2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns album info (v2)

    Similar to getAlbumInfo, but organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getAlbumInfo2`

    */
    pub async fn post_get_album_info2<'a>(
        &'a self,
        body: &'a types::PostGetAlbumInfo2Body,
    ) -> Result<ResponseValue<types::GetAlbumInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbumInfo2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_album_info2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a list of random, newest, highest rated etc. albums

    Returns a list of random, newest, highest rated etc. albums. Similar to the album lists on the home page of the Subsonic web interface.

    Sends a `GET` request to `/rest/getAlbumList`

    Arguments:
    - `from_year`: Required if `type=='byYear'`. The first year in the range. If `fromYear` > `toYear` a reverse chronological list is returned.
    - `genre`: Required if `type=='byGenre'`. The name of the genre, e.g., “Rock”.
    - `music_folder_id`: (Since 1.11.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    - `offset`: The list offset. Useful if you for example want to page through the list of newest albums.
    - `size`: The number of albums to return. Max 500.
    - `to_year`: Required if `type=='byYear'`. The last year in the range.
    - `type_`
    */
    pub async fn get_album_list<'a>(
        &'a self,
        from_year: Option<i64>,
        genre: Option<&'a str>,
        music_folder_id: Option<&'a str>,
        offset: Option<u64>,
        size: Option<::std::num::NonZeroU64>,
        to_year: Option<i64>,
        type_: types::AlbumListType,
    ) -> Result<ResponseValue<types::GetAlbumListResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbumList", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("fromYear", &from_year))
            .query(&progenitor_client::QueryParam::new("genre", &genre))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("size", &size))
            .query(&progenitor_client::QueryParam::new("toYear", &to_year))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_album_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a list of random, newest, highest rated etc. albums

    Returns a list of random, newest, highest rated etc. albums. Similar to the album lists on the home page of the Subsonic web interface.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getAlbumList`

    */
    pub async fn post_get_album_list<'a>(
        &'a self,
        body: &'a types::PostGetAlbumListBody,
    ) -> Result<ResponseValue<types::GetAlbumListResponse>, Error<()>> {
        let url = format!("{}/rest/getAlbumList", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_album_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a list of random, newest, highest rated etc. albums (v2)

    Similar to `getAlbumList`, but organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getAlbumList2`

    Arguments:
    - `from_year`: Required if `type=='byYear'`. The first year in the range. If `fromYear` > `toYear` a reverse chronological list is returned.
    - `genre`: Required if `type=='byGenre'`. The name of the genre, e.g., “Rock”.
    - `music_folder_id`: (Since 1.11.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    - `offset`: The list offset. Useful if you for example want to page through the list of newest albums.
    - `size`: The number of albums to return. Max 500.
    - `to_year`: Required if `type=='byYear'`. The last year in the range.
    - `type_`
    */
    pub async fn get_album_list2<'a>(
        &'a self,
        from_year: Option<i64>,
        genre: Option<&'a str>,
        music_folder_id: Option<&'a str>,
        offset: Option<u64>,
        size: Option<::std::num::NonZeroU64>,
        to_year: Option<i64>,
        type_: types::AlbumListType,
    ) -> Result<ResponseValue<types::GetAlbumList2Response>, Error<()>> {
        let url = format!("{}/rest/getAlbumList2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("fromYear", &from_year))
            .query(&progenitor_client::QueryParam::new("genre", &genre))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("size", &size))
            .query(&progenitor_client::QueryParam::new("toYear", &to_year))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_album_list2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a list of random, newest, highest rated etc. albums (v2)

    Similar to `getAlbumList`, but organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getAlbumList2`

    */
    pub async fn post_get_album_list2<'a>(
        &'a self,
        body: &'a types::PostGetAlbumList2Body,
    ) -> Result<ResponseValue<types::GetAlbumList2Response>, Error<()>> {
        let url = format!("{}/rest/getAlbumList2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_album_list2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for an artist

    Returns details for an artist, including a list of albums. This method organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getArtist`

    Arguments:
    - `id`: The artist ID.
    */
    pub async fn get_artist<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetArtistResponse>, Error<()>> {
        let url = format!("{}/rest/getArtist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_artist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for an artist

    Returns details for an artist, including a list of albums. This method organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getArtist`

    */
    pub async fn post_get_artist<'a>(
        &'a self,
        body: &'a types::PostGetArtistBody,
    ) -> Result<ResponseValue<types::GetArtistResponse>, Error<()>> {
        let url = format!("{}/rest/getArtist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_artist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns artist info

    Returns artist info with biography, image URLs and similar artists, using data from last.fm.

    Sends a `GET` request to `/rest/getArtistInfo`

    Arguments:
    - `count`: Max number of similar artists to return.
    - `id`: The artist, album or song ID.
    - `include_not_present`: Whether to return artists that are not present in the media library.
    */
    pub async fn get_artist_info<'a>(
        &'a self,
        count: Option<u64>,
        id: &'a str,
        include_not_present: Option<bool>,
    ) -> Result<ResponseValue<types::GetArtistInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getArtistInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new(
                "includeNotPresent",
                &include_not_present,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_artist_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns artist info

    Returns artist info with biography, image URLs and similar artists, using data from last.fm.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getArtistInfo`

    */
    pub async fn post_get_artist_info<'a>(
        &'a self,
        body: &'a types::PostGetArtistInfoBody,
    ) -> Result<ResponseValue<types::GetArtistInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getArtistInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_artist_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns artist info (v2)

    Similar to `getArtistInfo`, but organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getArtistInfo2`

    Arguments:
    - `count`: Max number of similar artists to return.
    - `id`: The artist, album or song ID.
    - `include_not_present`: Whether to return artists that are not present in the media library.
    */
    pub async fn get_artist_info2<'a>(
        &'a self,
        count: Option<u64>,
        id: &'a str,
        include_not_present: Option<bool>,
    ) -> Result<ResponseValue<types::GetArtistInfo2Response>, Error<()>> {
        let url = format!("{}/rest/getArtistInfo2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new(
                "includeNotPresent",
                &include_not_present,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_artist_info2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns artist info (v2)

    Similar to `getArtistInfo`, but organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getArtistInfo2`

    */
    pub async fn post_get_artist_info2<'a>(
        &'a self,
        body: &'a types::PostGetArtistInfo2Body,
    ) -> Result<ResponseValue<types::GetArtistInfo2Response>, Error<()>> {
        let url = format!("{}/rest/getArtistInfo2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_artist_info2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all artists

    Similar to `getIndexes`, but organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getArtists`

    Arguments:
    - `music_folder_id`: If specified, only return artists in the music folder with the given ID. See `getMusicFolders`.
    */
    pub async fn get_artists<'a>(
        &'a self,
        music_folder_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::GetArtistsResponse>, Error<()>> {
        let url = format!("{}/rest/getArtists", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_artists",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all artists

    Similar to `getIndexes`, but organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getArtists`

    */
    pub async fn post_get_artists<'a>(
        &'a self,
        body: &'a types::PostGetArtistsBody,
    ) -> Result<ResponseValue<types::GetArtistsResponse>, Error<()>> {
        let url = format!("{}/rest/getArtists", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_artists",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the avatar (personal image) for a user

    Returns the avatar (personal image) for a user.

    Sends a `GET` request to `/rest/getAvatar`

    */
    pub async fn get_avatar<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/getAvatar", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("username", &username))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_avatar",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the avatar (personal image) for a user

    Returns the avatar (personal image) for a user.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getAvatar`

    */
    pub async fn post_get_avatar<'a>(
        &'a self,
        body: &'a types::PostGetAvatarBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/getAvatar", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_avatar",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all bookmarks for this user

    Returns all bookmarks for this user. A bookmark is a position within a certain media file.

    Sends a `GET` request to `/rest/getBookmarks`

    */
    pub async fn get_bookmarks<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetBookmarksResponse>, Error<()>> {
        let url = format!("{}/rest/getBookmarks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_bookmarks",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all bookmarks for this user

    Returns all bookmarks for this user. A bookmark is a position within a certain media file.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getBookmarks`

    */
    pub async fn post_get_bookmarks<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetBookmarksResponse>, Error<()>> {
        let url = format!("{}/rest/getBookmarks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_bookmarks",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns captions (subtitles) for a video

    Returns captions (subtitles) for a video. Use `getVideoInfo` to get a list of available captions.

    Sends a `GET` request to `/rest/getCaptions`

    Arguments:
    - `format`: Preferred captions format (“srt” or “vtt”).
    - `id`: The ID of the video.
    */
    pub async fn get_captions<'a>(
        &'a self,
        format: Option<types::GetCaptionsFormat>,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/getCaptions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("format", &format))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_captions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns captions (subtitles) for a video

    Returns captions (subtitles) for a video. Use `getVideoInfo` to get a list of available captions.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getCaptions`

    */
    pub async fn post_get_captions<'a>(
        &'a self,
        body: &'a types::PostGetCaptionsBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/getCaptions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_captions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the current visible (non-expired) chat messages

    Returns the current visible (non-expired) chat messages.

    Sends a `GET` request to `/rest/getChatMessages`

    */
    pub async fn get_chat_messages<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetChatMessagesResponse>, Error<()>> {
        let url = format!("{}/rest/getChatMessages", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_chat_messages",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the current visible (non-expired) chat messages

    Returns the current visible (non-expired) chat messages.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getChatMessages`

    */
    pub async fn post_get_chat_messages<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetChatMessagesResponse>, Error<()>> {
        let url = format!("{}/rest/getChatMessages", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_chat_messages",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a cover art image

    Returns a cover art image.

    Sends a `GET` request to `/rest/getCoverArt`

    Arguments:
    - `id`: The coverArt ID. Returned by most entities likes `Child` or `AlbumID3`
    - `size`: If specified, scale image to this size.
    */
    pub async fn get_cover_art<'a>(
        &'a self,
        id: &'a str,
        size: Option<i64>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/getCoverArt", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("size", &size))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_cover_art",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a cover art image

    Returns a cover art image.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getCoverArt`

    */
    pub async fn post_get_cover_art<'a>(
        &'a self,
        body: &'a types::PostGetCoverArtBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/getCoverArt", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_cover_art",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all genres

    Returns all genres.

    Sends a `GET` request to `/rest/getGenres`

    */
    pub async fn get_genres<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetGenresResponse>, Error<()>> {
        let url = format!("{}/rest/getGenres", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_genres",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all genres

    Returns all genres.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getGenres`

    */
    pub async fn post_get_genres<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetGenresResponse>, Error<()>> {
        let url = format!("{}/rest/getGenres", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_genres",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns an indexed structure of all artists

    Returns an indexed structure of all artists.

    Sends a `GET` request to `/rest/getIndexes`

    Arguments:
    - `if_modified_since`: If specified, only return a result if the artist collection has changed since the given time (in milliseconds since 1 Jan 1970).
    - `music_folder_id`: If specified, only return artists in the music folder with the given ID. See `getMusicFolders`.
    */
    pub async fn get_indexes<'a>(
        &'a self,
        if_modified_since: Option<i64>,
        music_folder_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::GetIndexesResponse>, Error<()>> {
        let url = format!("{}/rest/getIndexes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "ifModifiedSince",
                &if_modified_since,
            ))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_indexes",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns an indexed structure of all artists

    Returns an indexed structure of all artists.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getIndexes`

    */
    pub async fn post_get_indexes<'a>(
        &'a self,
        body: &'a types::PostGetIndexesBody,
    ) -> Result<ResponseValue<types::GetIndexesResponse>, Error<()>> {
        let url = format!("{}/rest/getIndexes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_indexes",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all internet radio stations

    Returns all internet radio stations. Takes no extra parameters.

    Sends a `GET` request to `/rest/getInternetRadioStations`

    */
    pub async fn get_internet_radio_stations<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetInternetRadioStationsResponse>, Error<()>> {
        let url = format!("{}/rest/getInternetRadioStations", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_internet_radio_stations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all internet radio stations

    Returns all internet radio stations. Takes no extra parameters.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getInternetRadioStations`

    */
    pub async fn post_get_internet_radio_stations<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetInternetRadioStationsResponse>, Error<()>> {
        let url = format!("{}/rest/getInternetRadioStations", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_internet_radio_stations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get details about the software license

    Get details about the software license.

    Sends a `GET` request to `/rest/getLicense`

    */
    pub async fn get_license<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetLicenseResponse>, Error<()>> {
        let url = format!("{}/rest/getLicense", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_license",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get details about the software license

    Get details about the software license.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getLicense`

    */
    pub async fn post_get_license<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetLicenseResponse>, Error<()>> {
        let url = format!("{}/rest/getLicense", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_license",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Searches for and returns lyrics for a given song

    Searches for and returns lyrics for a given song.

    Sends a `GET` request to `/rest/getLyrics`

    Arguments:
    - `artist`: The artist name.
    - `title`: The song title.
    */
    pub async fn get_lyrics<'a>(
        &'a self,
        artist: Option<&'a str>,
        title: Option<&'a str>,
    ) -> Result<ResponseValue<types::GetLyricsResponse>, Error<()>> {
        let url = format!("{}/rest/getLyrics", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("artist", &artist))
            .query(&progenitor_client::QueryParam::new("title", &title))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_lyrics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Searches for and returns lyrics for a given song

    Searches for and returns lyrics for a given song.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getLyrics`

    */
    pub async fn post_get_lyrics<'a>(
        &'a self,
        body: &'a types::PostGetLyricsBody,
    ) -> Result<ResponseValue<types::GetLyricsResponse>, Error<()>> {
        let url = format!("{}/rest/getLyrics", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_lyrics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add support for synchronized lyrics, multiple languages, and retrieval by song ID.

    OpenSubsonic extension name `songLyrics` (As returned by `getOpenSubsonicExtensions`). Retrieves all structured lyrics from the server for a given song. The lyrics can come from embedded tags (SYLT/USLT), LRC file/text file, or any other external source.

    Sends a `GET` request to `/rest/getLyricsBySongId`

    Arguments:
    - `id`: The track ID.
    */
    pub async fn get_lyrics_by_song_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetLyricsBySongIdResponse>, Error<()>> {
        let url = format!("{}/rest/getLyricsBySongId", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_lyrics_by_song_id",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add support for synchronized lyrics, multiple languages, and retrieval by song ID

    OpenSubsonic extension name `songLyrics` (As returned by `getOpenSubsonicExtensions`). Retrieves all structured lyrics from the server for a given song. The lyrics can come from embedded tags (SYLT/USLT), LRC file/text file, or any other external source.

    Sends a `POST` request to `/rest/getLyricsBySongId`

    */
    pub async fn post_get_lyrics_by_song_id<'a>(
        &'a self,
        body: &'a types::PostGetLyricsBySongIdBody,
    ) -> Result<ResponseValue<types::GetLyricsBySongIdResponse>, Error<()>> {
        let url = format!("{}/rest/getLyricsBySongId", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_lyrics_by_song_id",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of all files in a music directory

    Returns a listing of all files in a music directory. Typically used to get list of albums for an artist, or list of songs for an album.

    Sends a `GET` request to `/rest/getMusicDirectory`

    Arguments:
    - `id`: A string which uniquely identifies the music folder. Obtained by calls to `getIndexes` or `getMusicDirectory`.
    */
    pub async fn get_music_directory<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetMusicDirectoryResponse>, Error<()>> {
        let url = format!("{}/rest/getMusicDirectory", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_music_directory",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of all files in a music directory

    Returns a listing of all files in a music directory. Typically used to get list of albums for an artist, or list of songs for an album.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getMusicDirectory`

    */
    pub async fn post_get_music_directory<'a>(
        &'a self,
        body: &'a types::PostGetMusicDirectoryBody,
    ) -> Result<ResponseValue<types::GetMusicDirectoryResponse>, Error<()>> {
        let url = format!("{}/rest/getMusicDirectory", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_music_directory",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all configured top-level music folders

    Returns all configured top-level music folders. Takes no extra parameters.

    Sends a `GET` request to `/rest/getMusicFolders`

    */
    pub async fn get_music_folders<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetMusicFoldersResponse>, Error<()>> {
        let url = format!("{}/rest/getMusicFolders", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_music_folders",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all configured top-level music folders

    Returns all configured top-level music folders. Takes no extra parameters.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getMusicFolders`

    */
    pub async fn post_get_music_folders<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetMusicFoldersResponse>, Error<()>> {
        let url = format!("{}/rest/getMusicFolders", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_music_folders",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the most recently published Podcast episodes

    Returns the most recently published Podcast episodes.

    Sends a `GET` request to `/rest/getNewestPodcasts`

    Arguments:
    - `count`: The maximum number of episodes to return.
    */
    pub async fn get_newest_podcasts<'a>(
        &'a self,
        count: Option<i64>,
    ) -> Result<ResponseValue<types::GetNewestPodcastsResponse>, Error<()>> {
        let url = format!("{}/rest/getNewestPodcasts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_newest_podcasts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the most recently published Podcast episodes

    Returns the most recently published Podcast episodes.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getNewestPodcasts`

    */
    pub async fn post_get_newest_podcasts<'a>(
        &'a self,
        body: &'a types::PostGetNewestPodcastsBody,
    ) -> Result<ResponseValue<types::GetNewestPodcastsResponse>, Error<()>> {
        let url = format!("{}/rest/getNewestPodcasts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_newest_podcasts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns what is currently being played by all users

    Returns what is currently being played by all users. Takes no extra parameters.

    Sends a `GET` request to `/rest/getNowPlaying`

    */
    pub async fn get_now_playing<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetNowPlayingResponse>, Error<()>> {
        let url = format!("{}/rest/getNowPlaying", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_now_playing",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns what is currently being played by all users

    Returns what is currently being played by all users. Takes no extra parameters.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getNowPlaying`

    */
    pub async fn post_get_now_playing<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetNowPlayingResponse>, Error<()>> {
        let url = format!("{}/rest/getNowPlaying", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_now_playing",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List the OpenSubsonic extensions supported by this server

    List the OpenSubsonic extensions supported by this server.

    Sends a `GET` request to `/rest/getOpenSubsonicExtensions`

    */
    pub async fn get_open_subsonic_extensions<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetOpenSubsonicExtensionsResponse>, Error<()>> {
        let url = format!("{}/rest/getOpenSubsonicExtensions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_open_subsonic_extensions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List the OpenSubsonic extensions supported by this server

    List the OpenSubsonic extensions supported by this server.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getOpenSubsonicExtensions`

    */
    pub async fn post_get_open_subsonic_extensions<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetOpenSubsonicExtensionsResponse>, Error<()>> {
        let url = format!("{}/rest/getOpenSubsonicExtensions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_open_subsonic_extensions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of files in a saved playlist

    Returns a listing of files in a saved playlist.

    Sends a `GET` request to `/rest/getPlaylist`

    Arguments:
    - `id`: ID of the playlist to return, as obtained by `getPlaylists`.
    */
    pub async fn get_playlist<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetPlaylistResponse>, Error<()>> {
        let url = format!("{}/rest/getPlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of files in a saved playlist

    Returns a listing of files in a saved playlist.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getPlaylist`

    */
    pub async fn post_get_playlist<'a>(
        &'a self,
        body: &'a types::PostGetPlaylistBody,
    ) -> Result<ResponseValue<types::GetPlaylistResponse>, Error<()>> {
        let url = format!("{}/rest/getPlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all playlists a user is allowed to play

    Returns all playlists a user is allowed to play.

    Sends a `GET` request to `/rest/getPlaylists`

    Arguments:
    - `username`: (Since 1.8.0) If specified, return playlists for this user rather than for the authenticated user. The authenticated user must have admin role if this parameter is used.
    */
    pub async fn get_playlists<'a>(
        &'a self,
        username: Option<&'a str>,
    ) -> Result<ResponseValue<types::GetPlaylistsResponse>, Error<()>> {
        let url = format!("{}/rest/getPlaylists", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("username", &username))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_playlists",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all playlists a user is allowed to play

    Returns all playlists a user is allowed to play.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getPlaylists`

    */
    pub async fn post_get_playlists<'a>(
        &'a self,
        body: &'a types::PostGetPlaylistsBody,
    ) -> Result<ResponseValue<types::GetPlaylistsResponse>, Error<()>> {
        let url = format!("{}/rest/getPlaylists", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_playlists",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the state of the play queue for this user

    Returns the state of the play queue for this user (as set by savePlayQueue). This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).

    Sends a `GET` request to `/rest/getPlayQueue`

    */
    pub async fn get_play_queue<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetPlayQueueResponse>, Error<()>> {
        let url = format!("{}/rest/getPlayQueue", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_play_queue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the state of the play queue for this user

    Returns the state of the play queue for this user (as set by savePlayQueue). This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getPlayQueue`

    */
    pub async fn post_get_play_queue<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetPlayQueueResponse>, Error<()>> {
        let url = format!("{}/rest/getPlayQueue", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_play_queue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the state of the play queue for this user, using queue index

    Returns the state of the play queue for this user (as set by savePlayQueue). This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).

    Sends a `GET` request to `/rest/getPlayQueueByIndex`

    */
    pub async fn get_play_queue_by_index<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetPlayQueueByIndexResponse>, Error<()>> {
        let url = format!("{}/rest/getPlayQueueByIndex", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_play_queue_by_index",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the state of the play queue for this user

    Returns the state of the play queue for this user (as set by savePlayQueue). This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getPlayQueueByIndex`

    */
    pub async fn post_get_play_queue_by_index<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetPlayQueueByIndexResponse>, Error<()>> {
        let url = format!("{}/rest/getPlayQueueByIndex", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_play_queue_by_index",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for a podcast episode

    OpenSubsonic extension name getPodcastEpisode (As returned by `getOpenSubsonicExtensions`). Returns details for a podcast episode.

    Sends a `GET` request to `/rest/getPodcastEpisode`

    Arguments:
    - `id`: The podcast episode ID.
    */
    pub async fn get_podcast_episode<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetPodcastEpisodeResponse>, Error<()>> {
        let url = format!("{}/rest/getPodcastEpisode", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_podcast_episode",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for a podcast episode

    OpenSubsonic extension name `getPodcastEpisode` (As returned by `getOpenSubsonicExtensions`). Returns details for a podcast episode.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getPodcastEpisode`

    */
    pub async fn post_get_podcast_episode<'a>(
        &'a self,
        body: &'a types::PostGetPodcastEpisodeBody,
    ) -> Result<ResponseValue<types::GetPodcastEpisodeResponse>, Error<()>> {
        let url = format!("{}/rest/getPodcastEpisode", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_podcast_episode",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all Podcast channels the server subscribes to, and (optionally) their episodes

    Returns all Podcast channels the server subscribes to, and (optionally) their episodes. This method can also be used to return details for only one channel - refer to the id parameter. A typical use case for this method would be to first retrieve all channels without episodes, and then retrieve all episodes for the single channel the user selects.

    Sends a `GET` request to `/rest/getPodcasts`

    Arguments:
    - `id`: (Since 1.9.0) If specified, only return the Podcast channel with this ID.
    - `include_episodes`: (Since 1.9.0) Whether to include Podcast episodes in the returned result.
    */
    pub async fn get_podcasts<'a>(
        &'a self,
        id: Option<&'a str>,
        include_episodes: Option<bool>,
    ) -> Result<ResponseValue<types::GetPodcastsResponse>, Error<()>> {
        let url = format!("{}/rest/getPodcasts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new(
                "includeEpisodes",
                &include_episodes,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_podcasts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all Podcast channels the server subscribes to, and (optionally) their episodes

    Returns all Podcast channels the server subscribes to, and (optionally) their episodes. This method can also be used to return details for only one channel - refer to the id parameter. A typical use case for this method would be to first retrieve all channels without episodes, and then retrieve all episodes for the single channel the user selects.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getPodcasts`

    */
    pub async fn post_get_podcasts<'a>(
        &'a self,
        body: &'a types::PostGetPodcastsBody,
    ) -> Result<ResponseValue<types::GetPodcastsResponse>, Error<()>> {
        let url = format!("{}/rest/getPodcasts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_podcasts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns random songs matching the given criteria

    Returns random songs matching the given criteria.

    Sends a `GET` request to `/rest/getRandomSongs`

    Arguments:
    - `from_year`: (Since 1.9.0) Only return songs from this year or later.
    - `genre`: Only returns songs belonging to this genre.
    - `music_folder_id`: Only return songs in the music folder with the given ID. See `getMusicFolders`.
    - `size`: The maximum number of songs to return. Max 500.
    - `to_year`: Only return songs published before or in this year.
    */
    pub async fn get_random_songs<'a>(
        &'a self,
        from_year: Option<i64>,
        genre: Option<&'a str>,
        music_folder_id: Option<&'a str>,
        size: Option<i64>,
        to_year: Option<i64>,
    ) -> Result<ResponseValue<types::GetRandomSongsResponse>, Error<()>> {
        let url = format!("{}/rest/getRandomSongs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("fromYear", &from_year))
            .query(&progenitor_client::QueryParam::new("genre", &genre))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("size", &size))
            .query(&progenitor_client::QueryParam::new("toYear", &to_year))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_random_songs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns random songs matching the given criteria

    Returns random songs matching the given criteria.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getRandomSongs`

    */
    pub async fn post_get_random_songs<'a>(
        &'a self,
        body: &'a types::PostGetRandomSongsBody,
    ) -> Result<ResponseValue<types::GetRandomSongsResponse>, Error<()>> {
        let url = format!("{}/rest/getRandomSongs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_random_songs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the current status for media library scanning

    Returns the current status for media library scanning. Takes no extra parameters.

    Sends a `GET` request to `/rest/getScanStatus`

    */
    pub async fn get_scan_status<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetScanStatusResponse>, Error<()>> {
        let url = format!("{}/rest/getScanStatus", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_scan_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the current status for media library scanning

    Returns the current status for media library scanning. Takes no extra parameters.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getScanStatus`

    */
    pub async fn post_get_scan_status<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetScanStatusResponse>, Error<()>> {
        let url = format!("{}/rest/getScanStatus", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_scan_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns information about shared media this user is allowed to manage

    Returns information about shared media this user is allowed to manage. Takes no extra parameters.

    Sends a `GET` request to `/rest/getShares`

    */
    pub async fn get_shares<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetSharesResponse>, Error<()>> {
        let url = format!("{}/rest/getShares", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_shares",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns information about shared media this user is allowed to manage

    Returns information about shared media this user is allowed to manage. Takes no extra parameters.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getShares`

    */
    pub async fn post_get_shares<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetSharesResponse>, Error<()>> {
        let url = format!("{}/rest/getShares", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_shares",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a random collection of songs from the given artist and similar artists

    Returns a random collection of songs from the given artist and similar artists, using data from last.fm. Typically used for artist radio features.

    Sends a `GET` request to `/rest/getSimilarSongs`

    Arguments:
    - `count`: Max number of songs to return.
    - `id`: The artist, album or song ID.
    */
    pub async fn get_similar_songs<'a>(
        &'a self,
        count: Option<u64>,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetSimilarSongsResponse>, Error<()>> {
        let url = format!("{}/rest/getSimilarSongs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_similar_songs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a random collection of songs from the given artist and similar artists

    Returns a random collection of songs from the given artist and similar artists, using data from last.fm. Typically used for artist radio features.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getSimilarSongs`

    */
    pub async fn post_get_similar_songs<'a>(
        &'a self,
        body: &'a types::PostGetSimilarSongsBody,
    ) -> Result<ResponseValue<types::GetSimilarSongsResponse>, Error<()>> {
        let url = format!("{}/rest/getSimilarSongs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_similar_songs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a random collection of songs from the given artist and similar artists (v2)

    Similar to `getSimilarSongs`, but organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getSimilarSongs2`

    Arguments:
    - `count`: Max number of songs to return.
    - `id`: The artist, album or song ID.
    */
    pub async fn get_similar_songs2<'a>(
        &'a self,
        count: Option<u64>,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetSimilarSongs2Response>, Error<()>> {
        let url = format!("{}/rest/getSimilarSongs2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_similar_songs2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a random collection of songs from the given artist and similar artists (v2)

    Similar to `getSimilarSongs`, but organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getSimilarSongs2`

    */
    pub async fn post_get_similar_songs2<'a>(
        &'a self,
        body: &'a types::PostGetSimilarSongs2Body,
    ) -> Result<ResponseValue<types::GetSimilarSongs2Response>, Error<()>> {
        let url = format!("{}/rest/getSimilarSongs2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_similar_songs2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for a song

    Returns details for a song.

    Sends a `GET` request to `/rest/getSong`

    Arguments:
    - `id`: The song ID.
    */
    pub async fn get_song<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetSongResponse>, Error<()>> {
        let url = format!("{}/rest/getSong", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_song",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for a song

    Returns details for a song.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getSong`

    */
    pub async fn post_get_song<'a>(
        &'a self,
        body: &'a types::PostGetSongBody,
    ) -> Result<ResponseValue<types::GetSongResponse>, Error<()>> {
        let url = format!("{}/rest/getSong", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_song",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns songs in a given genre

    Returns songs in a given genre.

    Sends a `GET` request to `/rest/getSongsByGenre`

    Arguments:
    - `count`: The maximum number of songs to return. Max 500.
    - `genre`: The genre, as returned by `getGenres`.
    - `music_folder_id`: (Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    - `offset`: The offset. Useful if you want to page through the songs in a genre.
    */
    pub async fn get_songs_by_genre<'a>(
        &'a self,
        count: Option<i64>,
        genre: &'a str,
        music_folder_id: Option<&'a str>,
        offset: Option<u64>,
    ) -> Result<ResponseValue<types::GetSongsByGenreResponse>, Error<()>> {
        let url = format!("{}/rest/getSongsByGenre", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("genre", &genre))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_songs_by_genre",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns songs in a given genre

    Returns songs in a given genre.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getSongsByGenre`

    */
    pub async fn post_get_songs_by_genre<'a>(
        &'a self,
        body: &'a types::PostGetSongsByGenreBody,
    ) -> Result<ResponseValue<types::GetSongsByGenreResponse>, Error<()>> {
        let url = format!("{}/rest/getSongsByGenre", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_songs_by_genre",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns starred songs, albums and artists

    Returns starred songs, albums and artists.

    Sends a `GET` request to `/rest/getStarred`

    Arguments:
    - `music_folder_id`: (Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    */
    pub async fn get_starred<'a>(
        &'a self,
        music_folder_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::GetStarredResponse>, Error<()>> {
        let url = format!("{}/rest/getStarred", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_starred",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns starred songs, albums and artists

    Returns starred songs, albums and artists.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getStarred`

    */
    pub async fn post_get_starred<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetStarredResponse>, Error<()>> {
        let url = format!("{}/rest/getStarred", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_starred",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns starred songs, albums and artists

    Similar to `getStarred`, but organizes music according to ID3 tags.

    Sends a `GET` request to `/rest/getStarred2`

    Arguments:
    - `music_folder_id`: (Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    */
    pub async fn get_starred2<'a>(
        &'a self,
        music_folder_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::GetStarred2Response>, Error<()>> {
        let url = format!("{}/rest/getStarred2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_starred2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns starred songs, albums and artists

    Similar to `getStarred`, but organizes music according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getStarred2`

    */
    pub async fn post_get_starred2<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetStarred2Response>, Error<()>> {
        let url = format!("{}/rest/getStarred2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_starred2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns top songs for the given artist

    Returns top songs for the given artist, using data from last.fm.

    Sends a `GET` request to `/rest/getTopSongs`

    Arguments:
    - `count`: The maximum number of songs to return.
    - `id`: The artist name.
    */
    pub async fn get_top_songs<'a>(
        &'a self,
        count: Option<u64>,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetTopSongsResponse>, Error<()>> {
        let url = format!("{}/rest/getTopSongs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_top_songs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns top songs for the given artist

    Returns top songs for the given artist, using data from last.fm.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getTopSongs`

    */
    pub async fn post_get_top_songs<'a>(
        &'a self,
        body: &'a types::PostGetTopSongsBody,
    ) -> Result<ResponseValue<types::GetTopSongsResponse>, Error<()>> {
        let url = format!("{}/rest/getTopSongs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_top_songs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get details about a given user, including which authorization roles and folder access it has

    Get details about a given user, including which authorization roles and folder access it has. Can be used to enable/disable certain features in the client, such as jukebox control.

    Sends a `GET` request to `/rest/getUser`

    Arguments:
    - `username`: The name of the user to retrieve. You can only retrieve your own user unless you have admin privileges.
    */
    pub async fn get_user<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<ResponseValue<types::GetUserResponse>, Error<()>> {
        let url = format!("{}/rest/getUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("username", &username))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get details about a given user, including which authorization roles and folder access it has

    Get details about a given user, including which authorization roles and folder access it has. Can be used to enable/disable certain features in the client, such as jukebox control.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getUser`

    */
    pub async fn post_get_user<'a>(
        &'a self,
        body: &'a types::PostGetUserBody,
    ) -> Result<ResponseValue<types::GetUserResponse>, Error<()>> {
        let url = format!("{}/rest/getUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get details about all users, including which authorization roles and folder access they have

    Get details about all users, including which authorization roles and folder access they have. Only users with admin privileges are allowed to call this method.

    Sends a `GET` request to `/rest/getUsers`

    */
    pub async fn get_users<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetUsersResponse>, Error<()>> {
        let url = format!("{}/rest/getUsers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_users",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get details about all users, including which authorization roles and folder access they have

    Get details about all users, including which authorization roles and folder access they have. Only users with admin privileges are allowed to call this method.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getUsers`

    */
    pub async fn post_get_users<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetUsersResponse>, Error<()>> {
        let url = format!("{}/rest/getUsers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_users",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for a video

    Returns details for a video, including information about available audio tracks, subtitles (captions) and conversions.

    Sends a `GET` request to `/rest/getVideoInfo`

    Arguments:
    - `id`: The video ID.
    */
    pub async fn get_video_info<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::GetVideoInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getVideoInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_video_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns details for a video

    Returns details for a video, including information about available audio tracks, subtitles (captions) and conversions.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getVideoInfo`

    */
    pub async fn post_get_video_info<'a>(
        &'a self,
        body: &'a types::PostGetVideoInfoBody,
    ) -> Result<ResponseValue<types::GetVideoInfoResponse>, Error<()>> {
        let url = format!("{}/rest/getVideoInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_video_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all video files

    Returns all video files.

    Sends a `GET` request to `/rest/getVideos`

    */
    pub async fn get_videos<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetVideosResponse>, Error<()>> {
        let url = format!("{}/rest/getVideos", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_videos",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns all video files

    Returns all video files.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/getVideos`

    */
    pub async fn post_get_videos<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetVideosResponse>, Error<()>> {
        let url = format!("{}/rest/getVideos", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_get_videos",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Downloads a given media file (HLS)

    Creates an HLS (HTTP Live Streaming) playlist used for streaming video or audio. HLS is a streaming protocol implemented by Apple and works by breaking the overall stream into a sequence of small HTTP-based file downloads. It’s supported by iOS and newer versions of Android. This method also supports adaptive bitrate streaming, see the bitRate parameter.

    Sends a `GET` request to `/rest/hls.m3u8`

    Arguments:
    - `audio_track`: The ID of the audio track to use. See `getVideoInfo` for how to get the list of available audio tracks for a video.
    - `bit_rate`: If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If this parameter is specified more than once, the server will create a variant playlist, suitable for adaptive bitrate streaming. The playlist will support streaming at all the specified bitrates. The server will automatically choose video dimensions that are suitable for the given bitrates. Since 1.9.0 you may explicitly request a certain width (480) and height (360) like so: bitRate=1000@480x360
    - `id`: A string which uniquely identifies the media file to stream.
    */
    pub async fn hls_m3u8<'a>(
        &'a self,
        audio_track: Option<&'a str>,
        bit_rate: Option<i64>,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/hls.m3u8", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new(
                "audioTrack",
                &audio_track,
            ))
            .query(&progenitor_client::QueryParam::new("bitRate", &bit_rate))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "hls_m3u8",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Downloads a given media file (HLS)

    Creates an HLS (HTTP Live Streaming) playlist used for streaming video or audio. HLS is a streaming protocol implemented by Apple and works by breaking the overall stream into a sequence of small HTTP-based file downloads. It’s supported by iOS and newer versions of Android. This method also supports adaptive bitrate streaming, see the bitRate parameter.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/hls.m3u8`

    */
    pub async fn post_hls_m3u8<'a>(
        &'a self,
        body: &'a types::PostHlsM3u8Body,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/hls.m3u8", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_hls_m3u8",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Controls the jukebox, i.e., playback directly on the server’s audio hardware

    Controls the jukebox, i.e., playback directly on the server’s audio hardware. Note: The user must be authorized to control the jukebox (see Settings > Users > User is allowed to play files in jukebox mode).

    Sends a `GET` request to `/rest/jukeboxControl`

    Arguments:
    - `action`: The operation to perform. Must be one of: get, status (since 1.7.0), set (since 1.7.0), start, stop, skip, add, clear, remove, shuffle, setGain
    - `gain`: Used by `setGain` to control the playback volume. A float value between 0.0 and 1.0.
    - `id`: Used by `add` and `set`. ID of song to add to the jukebox playlist. Use multiple id parameters to add many songs in the same request. (set is similar to a clear followed by a add, but will not change the currently playing track.)
    - `index`: Used by `skip` and `remove`. Zero-based index of the song to skip to or remove.
    - `offset`: (Since 1.7.0) Used by `skip`. Start playing this many seconds into the track.
    */
    pub async fn jukebox_control<'a>(
        &'a self,
        action: types::JukeboxAction,
        gain: Option<f32>,
        id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        index: Option<u64>,
        offset: Option<u64>,
    ) -> Result<ResponseValue<types::JukeboxControlResponse>, Error<()>> {
        let url = format!("{}/rest/jukeboxControl", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("action", &action))
            .query(&progenitor_client::QueryParam::new("gain", &gain))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("index", &index))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "jukebox_control",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Controls the jukebox, i.e., playback directly on the server’s audio hardware

    Controls the jukebox, i.e., playback directly on the server’s audio hardware. Note: The user must be authorized to control the jukebox (see Settings > Users > User is allowed to play files in jukebox mode).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/jukeboxControl`

    */
    pub async fn post_jukebox_control<'a>(
        &'a self,
        body: &'a types::PostJukeboxControlBody,
    ) -> Result<ResponseValue<types::JukeboxControlResponse>, Error<()>> {
        let url = format!("{}/rest/jukeboxControl", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_jukebox_control",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Used to test connectivity with the server

    Test connectivity with the server.

    Sends a `GET` request to `/rest/ping`

    */
    pub async fn ping<'a>(&'a self) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/ping", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Used to test connectivity with the server

    Test connectivity with the server.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/ping`

    */
    pub async fn post_ping<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/ping", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_ping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Requests the server to check for new Podcast episodes

    Requests the server to check for new Podcast episodes. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Sends a `GET` request to `/rest/refreshPodcasts`

    */
    pub async fn refresh_podcasts<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/refreshPodcasts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "refresh_podcasts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Requests the server to check for new Podcast episodes

    Requests the server to check for new Podcast episodes. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/refreshPodcasts`

    */
    pub async fn post_refresh_podcasts<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/refreshPodcasts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_refresh_podcasts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Saves the state of the play queue for this user

    Saves the state of the play queue for this user. This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book). `id` is optional. Send a call without any parameters to clear the currently saved queue.

    Sends a `GET` request to `/rest/savePlayQueue`

    Arguments:
    - `current`: The ID of the current playing song.  This is required if one or more IDs is provided.
    - `id`: ID of a song in the play queue. Use one id parameter for each song in the play queue. Specify no IDs to clear
    - `position`: The position in milliseconds within the currently playing song.
    */
    pub async fn save_play_queue<'a>(
        &'a self,
        current: Option<&'a str>,
        id: Option<&'a str>,
        position: Option<u64>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/savePlayQueue", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("current", &current))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("position", &position))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "save_play_queue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Saves the state of the play queue for this user

    Saves the state of the play queue for this user. This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book). `id` is optional. Send a call without any parameters to clear the currently saved queue.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/savePlayQueue`

    */
    pub async fn post_save_play_queue<'a>(
        &'a self,
        body: &'a types::PostSavePlayQueueBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/savePlayQueue", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_save_play_queue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Saves the state of the play queue for this user, using queue index

    Saves the state of the play queue for this user. This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book). `id` is optional. Send a call without any parameters to clear the currently saved queue.

    Sends a `GET` request to `/rest/savePlayQueueByIndex`

    Arguments:
    - `current_index`: The index of the current playing song. This is required if one or more IDs is provided.
    - `id`: ID of a song in the play queue. Use one id parameter for each song in the play queue. Specify no IDs to clear
    - `position`: The position in milliseconds within the currently playing song.
    */
    pub async fn save_play_queue_by_index<'a>(
        &'a self,
        current_index: Option<u64>,
        id: Option<&'a str>,
        position: Option<u64>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/savePlayQueueByIndex", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "currentIndex",
                &current_index,
            ))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("position", &position))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "save_play_queue_by_index",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Saves the state of the play queue for this user

    Saves the state of the play queue for this user. This includes the tracks in the play queue, the currently playing track, and the position within this track. Typically used to allow a user to move between different clients/apps while retaining the same play queue (for instance when listening to an audio book). `id` is optional. Send a call without any parameters to clear the currently saved queue.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/savePlayQueueByIndex`

    */
    pub async fn post_save_play_queue_by_index<'a>(
        &'a self,
        body: &'a types::PostSavePlayQueueByIndexBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/savePlayQueueByIndex", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_save_play_queue_by_index",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Registers the local playback of one or more media files

    Registers the local playback of one or more media files. Typically used when playing media that is cached on the client. This operation includes the following:

    * “Scrobbles” the media files on last.fm if the user has configured his/her last.fm credentials on the server.
    * Updates the play count and last played timestamp for the media files. (Since 1.11.0)
    * Makes the media files appear in the “Now playing” page in the web app, and appear in the list of songs returned by getNowPlaying (Since 1.11.0)

    Since 1.8.0 you may specify multiple id (and optionally time) parameters to scrobble multiple files.

    Sends a `GET` request to `/rest/scrobble`

    Arguments:
    - `id`: A string which uniquely identifies the file to scrobble.
    - `submission`: Whether this is a “submission” or a “now playing” notification.
    - `time`: (Since 1.8.0) The time (in milliseconds since 1 Jan 1970) at which the song was listened to.
    */
    pub async fn scrobble<'a>(
        &'a self,
        id: &'a str,
        submission: Option<bool>,
        time: Option<u64>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/scrobble", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new(
                "submission",
                &submission,
            ))
            .query(&progenitor_client::QueryParam::new("time", &time))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "scrobble",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Registers the local playback of one or more media files

    Registers the local playback of one or more media files. Typically used when playing media that is cached on the client. This operation includes the following:

    * “Scrobbles” the media files on last.fm if the user has configured his/her last.fm credentials on the server.
    * Updates the play count and last played timestamp for the media files. (Since 1.11.0)
    * Makes the media files appear in the “Now playing” page in the web app, and appear in the list of songs returned by getNowPlaying (Since 1.11.0)

    Since 1.8.0 you may specify multiple id (and optionally time) parameters to scrobble multiple files.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/scrobble`

    */
    pub async fn post_scrobble<'a>(
        &'a self,
        body: &'a types::PostScrobbleBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/scrobble", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_scrobble",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of files matching the given search criteria. Supports paging through the result

    Deprecated since 1.4.0, use search2 instead.

    Returns a listing of files matching the given search criteria. Supports paging through the result.

    Sends a `GET` request to `/rest/search`

    Arguments:
    - `album`: Album to search for.
    - `any`: Searches all fields.
    - `artist`: Artist to search for.
    - `count`: Maximum number of results to return.
    - `newer_than`: Only return matches that are newer than this. Given as milliseconds since 1970.
    - `offset`: Search result offset. Used for paging.
    - `title`: Song title to search for.
    */
    pub async fn search<'a>(
        &'a self,
        album: Option<&'a str>,
        any: Option<bool>,
        artist: Option<&'a str>,
        count: Option<u64>,
        newer_than: Option<u64>,
        offset: Option<u64>,
        title: Option<&'a str>,
    ) -> Result<ResponseValue<types::SearchResponse>, Error<()>> {
        let url = format!("{}/rest/search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("album", &album))
            .query(&progenitor_client::QueryParam::new("any", &any))
            .query(&progenitor_client::QueryParam::new("artist", &artist))
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new(
                "newerThan",
                &newer_than,
            ))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("title", &title))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of files matching the given search criteria. Supports paging through the result

    Deprecated since 1.4.0, use search2 instead.

    Returns a listing of files matching the given search criteria. Supports paging through the result.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/search`

    */
    pub async fn post_search<'a>(
        &'a self,
        body: &'a types::PostSearchBody,
    ) -> Result<ResponseValue<types::SearchResponse>, Error<()>> {
        let url = format!("{}/rest/search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_search",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of files matching the given search criteria. Supports paging through the result. (v2)

    Returns albums, artists and songs matching the given search criteria. Supports paging through the result.

    Sends a `GET` request to `/rest/search2`

    Arguments:
    - `album_count`: Maximum number of albums to return.
    - `album_offset`: Search result offset for albums. Used for paging.
    - `artist_count`: Maximum number of artists to return.
    - `artist_offset`: Search result offset for artists. Used for paging.
    - `music_folder_id`: (Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    - `query`: Search query.
    - `song_count`: Maximum number of songs to return.
    - `song_offset`: Search result offset for songs. Used for paging.
    */
    pub async fn search2<'a>(
        &'a self,
        album_count: Option<u64>,
        album_offset: Option<u64>,
        artist_count: Option<u64>,
        artist_offset: Option<u64>,
        music_folder_id: Option<&'a str>,
        query: &'a str,
        song_count: Option<u64>,
        song_offset: Option<u64>,
    ) -> Result<ResponseValue<types::Search2Response>, Error<()>> {
        let url = format!("{}/rest/search2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "albumCount",
                &album_count,
            ))
            .query(&progenitor_client::QueryParam::new(
                "albumOffset",
                &album_offset,
            ))
            .query(&progenitor_client::QueryParam::new(
                "artistCount",
                &artist_count,
            ))
            .query(&progenitor_client::QueryParam::new(
                "artistOffset",
                &artist_offset,
            ))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("query", &query))
            .query(&progenitor_client::QueryParam::new(
                "songCount",
                &song_count,
            ))
            .query(&progenitor_client::QueryParam::new(
                "songOffset",
                &song_offset,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns a listing of files matching the given search criteria. Supports paging through the result. (v2)

    Returns albums, artists and songs matching the given search criteria. Supports paging through the result.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/search2`

    */
    pub async fn post_search2<'a>(
        &'a self,
        body: &'a types::PostSearch2Body,
    ) -> Result<ResponseValue<types::Search2Response>, Error<()>> {
        let url = format!("{}/rest/search2", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_search2",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns albums, artists and songs matching the given search criteria. Supports paging through the result. (v3)

    Returns albums, artists and songs matching the given search criteria. Supports paging through the result.

    Music is organized according to ID3 tags.

    Sends a `GET` request to `/rest/search3`

    Arguments:
    - `album_count`: Maximum number of albums to return.
    - `album_offset`: Search result offset for albums. Used for paging.
    - `artist_count`: Maximum number of artists to return.
    - `artist_offset`: Search result offset for artists. Used for paging.
    - `music_folder_id`: (Since 1.12.0) Only return albums in the music folder with the given ID. See `getMusicFolders`.
    - `query`: Search query. Servers must support an empty query and return all the data to allow clients to properly access all the media information for offline sync.
    - `song_count`: Maximum number of songs to return.
    - `song_offset`: Search result offset for songs. Used for paging.
    */
    pub async fn search3<'a>(
        &'a self,
        album_count: Option<u64>,
        album_offset: Option<u64>,
        artist_count: Option<u64>,
        artist_offset: Option<u64>,
        music_folder_id: Option<&'a str>,
        query: &'a str,
        song_count: Option<u64>,
        song_offset: Option<u64>,
    ) -> Result<ResponseValue<types::Search3Response>, Error<()>> {
        let url = format!("{}/rest/search3", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "albumCount",
                &album_count,
            ))
            .query(&progenitor_client::QueryParam::new(
                "albumOffset",
                &album_offset,
            ))
            .query(&progenitor_client::QueryParam::new(
                "artistCount",
                &artist_count,
            ))
            .query(&progenitor_client::QueryParam::new(
                "artistOffset",
                &artist_offset,
            ))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("query", &query))
            .query(&progenitor_client::QueryParam::new(
                "songCount",
                &song_count,
            ))
            .query(&progenitor_client::QueryParam::new(
                "songOffset",
                &song_offset,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search3",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns albums, artists and songs matching the given search criteria. Supports paging through the result. (v3)

    Returns albums, artists and songs matching the given search criteria. Supports paging through the result.

    Music is organized according to ID3 tags.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/search3`

    */
    pub async fn post_search3<'a>(
        &'a self,
        body: &'a types::PostSearch3Body,
    ) -> Result<ResponseValue<types::Search3Response>, Error<()>> {
        let url = format!("{}/rest/search3", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_search3",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Sets the rating for a music file

    Sets the rating for a music file.

    Sends a `GET` request to `/rest/setRating`

    Arguments:
    - `id`: A string which uniquely identifies the file (song) or folder (album/artist) to rate.
    - `rating`: The rating between 1 and 5 (inclusive), or 0 to remove the rating.
    */
    pub async fn set_rating<'a>(
        &'a self,
        id: &'a str,
        rating: i64,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/setRating", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("rating", &rating))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "set_rating",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Sets the rating for a music file

    Sets the rating for a music file.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/setRating`

    */
    pub async fn post_set_rating<'a>(
        &'a self,
        body: &'a types::PostSetRatingBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/setRating", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_set_rating",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Attaches a star to a song, album or artist

    Attaches a star to a song, album or artist.

    Sends a `GET` request to `/rest/star`

    Arguments:
    - `album_id`: The ID of an album to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
    - `artist_id`: The ID of an artist to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
    - `id`: The ID of the file (song) or folder (album/artist) to star. Multiple parameters allowed.
    */
    pub async fn star<'a>(
        &'a self,
        album_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        artist_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        id: Option<&'a ::std::vec::Vec<::std::string::String>>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/star", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("albumId", &album_id))
            .query(&progenitor_client::QueryParam::new("artistId", &artist_id))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "star",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Attaches a star to a song, album or artist

    Attaches a star to a song, album or artist.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/star`

    */
    pub async fn post_star<'a>(
        &'a self,
        body: &'a types::PostStarBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/star", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_star",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Initiates a rescan of the media libraries

    Initiates a rescan of the media libraries. Takes no extra parameters.

    Sends a `GET` request to `/rest/startScan`

    */
    pub async fn start_scan<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::StartScanResponse>, Error<()>> {
        let url = format!("{}/rest/startScan", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "start_scan",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Initiates a rescan of the media libraries

    Initiates a rescan of the media libraries. Takes no extra parameters.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/startScan`

    */
    pub async fn post_start_scan<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::StartScanResponse>, Error<()>> {
        let url = format!("{}/rest/startScan", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_start_scan",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Streams a given media file

    Streams a given media file.

    OpenSubsonic servers must not count access to this endpoint as a play and increase playcount. Clients can use the Scrobble endpoint to indicate that a media is played ensuring proper data in all cases.

    If the server support the Transcode Offet extension, then it must accept the timeOffset parameter for music too.

    Sends a `GET` request to `/rest/stream`

    Arguments:
    - `converted`: (Since 1.14.0) Only applicable to video streaming. Servers can optimize videos for streaming by converting them to MP4. If a conversion exists for the video in question, then setting this parameter to “true” will cause the converted video to be returned instead of the original.
    - `estimate_content_length`: (Since 1.8.0). If set to “true”, the Content-Length HTTP header will be set to an estimated value for transcoded or downsampled media.
    - `format`: (Since 1.6.0) Specifies the preferred target format (e.g., “mp3” or “flv”) in case there are multiple applicable transcodings. Starting with 1.9.0 you can use the special value “raw” to disable transcoding.
    - `id`: A string which uniquely identifies the file to stream. Obtained by calls to getMusicDirectory.
    - `max_bit_rate`: (Since 1.2.0) If specified, the server will attempt to limit the bitrate to this value, in kilobits per second. If set to zero, no limit is imposed.
    - `size`: (Since 1.6.0) Only applicable to video streaming. Requested video size specified as WxH, for instance “640x480”.
    - `time_offset`: By default only applicable to video streaming. If specified, start streaming at the given offset (in seconds) into the media. The `Transcode Offset` extension enables the parameter to music too.
    */
    pub async fn stream<'a>(
        &'a self,
        converted: Option<bool>,
        estimate_content_length: Option<bool>,
        format: Option<&'a str>,
        id: &'a str,
        max_bit_rate: Option<u64>,
        size: Option<&'a types::StreamSize>,
        time_offset: Option<u64>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/stream", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("converted", &converted))
            .query(&progenitor_client::QueryParam::new(
                "estimateContentLength",
                &estimate_content_length,
            ))
            .query(&progenitor_client::QueryParam::new("format", &format))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new(
                "maxBitRate",
                &max_bit_rate,
            ))
            .query(&progenitor_client::QueryParam::new("size", &size))
            .query(&progenitor_client::QueryParam::new(
                "timeOffset",
                &time_offset,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "stream",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Streams a given media file

    Streams a given media file.

    OpenSubsonic servers must not count access to this endpoint as a play and increase playcount. Clients can use the Scrobble endpoint to indicate that a media is played ensuring proper data in all cases.

    If the server supports the Transcode Offset extension, then it must accept the timeOffset parameter for music too.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/stream`

    */
    pub async fn post_stream<'a>(
        &'a self,
        body: &'a types::PostStreamBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/rest/stream", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_stream",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns information about an API key

    OpenSubsonic extension name `apiKeyAuthentication` (As returned by `getOpenSubsonicExtensions`). Returns data about an API key.

    Sends a `GET` request to `/rest/tokenInfo`

    */
    pub async fn token_info<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetTokenInfoResponse>, Error<()>> {
        let url = format!("{}/rest/tokenInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "token_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns information about an API key

    OpenSubsonic extension name `apiKeyAuthentication` (As returned by `getOpenSubsonicExtensions`). Returns data about an API key.

    Sends a `POST` request to `/rest/tokenInfo`

    */
    pub async fn post_token_info<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetTokenInfoResponse>, Error<()>> {
        let url = format!("{}/rest/tokenInfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_token_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Removes a star to a song, album or artist

    Removes a star to a song, album or artist.

    Sends a `GET` request to `/rest/unstar`

    Arguments:
    - `album_id`: The ID of an album to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
    - `artist_id`: The ID of an artist to star. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
    - `id`: The ID of the file (song) or folder (album/artist) to star. Multiple parameters allowed.
    */
    pub async fn unstar<'a>(
        &'a self,
        album_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        artist_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        id: Option<&'a ::std::vec::Vec<::std::string::String>>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/unstar", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("albumId", &album_id))
            .query(&progenitor_client::QueryParam::new("artistId", &artist_id))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "unstar",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Removes a star to a song, album or artist

    Removes a star to a song, album or artist.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/unstar`

    */
    pub async fn post_unstar<'a>(
        &'a self,
        body: &'a types::PostUnstarBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/unstar", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_unstar",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates an existing internet radio station

    Updates an existing internet radio station. Only users with admin privileges are allowed to call this method.

    Sends a `GET` request to `/rest/updateInternetRadioStation`

    Arguments:
    - `homepage_url`: The home page URL for the station.
    - `id`: The ID of the station.
    - `name`: The user-defined name for the station.
    - `stream_url`: The stream URL for the station.
    */
    pub async fn update_internet_radio_station<'a>(
        &'a self,
        homepage_url: Option<&'a str>,
        id: &'a str,
        name: &'a str,
        stream_url: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updateInternetRadioStation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "homepageUrl",
                &homepage_url,
            ))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new(
                "streamUrl",
                &stream_url,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_internet_radio_station",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates an existing internet radio station

    Updates an existing internet radio station. Only users with admin privileges are allowed to call this method.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/updateInternetRadioStation`

    */
    pub async fn post_update_internet_radio_station<'a>(
        &'a self,
        body: &'a types::PostUpdateInternetRadioStationBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updateInternetRadioStation", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_update_internet_radio_station",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates a playlist. Only the owner of a playlist is allowed to update it

    Updates a playlist. Only the owner of a playlist is allowed to update it.

    Sends a `GET` request to `/rest/updatePlaylist`

    Arguments:
    - `comment`: The playlist comment.
    - `name`: The human-readable name of the playlist.
    - `playlist_id`: The playlist ID.
    - `public`: `true` if the playlist should be visible to all users, `false` otherwise.
    - `song_id_to_add`: Add this song with this ID to the playlist. Multiple parameters allowed.
    - `song_index_to_remove`: Remove the song at this position in the playlist. Multiple parameters allowed.
    */
    pub async fn update_playlist<'a>(
        &'a self,
        comment: Option<&'a str>,
        name: Option<&'a str>,
        playlist_id: &'a str,
        public: Option<bool>,
        song_id_to_add: Option<&'a ::std::vec::Vec<::std::string::String>>,
        song_index_to_remove: Option<&'a ::std::vec::Vec<i64>>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updatePlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("comment", &comment))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new(
                "playlistId",
                &playlist_id,
            ))
            .query(&progenitor_client::QueryParam::new("public", &public))
            .query(&progenitor_client::QueryParam::new(
                "songIdToAdd",
                &song_id_to_add,
            ))
            .query(&progenitor_client::QueryParam::new(
                "songIndexToRemove",
                &song_index_to_remove,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates a playlist. Only the owner of a playlist is allowed to update it

    Updates a playlist. Only the owner of a playlist is allowed to update it.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/updatePlaylist`

    */
    pub async fn post_update_playlist<'a>(
        &'a self,
        body: &'a types::PostUpdatePlaylistBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updatePlaylist", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_update_playlist",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates the description and/or expiration date for an existing share

    Updates the description and/or expiration date for an existing share.

    Sends a `GET` request to `/rest/updateShare`

    Arguments:
    - `description`: A user-defined description that will be displayed to people visiting the shared media.
    - `expires`: The time at which the share expires. Given as milliseconds since 1970, or zero to remove the expiration.
    - `id`: ID of the share to update.
    */
    pub async fn update_share<'a>(
        &'a self,
        description: Option<&'a str>,
        expires: Option<u64>,
        id: &'a str,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updateShare", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "description",
                &description,
            ))
            .query(&progenitor_client::QueryParam::new("expires", &expires))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates the description and/or expiration date for an existing share

    Updates the description and/or expiration date for an existing share.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/updateShare`

    */
    pub async fn post_update_share<'a>(
        &'a self,
        body: &'a types::PostUpdateShareBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updateShare", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_update_share",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Modifies an existing user on the server

    Modifies an existing user on the server.

    Sends a `GET` request to `/rest/updateUser`

    Arguments:
    - `admin_role`: Whether the user is administrator.
    - `comment_role`: Whether the user is allowed to create and edit comments and ratings.
    - `cover_art_role`: Whether the user is allowed to change cover art and tags.
    - `download_role`: Whether the user is allowed to download files.
    - `email`: The email address of the user.
    - `jukebox_role`: Whether the user is allowed to play files in jukebox mode.
    - `ldap_authenticated`: Whether the user is authenicated in LDAP.
    - `max_bit_rate`: (Since 1.13.0) The maximum bit rate (in Kbps) for the user. Audio streams of higher bit rates are automatically downsampled to this bit rate. Legal values: 0 (no limit), 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320.
    - `music_folder_id`: (Since 1.12.0) IDs of the music folders the user is allowed access to. Include the parameter once for each folder.
    - `password`: The password of the user, either in clear text of hex-encoded (see above).
    - `podcast_role`: Whether the user is allowed to administrate Podcasts.
    - `settings_role`: Whether the user is allowed to change personal settings and password.
    - `share_role`: Whether the user is allowed to share files with anyone.
    - `stream_role`: Whether the user is allowed to play files.
    - `upload_role`: Whether the user is allowed to upload files.
    - `username`: The name of the user.
    - `video_conversion_role`: (Since 1.15.0) Whether the user is allowed to start video conversions.
    */
    pub async fn update_user<'a>(
        &'a self,
        admin_role: Option<bool>,
        comment_role: Option<bool>,
        cover_art_role: Option<bool>,
        download_role: Option<bool>,
        email: Option<&'a str>,
        jukebox_role: Option<bool>,
        ldap_authenticated: Option<bool>,
        max_bit_rate: Option<&'a types::UpdateUserMaxBitRate>,
        music_folder_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        password: &'a str,
        podcast_role: Option<bool>,
        settings_role: Option<bool>,
        share_role: Option<bool>,
        stream_role: Option<bool>,
        upload_role: Option<bool>,
        username: &'a str,
        video_conversion_role: Option<bool>,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updateUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "adminRole",
                &admin_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "commentRole",
                &comment_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "coverArtRole",
                &cover_art_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "downloadRole",
                &download_role,
            ))
            .query(&progenitor_client::QueryParam::new("email", &email))
            .query(&progenitor_client::QueryParam::new(
                "jukeboxRole",
                &jukebox_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "ldapAuthenticated",
                &ldap_authenticated,
            ))
            .query(&progenitor_client::QueryParam::new(
                "maxBitRate",
                &max_bit_rate,
            ))
            .query(&progenitor_client::QueryParam::new(
                "musicFolderId",
                &music_folder_id,
            ))
            .query(&progenitor_client::QueryParam::new("password", &password))
            .query(&progenitor_client::QueryParam::new(
                "podcastRole",
                &podcast_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "settingsRole",
                &settings_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "shareRole",
                &share_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "streamRole",
                &stream_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "uploadRole",
                &upload_role,
            ))
            .query(&progenitor_client::QueryParam::new("username", &username))
            .query(&progenitor_client::QueryParam::new(
                "videoConversionRole",
                &video_conversion_role,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Modifies an existing user on the server

    Modifies an existing user on the server.

    Requires OpenSubsonic extension name `formPost` (As returned by `getOpenSubsonicExtensions`)

    Sends a `POST` request to `/rest/updateUser`

    */
    pub async fn post_update_user<'a>(
        &'a self,
        body: &'a types::PostUpdateUserBody,
    ) -> Result<ResponseValue<types::SubsonicResponse>, Error<()>> {
        let url = format!("{}/rest/updateUser", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_update_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
