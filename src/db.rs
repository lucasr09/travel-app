use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("travel.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS countries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            continent TEXT,
            code TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS trips (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country_id INTEGER,
            start_date TEXT,
            end_date TEXT,
            trip_type TEXT,
            rating INTEGER,
            notes TEXT
        )",
        [],
    )?;

    Ok(conn)
}