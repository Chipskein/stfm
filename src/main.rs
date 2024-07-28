use std::io::Read;
fn list_files(current_dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path.display().to_string());
    }
    return files;
}
fn create_file(file_name: &str) {
    std::fs::File::create(file_name).unwrap();
}
fn delete_file(file_name: &str) {
    std::fs::remove_file(file_name).unwrap();
}
fn read_file(file_name: &str) -> String {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
fn change_dir(dir_name: &str) {
    std::env::set_current_dir(dir_name).unwrap();
}
fn make_dir(dir_name: &str) {
    std::fs::create_dir(dir_name).unwrap();
}
fn delete_dir(dir_name: &str) {
    std::fs::remove_dir_all(dir_name).unwrap();
}
fn rename_file(old_name: &str, new_name: &str) {
    std::fs::rename(old_name, new_name).unwrap();
}
fn copy_file(old_name: &str, new_name: &str) {
    std::fs::copy(old_name, new_name).unwrap();
}
fn get_absolute_path(file_name: &str) -> String {
    let path = std::path::Path::new(file_name);
    return path.canonicalize().unwrap().display().to_string();
}
fn file_exists(file_name: &str) -> bool {
    return std::path::Path::new(file_name).exists();
}
fn main()  {
   
}
