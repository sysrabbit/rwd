use walkdir::WalkDir;

// This is to load multiple files
pub fn get_files(ext: String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new("tests/files/").into_iter().filter_map(|e| e.ok()) {
        if entry.path().to_str().unwrap().ends_with(&ext) {
            match entry.path().to_str() {
                Some(obj) => files.push(String::from(obj)),
                _ => (),
            }
        }
    }
    files
}