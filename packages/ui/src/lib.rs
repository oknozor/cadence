//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

pub mod client;
pub use client::SubsonicClient;

mod login;
pub use login::Login;

mod player;
pub use player::{Player, Track};

mod library;
pub use library::{Album, LibraryBrowser};
