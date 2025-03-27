use crate::helper;
use crate::player;
use mpvipc::*;

/// Add files at the end of the current playlist.
pub fn append(mpv: &Mpv, files: &[String]) {
    for file in files.iter() {
        let path = helper::abspath(file);
        mpv.playlist_add(
            path.as_str(),
            PlaylistAddTypeOptions::File,
            PlaylistAddOptions::Append,
        )
        .unwrap();
        println!("Added {:?}", file);
    }
}

/// Insert files after the currently played one.
pub fn queue(mpv: &Mpv, files: &[String], play: &bool) {
    for file in files.iter().rev() {
        let path = helper::abspath(file);
        mpv.run_command_raw("loadfile", &[path.as_str(), "insert-next"])
            .unwrap();
        println!("Added {:?}", file);
    }
    if *play {
        mpv.set_property("pause", false).expect("Failed to resume");
        player::next(mpv);
    }
}
