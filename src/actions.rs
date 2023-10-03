pub enum Action {
    Cache,
    Sort,
}

pub fn cache_path<'a>(path: String) {
    dbg!(path);
}

pub fn sort_paths<'a>(paths: Vec<String>) {
    dbg!(paths);
}
