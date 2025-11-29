#[cfg(target_os = "android")]
mod android;

#[cfg(not(target_os = "android"))]
mod mpris;

use crate::state::HostNotificationCommand;
use flume::Sender;
pub struct NotificationControl;

impl NotificationControl {
    pub fn init(sender: Sender<HostNotificationCommand>) {
        #[cfg(target_os = "android")]
        android::init(sender);

        #[cfg(not(target_os = "android"))]
        mpris::init(sender);
    }

    pub async fn send(command: HostNotificationCommand) {
        #[cfg(target_os = "android")]
        android::send_media_message(command);

        #[cfg(not(target_os = "android"))]
        mpris::send_media_message(command).await;
    }

    pub fn update_media_notification(
        title: &str,
        artist: &str,
        track_len: i64,
        track_progress: i64,
        playing: bool,
        artwork_bytes: Option<Vec<u8>>,
    ) {
        #[cfg(target_os = "android")]
        android::update_media_notification(
            title,
            artist,
            track_len,
            track_progress,
            playing,
            artwork_bytes,
        );

        #[cfg(not(target_os = "android"))]
        mpris::update_media_notification(
            title,
            artist,
            track_len,
            track_progress,
            playing,
            artwork_bytes,
        );
    }
}
