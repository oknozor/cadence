#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub cover_art: Option<String>,
}
