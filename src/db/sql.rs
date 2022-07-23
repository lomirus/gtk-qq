use std::error::Error;

use crate::config::DB_VERSION;
use crate::handler::CLIENT;
use resource_loader::{SqlDataBase, SyncCreatePath, SyncLoadResource};
use ricq::structs::{FriendGroupInfo, FriendInfo, GroupInfo};
use rusqlite::{params, Connection};

pub struct SqlDb;

impl SyncLoadResource<rusqlite::Connection> for SqlDb {
    type Args = ();

    type Error = rusqlite::Error;

    fn load_resource(_: Self::Args) -> Result<rusqlite::Connection, Self::Error> {
        let db_file = SqlDataBase::create_and_get_path().expect("Failure Load DB information");

        Connection::open(db_file)
    }
}

pub fn get_db() -> Connection {
    SqlDb::load_resource(()).expect("Load Sqlite Db Failure")
}

#[derive(Debug)]
pub struct Config {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Friend {
    pub id: i64,
    pub name: String,
    // TODO: Make this Option<String>
    pub remark: String,
    pub group_id: u8,
}

pub struct FriendsGroup {
    pub id: u8,
    pub name: String,
    pub online_friends: i32,
}

#[derive(Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

pub fn init_sqlite() {
    let conn = SqlDb::load_resource(()).expect("Load Sqlite Db Failure");

    conn.execute(
        "Create table if not exists configs (
            key     TEXT PRIMARY KEY,
            value   TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    check_db_version();

    conn.execute(
        "Create table if not exists friends (
            id          INT PRIMARY KEY,
            name        TEXT NOT NULL,
            remark      TEXT NOT NULL,
            group_id    INT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "Create table if not exists friends_groups (
            id              INT PRIMARY KEY,
            name            TEXT NOT NULL,
            online_friends  INT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "Create table if not exists groups (
            id          INT PRIMARY KEY,
            name        TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
}

pub async fn refresh_friends_list() -> Result<(), Box<dyn Error>> {
    let conn = get_db();
    // Request for friend list
    let client = CLIENT.get().unwrap();
    let res = client.get_friend_list().await?;
    // Store the friend list in the memory
    let friends = res.friends;
    let friend_groups = res.friend_groups;
    // Handle the `friends_groups`
    let mut friends_groups = friend_groups
        .iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<FriendGroupInfo>>();
    friends_groups.sort_by(|a, b| a.seq_id.cmp(&b.seq_id));
    let friends_groups = friends_groups
        .into_iter()
        .map(|friends_group| FriendsGroup {
            id: friends_group.group_id,
            name: friends_group.group_name,
            online_friends: friends_group.online_friend_count,
        });
    conn.execute("DELETE FROM friends_groups", [])?;
    let mut stmt = conn.prepare("INSERT INTO friends_groups values (?1, ?2, ?3)")?;
    for friends_group in friends_groups {
        stmt.execute(params![
            friends_group.id,
            friends_group.name,
            friends_group.online_friends
        ])?;
    }
    // Handle the friends
    let friends = friends.into_iter().map(
        |FriendInfo {
             uin,
             nick,
             remark,
             group_id,
             ..
         }| Friend {
            id: uin,
            name: nick,
            remark,
            group_id,
        },
    );
    conn.execute("DELETE FROM friends", [])?;
    let mut stmt = conn.prepare("INSERT INTO friends values (?1, ?2, ?3, ?4)")?;
    for friend in friends {
        stmt.execute(params![
            friend.id,
            friend.name,
            friend.remark,
            friend.group_id
        ])?;
    }

    Ok(())
}

pub async fn refresh_groups_list() -> Result<(), Box<dyn Error>> {
    let conn = get_db();
    let client = CLIENT.get().unwrap();
    let res = client.get_group_list().await?;

    let groups = res
        .into_iter()
        .map(|GroupInfo { code, name, .. }| Group { id: code, name });

    conn.execute("DELETE FROM groups", [])?;
    let mut stmt = conn.prepare("INSERT INTO groups values (?1, ?2)")?;
    for group in groups {
        stmt.execute(params![group.id, group.name])?;
    }

    Ok(())
}

pub fn get_friend_remark(friend_id: i64) -> String {
    get_db()
        .query_row(
            "Select remark from friends where id=?1",
            [friend_id],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| {
            println!("Failed to get friend remark: {}", friend_id);
            println!(concat!(
                "Help: Try to refresh the friends list in sidebar. ",
                "If the problem still exists, please report it on Github.",
            ));
            friend_id.to_string()
        })
}

pub fn get_group_name(group_id: i64) -> String {
    get_db()
        .query_row("Select name from groups where id=?1", [group_id], |row| {
            row.get(0)
        })
        .unwrap_or_else(|_| {
            println!("Failed to get group name: {}", group_id);
            println!(concat!(
                "Help: Try to refresh the groups list in sidebar. ",
                "If the problem still exists, please report it on Github.",
            ));
            group_id.to_string()
        })
}

pub fn check_db_version() {
    let conn = get_db();
    let res = conn.query_row::<String, _, _>(
        "Select value from configs where key='db_version'",
        [],
        |row| row.get(0),
    );
    match res {
        Ok(version) => {
            let version: usize = version.parse().unwrap();
            if version != DB_VERSION {
                panic!("unrecognized database version")
            }
        }
        Err(err) => {
            if err.to_string() == "Query returned no rows" {
                conn.execute(
                    "Insert into configs values ('db_version', ?1)",
                    [DB_VERSION.to_string()],
                )
                .unwrap();
            } else {
                panic!("{}", err);
            }
        }
    }
}

pub fn load_sql_config(
    key: &(impl AsRef<str> + ?Sized),
) -> Result<Option<String>, rusqlite::Error> {
    let conn = get_db();
    let mut stmt = conn.prepare("SELECT value FROM configs where key= ? ")?;
    let mut query = stmt.query(params![key.as_ref()])?;
    query.next()?.map(|row| row.get::<_, String>(0)).transpose()
}

pub fn save_sql_config(
    key: &(impl AsRef<str> + ?Sized),
    value: impl AsRef<str>,
) -> Result<(), rusqlite::Error> {
    let db = get_db();

    db.execute(
        "REPLACE INTO configs (key, value) VALUES (?1, ?2)",
        params![key.as_ref(), value.as_ref()],
    )
    .map(|_| ())
}

#[cfg(test)]
mod test {

    use crate::db::sql::load_sql_config;

    #[test]
    fn test_account_load() {
        let acc = load_sql_config("account");

        println!("{acc:?}")
    }
}
