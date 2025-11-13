use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Track(track_id: String, title: String) -> Element {
    let sender = use_context::<Sender<PlayerCommand>>();

    rsx!(
        div {
            class: "track-item",
            onclick: move |_| {
                let sender = sender.clone();
                let track_id = track_id.clone();

                spawn(async move {
                    sender.send(PlayerCommand::Queue(track_id)).await.unwrap();
                });
            },
            "{title}"
        }
    )
}
