use tokio::sync::broadcast::Sender;

use crate::PlayerCommand;

pub fn init(_sender: Sender<PlayerCommand>) {
    todo!("Init not implemented for mpris")
}

pub async fn send_media_message(_command: PlayerCommand) {
    todo!("Notification control not implemented for mpris")
}

pub fn update_media_notification(
    _title: &str,
    _artist: &str,
    _track_len: i64,
    _track_progress: i64,
    _playing: bool,
    _artwork_bytes: Option<Vec<u8>>,
) {
    todo!("Notification control not implemented for mpris")
}
