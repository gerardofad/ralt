// Clean and format in standard paths
pub fn standarize_path(path: &String) -> String {
    let mut standard_path: String = String::new();
    let mut space_exists: bool = false;

    for character in (*path).chars() {

        // Is whitespace
        if character == ' '  || character == '\t' ||
           character == '\r' || character == '\n' {
            if !space_exists { // Add only 1 normal space
                space_exists = true;
                standard_path.push(' ');
                continue;
            }

        // Is other character
        } else {
            // Deactivate whitespaces uniter
            if space_exists { space_exists = false; }
            standard_path.push(character);
        }
    }

    standard_path.make_ascii_lowercase();
    standard_path
}
