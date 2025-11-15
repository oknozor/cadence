include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

use once_cell::sync::OnceCell;

pub static USERNAME: OnceCell<String> = OnceCell::new();
pub static PASSWORD: OnceCell<String> = OnceCell::new();

impl ClientHooks<()> for Client {
    async fn pre<E>(
        &self,
        request: &mut reqwest::Request,
        _: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        let username = USERNAME.get().expect("username to be set");
        let password = PASSWORD.get().expect("password to be set");
        let username = username.to_string();
        let password = password.to_string();

        request.url_mut().query_pairs_mut().extend_pairs(&[
            ("u", username.as_str()),
            ("p", password.as_str()),
            ("f", "json"),
            ("v", "1.16.1"),
            ("c", "scrobz"),
        ]);

        println!("{:?}", request.url());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, PASSWORD, USERNAME};
    use crate::types::AlbumListType;

    const ALBUM_ID: &str = "5imOrY1P5PnAYsaoM06BXf";
    const ARTIST_ID: &str = "5imOrY1P5PnAYsaoM06BXf";
    const API_URL: &str = "https://music-api.hoohoot.org";

    #[tokio::test]
    async fn test_get_album() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client.get_album(ALBUM_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_album_info() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client.get_album_info(ALBUM_ID).await?;
        Ok(())
    }
    #[tokio::test]
    async fn test_get_artist() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client.get_artist(ARTIST_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_artist_info() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client.get_artist_info(Some(1), ARTIST_ID, None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_artist_info2() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client.get_artist_info2(Some(1), ARTIST_ID, None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_search3() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client.search3(Some(15), None, Some(15), None, None, "nirvana", Some(15), None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_album_list2() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(API_URL);
        client
            .get_album_list2(None, None, None, None, None, None, AlbumListType::Newest)
            .await?;
        Ok(())
    }
}
