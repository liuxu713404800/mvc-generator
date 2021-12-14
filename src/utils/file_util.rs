use std::fs::File;
use std::io::prelude::*;

pub fn get_content(file: &str) -> String {
    let mut f = File::open(file).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}