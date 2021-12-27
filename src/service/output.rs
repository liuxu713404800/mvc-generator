use std::fs;


const BASE_DIR: &str = "src/output";

pub fn write_result(model: &str, filename: &str, content: &str) {
    let dir_name = String::from(BASE_DIR) + "/" + model;
    fs::create_dir_all(&dir_name).unwrap();
    let file = dir_name + "/" + filename;
    fs::write(file, content).unwrap();
}