extern crate termion;

use std::fs;
use std::fs::File;
use std::io::Write;
use super::cursors;


pub fn files_view(target:&str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
        for path in fs::read_dir(target).unwrap() {
            files.push(path.unwrap().path().display().to_string())
        }
    files
}

pub fn file_open(file_view: &mut Vec<String>, cursor:&mut cursors::Cursor) -> (Vec<String>, String) {
    let target = file_view[cursor.row - 1].to_string();
    let file_opend = files_view(&target);
    cursor.row = 1;
    return (file_opend, target);
}

pub fn save(buffer: &Vec<Vec<char>>, path: &std::string::String) {
    if let Ok(mut file) = File::create(path) {
        for line in buffer {
            for c in line {
                write!(file, "{}", c).unwrap();
            }
            writeln!(file).unwrap();
        }
    }
}
