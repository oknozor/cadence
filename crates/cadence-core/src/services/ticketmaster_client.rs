use reqwest::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://app.ticketmaster.com/discovery/v2/events";

#[derive(Clone)]
pub struct TicketmasterClient {
    client: Client,
    api_key: String,
}

#[derive(Debug)]
pub enum TicketmasterError {
    Http(reqwest::Error),
    NotFound(String),
    ApiError(String),
}

impl std::fmt::Display for TicketmasterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketmasterError::Http(e) => write!(f, "HTTP error: {}", e),
            TicketmasterError::NotFound(msg) => write!(f, "Not found: {}", msg),
            TicketmasterError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for TicketmasterError {}

impl From<reqwest::Error> for TicketmasterError {
    fn from(e: reqwest::Error) -> Self {
        TicketmasterError::Http(e)
    }
}

// API Response types
#[derive(Debug, Deserialize)]
pub struct EventsResponse {
    #[serde(rename = "_embedded")]
    pub embedded: Option<EventsEmbedded>,
}

#[derive(Debug, Deserialize)]
pub struct EventsEmbedded {
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    pub url: String,
    pub dates: EventDates,
    #[serde(rename = "_embedded")]
    pub embedded: Option<EventEmbedded>,
}

#[derive(Debug, Deserialize)]
pub struct EventDates {
    pub start: EventStart,
}

#[derive(Debug, Deserialize)]
pub struct EventStart {
    #[serde(rename = "localDate")]
    pub local_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EventEmbedded {
    pub venues: Vec<Venue>,
}

#[derive(Debug, Deserialize)]
pub struct Venue {
    pub name: Option<String>,
    pub city: Option<VenueCity>,
    pub country: Option<VenueCountry>,
}

#[derive(Debug, Deserialize)]
pub struct VenueCity {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VenueCountry {
    pub name: Option<String>,
}

// Simplified output type
#[derive(Debug, Clone, PartialEq)]
pub struct Concert {
    pub name: String,
    pub date: String,
    pub venue: String,
    pub city: String,
    pub country: String,
    pub url: String,
}

impl TicketmasterClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    /// Search for events by artist and city with optional radius
    pub async fn search_events(
        &self,
        artist: &str,
        city: &str,
        radius_km: Option<u32>,
    ) -> Result<Vec<Concert>, TicketmasterError> {
        tracing::info!(
            "[Ticketmaster] Searching for artist: {} in city: {} (radius: {:?} km)",
            artist,
            city,
            radius_km
        );

        let radius_str = radius_km.map(|r| r.to_string());
        let mut query_params = vec![
            ("keyword", artist.to_string()),
            ("city", city.to_string()),
            ("locale", "*".to_string()),
            ("apikey", self.api_key.clone()),
        ];

        if let Some(ref radius) = radius_str {
            query_params.push(("radius", radius.clone()));
            query_params.push(("unit", "km".to_string()));
        }

        let response = self
            .client
            .get(BASE_URL)
            .query(&query_params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("[Ticketmaster] Search failed: {} - {}", status, text);
            return Err(TicketmasterError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let events_response: EventsResponse = response.json().await?;

        let concerts = match events_response.embedded {
            Some(embedded) => embedded
                .events
                .into_iter()
                .map(|event| {
                    let (venue, city, country) = event
                        .embedded
                        .and_then(|e| e.venues.into_iter().next())
                        .map(|v| {
                            (
                                v.name.unwrap_or_default(),
                                v.city.and_then(|c| c.name).unwrap_or_default(),
                                v.country.and_then(|c| c.name).unwrap_or_default(),
                            )
                        })
                        .unwrap_or_default();

                    Concert {
                        name: event.name,
                        date: event.dates.start.local_date.unwrap_or_default(),
                        venue,
                        city,
                        country,
                        url: event.url,
                    }
                })
                .collect(),
            None => Vec::new(),
        };

        tracing::info!("[Ticketmaster] Found {} events", concerts.len());
        Ok(concerts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_orelsan_in_lille() {
        let client = TicketmasterClient::new("BTFNd5GjAOjNYbXv2AzFByf19C46IAKM");

        // Test searching in Lille with 50km radius
        let result = client.search_events("orelsan", "lille", Some(50)).await;

        match result {
            Ok(ref concerts) => {
                println!("Found {} concerts for Orelsan in Lille (50km radius):", concerts.len());
                for concert in concerts {
                    println!(
                        "  - {} on {} at {} ({}, {})",
                        concert.name, concert.date, concert.venue, concert.city, concert.country
                    );
                    println!("    Tickets: {}", concert.url);
                }
            }
            Err(ref e) => {
                panic!("Error searching for concerts: {}", e);
            }
        }

        assert!(result.is_ok(), "API call should succeed");
    }
}
