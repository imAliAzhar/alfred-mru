use super::cached_paths;

pub fn list_paths() {
    let cached_paths = cached_paths::get_cached_paths();

    print!("{}", cached_paths.join("\n"));
}
