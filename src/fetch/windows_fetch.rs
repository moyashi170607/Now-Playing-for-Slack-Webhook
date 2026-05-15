use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

use crate::fetch::MusicMetadata;

pub fn get_music_metadata() -> Option<Vec<MusicMetadata>> {
    pollster::block_on(async {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .ok()?
            .await
            .ok()?;

        // 全セッションを取得
        let sessions = manager.GetSessions().ok()?;

        let mut results = Vec::new();
        for session in &sessions {
            let is_playing = session
                .GetPlaybackInfo()
                .and_then(|info| info.PlaybackStatus())
                .map(|s| s == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing)
                .unwrap_or(false);
            if !is_playing {
                continue;
            }

            let props = match session.TryGetMediaPropertiesAsync().ok()?.await {
                Ok(p) => p,
                Err(_) => continue,
            };
            let raw_artist = props.Artist().unwrap_or_default().to_string();
            let (artist, album) = if let Some((a, al)) = raw_artist.split_once(" \u{2014} ") {
                (a.to_string(), al.to_string())
            } else {
                (raw_artist, props.AlbumTitle().unwrap_or_default().to_string())
            };
            results.push(MusicMetadata {
                title: props.Title().unwrap_or_default().to_string(),
                artist,
                album,
            });
        }

        Some(results)
    })
}
