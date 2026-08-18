use crate::models::tracks;
pub struct Playlist {
    pub playlist: PlaylistInformation
}

pub struct PlaylistInformation {
    pub location: String,
    pub tracks: Vec<tracks::Track>,
    pub track_count: i32,
    pub favorite: bool,
    pub id: i32
}
