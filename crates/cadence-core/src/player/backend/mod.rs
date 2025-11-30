#[cfg(not(target_arch = "wasm32"))]
mod rodio;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::{AudioBackend, MusicPlayerError};

#[cfg(not(target_arch = "wasm32"))]
pub use rodio::{AudioBackend, MusicPlayerError};
