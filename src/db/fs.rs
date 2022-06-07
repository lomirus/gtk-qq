use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

pub async fn download_user_avatar_file(user_id: i64) {
    let mut path = dirs::home_dir().unwrap();
    path.push(".gtk-qq");
    path.push("avatars");
    path.push("users");
    create_dir_all(path.clone()).unwrap();
    path.push(format!("{}.png", user_id));
    let mut file = File::create(path).unwrap();

    let url = format!("http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160", user_id);
    println!("Downloading {}", url);
    let body = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    file.write_all(&body).unwrap();
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
    create_dir_all(path.clone()).unwrap();
    path.push(format!("{}.png", group_id));
    let mut file = File::create(path).unwrap();

    let url = format!("https://p.qlogo.cn/gh/{}/{}/0", group_id, group_id);
    println!("Downloading {}", url);
    let body = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    file.write_all(&body).unwrap();
}

pub fn get_group_avatar_path(group_id: i64) -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".gtk-qq");
    path.push("avatars");
    path.push("groups");
    path.push(format!("{}.png", group_id));

    path
}
