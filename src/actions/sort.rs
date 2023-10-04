use crate::consts;

use super::cached_paths;

pub fn sort_paths(paths: &mut Vec<String>) {
    let cached_paths = cached_paths::get_cached_paths();
    let sorted_paths = create_sorted_paths(paths, cached_paths);

    print!("{}", sorted_paths.join("\n"));
}

fn create_sorted_paths(paths: &mut Vec<String>, cached_paths: Vec<String>) -> Vec<String> {
    let mut sorted_paths: Vec<String> = vec![];

    for cached_path in cached_paths {
        if sorted_paths.len() >= consts::SUGGESTIONS_THRESHOLD {
            return sorted_paths;
        }

        if paths.contains(&cached_path) {
            paths.retain(|p| *p != cached_path);
            sorted_paths.push(cached_path);
        }
    }

    sorted_paths.append(paths);
    sorted_paths
}
