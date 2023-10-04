use dirs;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::consts;

pub fn cache_path<'a>(path: String) {
    let config_file = get_cache_file_path();

    let content = read_to_string(&config_file);
    let mut cached_paths = content.lines().collect::<Vec<_>>();

    if cached_paths.contains(&path.as_str()) {
        cached_paths.retain(|&x| x != path);
    }

    cached_paths.insert(0, path.as_str());

    if cached_paths.len() > consts::CACHE_THRESHOLD {
        cached_paths.truncate(consts::CACHE_THRESHOLD);
    }

    fs::write(&config_file, cached_paths.join("\n")).unwrap();
}

fn get_cache_file_path() -> PathBuf {
    let config_dir = dirs::home_dir().unwrap();
    let config_dir = config_dir.join(consts::CACHE_DIR).join(consts::PKG_NAME);

    fs::create_dir_all(&config_dir).unwrap();

    config_dir.join(consts::PKG_NAME)
}

fn read_to_string(path: &PathBuf) -> String {
    let mut content = String::new();

    File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    content
}
