#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchResult {
    Artist {
        id: String,
        name: String,
        thumbnail: Option<String>,
    },
    Album {
        id: String,
        name: String,
        cover: Option<String>,
        artist: Option<String>,
    },
    Song {
        id: String,
        name: String,
        cover: Option<String>,
        artist: Option<String>,
    },
}
