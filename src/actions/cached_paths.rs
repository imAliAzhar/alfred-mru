use dirs;

use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::consts;

pub fn get_cached_paths() -> Vec<String> {
    let config_file = get_cache_file_path();

    read_to_strings(&config_file)
}

pub fn save_cached_paths(paths: Vec<String>) {
    let config_file = get_cache_file_path();

    fs::write(&config_file, paths.join("\n")).unwrap();
}

fn get_cache_file_path() -> PathBuf {
    let config_dir = dirs::home_dir().unwrap();
    let config_dir = config_dir.join(consts::CACHE_DIR).join(consts::PKG_NAME);

    fs::create_dir_all(&config_dir).unwrap();

    config_dir.join(consts::PKG_NAME)
}

fn read_to_strings(path: &PathBuf) -> Vec<String> {
    let mut content = String::new();

    File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    content.lines().map(|l| l.to_owned()).collect::<Vec<_>>()
}
