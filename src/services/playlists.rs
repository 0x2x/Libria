use crate::models::playlist;
use crate::models::playlist::{Playlist, PlaylistInformation};
use crate::services::database;
use crate::services::database::{init_database, save_playlist};
/*
    {
        "playlist_name": {
            "location": "",
            "tracks": [],
            "track_count": 0,
            "playlist_favorite": false,
            "playlist_id": 0
        }
   }
 */


fn create_playlist(playlist_name: String, playlist_id: i32) -> Playlist {
    let conn = init_database("music_libary.db").unwrap();
    let _ = save_playlist(&conn, 1, "Favorites", "/Music/Favorites", 0, true);

    let playlist = Playlist {
        playlist: PlaylistInformation {
            location: String::new(),
            tracks: Vec::new(),
            favorite: false,
            track_count: 0,
            id: playlist_id
        }
    };
    playlist
}

