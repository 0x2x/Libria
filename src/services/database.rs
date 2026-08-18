use rusqlite::{Connection, Result};

pub fn init_database(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            location TEXT NOT NULL,
            track_count INTEGER NOT NULL,
            favorite BOOLEAN NOT NULL
        )",
        (),
    )?;
    // Create a tracks table to hold the songs inside those playlists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id INTEGER,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            file_path TEXT NOT NULL,
            play_count INTEGER,
            FOREIGN KEY(playlist_id) REFERENCES playlists(id)
        )",
        (),
    )?;
    Ok(conn)
}

pub fn save_playlist(conn: &Connection, id: i32, name: &str, location: &str, track_count: i32, favorite: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO playlists (id, name, location, track_count, favorite)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            name=excluded.name,
            location=excluded.location,
            track_count=excluded.track_count,
            favorite=excluded.favorite",
        rusqlite::params![id, name, location, track_count, favorite],
    )?;
    Ok(())
}

pub fn delete_playlist(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM playlists WHERE id = ?1",
        rusqlite::params![id],
    )?;
    Ok(())
}
