use std::path::{Path, PathBuf};

const BAD_FILENAME_CHARACTERS: [char; 9] = ['<', '>', ':', '\\', '/', '\\', '"', '?', '*'];

pub fn str_clean_as_filename(name: &str, extension: Option<&str>) -> String {
    let f = name.to_owned().replace(BAD_FILENAME_CHARACTERS, "");

    match extension {
        Some(e) => f + "." + e,
        None => f + ".csv",
    }
}

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

pub fn full_path(f: &str) -> PathBuf {
    // current file
    let mut path = std::env::current_dir().unwrap();
    path.push(Path::new(f));

    path
}
