use crate::components::ItemInfo;
use cadence_core::services::ticketmaster_client::Concert;
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
pub fn ArtistLiveEvents(concerts: Vec<Concert>) -> Element {
    rsx! {
        section { class: "artist-events",
            h2 { "Upcoming Concerts" }
            div { class: "concerts-list",
                for concert in concerts {
                    a {
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "concert-item",
                        href: "{concert.url}",
                        div { class: "concert-item-content",
                            DateCard { date: concert.date }
                            div {
                                ItemInfo {
                                    primary: "{concert.name}",
                                    secondary: "{concert.venue} - {concert.city}, {concert.country}",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DateCard(date: NaiveDate) -> Element {
    let month = date_month(date);
    let day = date_day(date);

    rsx! {
        div { class: "event-date-card",
            span { class: "event-date-month", "{month}" }
            span { class: "event-date-day", "{day}" }
        }
    }
}

fn date_month(date: NaiveDate) -> String {
    date.format("%b").to_string()
}

fn date_day(date: NaiveDate) -> String {
    date.format("%d").to_string()
}
