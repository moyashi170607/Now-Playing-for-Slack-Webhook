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
            results.push(MusicMetadata {
                title: props.Title().unwrap_or_default().to_string(),
                artist: props.Artist().unwrap_or_default().to_string(),
                album: props.AlbumTitle().unwrap_or_default().to_string(),
            });
        }

        Some(results)
    })
}
