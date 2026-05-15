use serde::{Deserialize, Serialize};

pub mod windows_fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
}
