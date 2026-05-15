use mpris::PlayerFinder;

use crate::fetch::MusicMetadata;

pub fn get_music_metadata() -> Option<Vec<MusicMetadata>> {
    let finder = PlayerFinder::new().ok()?;
    let players = finder.find_all().ok()?;

    let results = players
        .into_iter()
        .filter(|p| {
            p.get_playback_status()
                .map(|s| s == mpris::PlaybackStatus::Playing)
                .unwrap_or(false)
        })
        .filter_map(|p| {
            let meta = p.get_metadata().ok()?;
            let title = meta.title().unwrap_or_default().to_string();
            let raw_artist = meta
                .artists()
                .and_then(|a| a.first().cloned())
                .unwrap_or_default()
                .to_string();
            let (artist, album) = if let Some((a, al)) = raw_artist.split_once(" \u{2014} ") {
                (a.to_string(), al.to_string())
            } else {
                (raw_artist, meta.album_name().unwrap_or_default().to_string())
            };
            Some(MusicMetadata { title, artist, album })
        })
        .collect();

    Some(results)
}
