use std::path::PathBuf;

const BAD_FILENAME_CHARACTERS: [&'static str; 9] = ["<", ">", ":", "\"", "/", "\\", "|", "?", "*"];

pub fn clean_filename(name: &str) -> String {
    let mut new_name = name.to_owned();
    for s in BAD_FILENAME_CHARACTERS {
        new_name = new_name.replace(s, "");
    }
    new_name
}

pub fn generate_filename(name: &str, extension: Option<&str>) -> String {
    let new_name = clean_filename(name);
    let extension = extension.unwrap_or("csv");
    new_name + "." + extension
}

pub fn new_path(path: &PathBuf, suffix: &str) -> PathBuf {
    // new file
    let mut new_path = path.clone();
    new_path.set_file_name(path.file_stem().unwrap().to_str().unwrap().to_owned() + suffix);
    if let Some(e) = path.extension() {
        new_path.set_extension(e);
    };
    new_path
}
