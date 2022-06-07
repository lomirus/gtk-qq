use std::path::PathBuf;

use tokio::fs::{create_dir_all, write};

pub async fn download_user_avatar_file(user_id: i64) {
    let mut path = dirs::home_dir().unwrap();
    path.push(".gtk-qq");
    path.push("avatars");
    path.push("users");
    create_dir_all(path.clone()).await.unwrap();
    path.push(format!("{}.png", user_id));

    let url = format!("http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160", user_id);
    println!("Downloading {}", url);
    let body = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    write(path, body).await.unwrap();
}

pub fn get_user_avatar_path(user_id: i64) -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".gtk-qq");
    path.push("avatars");
    path.push("users");
    path.push(format!("{}.png", user_id));

    path
}

pub async fn download_group_avatar_file(group_id: i64) {
    let mut path = dirs::home_dir().unwrap();
    path.push(".gtk-qq");
    path.push("avatars");
    path.push("groups");
    create_dir_all(path.clone()).await.unwrap();
    path.push(format!("{}.png", group_id));

    let url = format!("https://p.qlogo.cn/gh/{}/{}/0", group_id, group_id);
    println!("Downloading {}", url);
    let body = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    write(path, body).await.unwrap();
}

pub fn get_group_avatar_path(group_id: i64) -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".gtk-qq");
    path.push("avatars");
    path.push("groups");
    path.push(format!("{}.png", group_id));

    path
}
