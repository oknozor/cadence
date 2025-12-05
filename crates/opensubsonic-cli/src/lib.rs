pub mod codegen;
pub use codegen::*;

use once_cell::sync::OnceCell;
use progenitor_client::{ClientHooks, OperationInfo};

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
