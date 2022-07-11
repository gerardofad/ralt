pub fn get_file_name(path: &str) -> &str {
    let directories: Vec<&str> = path.split(
        |character| character == '/' || character == '\\'
    ).collect();

    directories[directories.len() - 1]
}

pub fn get_folder(path: &str) -> String {
    let mut directories: Vec<&str> = path.split(
        |character| character == '/' || character == '\\'
    ).collect();

    directories.remove(directories.len() - 1);
    directories.join("/")
}
