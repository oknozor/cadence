use opensubsonic_cli::types::InternetRadioStation;

#[derive(Clone, PartialEq)]
pub struct RadioStation {
    pub id: String,
    pub name: String,
    pub stream_url: String,
    pub home_page_url: Option<String>,
}

impl From<InternetRadioStation> for RadioStation {
    fn from(station: InternetRadioStation) -> Self {
        RadioStation {
            id: station.id,
            name: station.name,
            stream_url: station.stream_url,
            home_page_url: station.home_page_url,
        }
    }
}

