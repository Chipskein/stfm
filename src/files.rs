#![allow(unused)]
use std::io::Read;
pub fn list_files(current_dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path.display().to_string());
    }
    return files;
}
pub fn create_file(file_name: &str) {
    std::fs::File::create(file_name).unwrap();
}
pub fn delete_file(file_name: &str) {
    std::fs::remove_file(file_name).unwrap();
}
pub fn read_file(file_name: &str) -> String {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
pub fn change_dir(dir_name: &str) {
    std::env::set_current_dir(dir_name).unwrap();
}
pub fn make_dir(dir_name: &str) {
    std::fs::create_dir(dir_name).unwrap();
}
pub fn delete_dir(dir_name: &str) {
    std::fs::remove_dir_all(dir_name).unwrap();
}
pub fn rename_file(old_name: &str, new_name: &str) {
    std::fs::rename(old_name, new_name).unwrap();
}
pub fn copy_file(old_name: &str, new_name: &str) {
    std::fs::copy(old_name, new_name).unwrap();
}
pub fn get_absolute_path(file_name: &str) -> String {
    let path = std::path::Path::new(file_name);
    return path.canonicalize().unwrap().display().to_string();
}
pub fn file_exists(file_name: &str) -> bool {
    return std::path::Path::new(file_name).exists();
}
