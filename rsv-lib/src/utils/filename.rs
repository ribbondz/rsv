use std::path::{Path, PathBuf};

const BAD_FILENAME_CHARACTERS: [char; 9] = ['<', '>', ':', '\\', '/', '\\', '"', '?', '*'];

pub fn new_path(path: &Path, suffix: &str) -> PathBuf {
    let p = path.with_file_name(format!(
        "{}{}",
        path.file_stem().unwrap().to_str().unwrap(),
        suffix
    ));

    // new file
    match path.extension() {
        Some(e) => p.with_extension(e),
        None => p,
    }
}

pub fn new_file(name: &str) -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push(name);

    path
}

pub fn str_to_filename(s: &str) -> String {
    s.replace(BAD_FILENAME_CHARACTERS, "")
}

pub fn dir_file(dir: &Path, name: &str) -> PathBuf {
    let mut out = dir.to_path_buf();
    out.push(name);

    out
}

pub fn full_path(f: &str) -> PathBuf {
    // current file
    let mut path = std::env::current_dir().unwrap();
    path.push(Path::new(f));

    path
}
