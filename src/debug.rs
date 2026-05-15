use crate::fetch::windows_fetch::get_music_metadata;

pub fn _print_metadata() {
    let data = get_music_metadata();

    match data {
        Some(tracks) if tracks.is_empty() => println!("再生中の音楽はありません"),
        Some(tracks) => {
            for track in &tracks {
                println!("{} - {} ({})", track.title, track.artist, track.album);
            }
        }
        None => print!("error"),
    }
}
