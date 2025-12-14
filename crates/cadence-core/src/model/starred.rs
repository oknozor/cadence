use crate::model::{Album, Artist, Song};

pub struct Starred {
    pub songs: Vec<Song>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
}
