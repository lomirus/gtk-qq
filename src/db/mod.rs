use rusqlite::Connection;

#[derive(Debug)]
pub struct User {
    pub account: i64,
    pub name: String,
    pub remark: String,
}

#[derive(Debug)]
pub struct Config {
    pub key: String,
    pub value: String,
}

pub fn init_sqlite() {
    // Initialize `~/.gtk-qq/` directory
    let mut db_path = dirs::home_dir().unwrap();
    db_path.push(".gtk-qq");
    std::fs::create_dir_all(db_path.clone()).unwrap();

    // Create or read from `~/.gtk-qq/data.db`
    db_path.push("data.db");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "Create table if not exists users (
            account INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            remark  TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "
        Create table if not exists config (
            key     TEXT PRIMARY KEY,
            value   TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
}
