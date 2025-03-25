use mpvipc::Mpv;

pub fn pause(mpv: &Mpv) {
    mpv.pause().unwrap();
}

pub fn next(mpv: &Mpv) {
    let playlist_pos: f64 = mpv.get_property("playlist-pos").unwrap();
    let track_count: usize = mpv.get_property("playlist-count").unwrap();
    if playlist_pos == -1.0 && track_count > 0 {
        mpv.playlist_play_id(track_count - 1).unwrap();
    } else {
        mpv.next().expect("Hit the end of the playlist");
    };
}

pub fn prev(mpv: &Mpv) {
    let playlist_pos: f64 = mpv.get_property("playlist-pos").unwrap();
    let track_count: usize = mpv.get_property("playlist-count").unwrap();
    if playlist_pos == -1.0 && track_count > 0 {
        mpv.playlist_play_id(track_count - 2).unwrap();
    } else {
        mpv.prev().expect("Hit the start of the playlist");
    };
}

pub fn set_volume(mpv: &Mpv, volume: f64) {
    mpv.set_volume(volume, mpvipc::NumberChangeOptions::Absolute)
        .unwrap();
}

pub fn increase_volume(mpv: &Mpv, volume: f64) {
    mpv.set_volume(volume, mpvipc::NumberChangeOptions::Increase)
        .unwrap();
}

pub fn decrease_volume(mpv: &Mpv, volume: f64) {
    mpv.set_volume(volume, mpvipc::NumberChangeOptions::Decrease)
        .unwrap();
}

pub fn get_volume(mpv: &Mpv) {
    let volume: f64 = mpv.get_property("volume").unwrap();
    print!("{}", volume);
}
