use super::cached_paths;

use crate::consts;

pub fn cache_path<'a>(path: String) {
    let mut cached_paths = cached_paths::get_cached_paths();

    if cached_paths.contains(&path) {
        cached_paths.retain(|p| *p != path);
    }

    cached_paths.insert(0, path);

    if cached_paths.len() > consts::CACHE_THRESHOLD {
        cached_paths.truncate(consts::CACHE_THRESHOLD);
    }

    cached_paths::save_cached_paths(cached_paths);
}
