use flume::Sender;

use crate::state::HostNotificationCommand;

pub fn init(_sender: Sender<HostNotificationCommand>) {
    tracing::debug!("Init not implemented for mpris")
}

pub async fn send_media_message(_command: HostNotificationCommand) {
    tracing::debug!("Notification control not implemented for mpris")
}

pub fn update_media_notification(
    _title: &str,
    _artist: &str,
    _track_len: i64,
    _track_progress: i64,
    _playing: bool,
    _artwork_bytes: Option<Vec<u8>>,
) {
    tracing::debug!("Notification control not implemented for mpris")
}
