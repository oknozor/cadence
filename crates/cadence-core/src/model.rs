mod album;
mod artist;
mod playlist;
mod search;
mod song;
mod starred;

pub use album::Album;
pub use artist::Artist;
pub use playlist::PlaylistInfo;
pub use search::SearchResult;
pub use song::Song;
pub use starred::Starred;

fn ensure_https(url: String) -> String {
    if url.starts_with("https://") {
        url
    } else if let Some(rest) = url.strip_prefix("http://") {
        format!("https://{}", rest)
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
