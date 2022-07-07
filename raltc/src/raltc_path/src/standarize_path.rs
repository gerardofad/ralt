pub fn standarize_path(path: &String) -> String {
    let standard_path: String;

    for character in path {
        if character == " "  || character == "\t" ||
           character == "\r" || character == "\n" {
        }
    }

    standard_path.make_ascii_lowercase();
    standard_path
}
