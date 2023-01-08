use std::{
    error::Error,
    path::{Path, PathBuf},
};

const BAD_FILENAME_CHARACTERS: [char; 9] = ['<', '>', ':', '\\', '/', '\\', '"', '?', '*'];

pub fn str_clean_as_filename(name: &str, extension: Option<&str>) -> String {
    let f = name.to_owned().replace(BAD_FILENAME_CHARACTERS, "");

    match extension {
        Some(e) => f + "." + e,
        None => f + ".csv",
    }
}

pub fn new_path(path: &Path, suffix: &str) -> PathBuf {
    // new file
    path.with_file_name(format!(
        "{}{}",
        path.file_stem().unwrap().to_str().unwrap(),
        suffix
    ))
    .with_extension(path.extension().unwrap())
}

pub fn full_path(f: &str) -> Result<PathBuf, Box<dyn Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(f));

    Ok(path)
}
