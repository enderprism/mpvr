pub fn abspath(file: &String) -> String {
    if !file.contains("http") {
        std::path::absolute(file)
            .expect("Failed to load file")
            .to_str()
            .expect("File path is invalid unicode")
            .to_owned()
    } else {
        file.to_owned()
    }
}
