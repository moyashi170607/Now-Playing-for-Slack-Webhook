use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
pub mod windows_fetch;

#[cfg(target_os = "linux")]
pub mod linux_fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub fn get_music_metadata() -> Option<Vec<MusicMetadata>> {
    #[cfg(target_os = "windows")]
    return windows_fetch::get_music_metadata();

    #[cfg(target_os = "linux")]
    return linux_fetch::get_music_metadata();

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    return None;
}
