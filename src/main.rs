extern crate termion;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::io::{stdin, stdout, Write};
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use termion::clear;
use termion::screen::AlternateScreen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();
    let file_buffer = BufReader::new(&file);

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    let mut buffer:Vec<Vec<char>> = Vec::new();

    for line in file_buffer.lines() {
        let mut line_vec:Vec<char> = Vec::new();
        for c in line.expect("error").chars() {
            line_vec.push(c);
            write!(stdout, "{}", c);
        }
        buffer.push(line_vec);
        write!(stdout, "{}","\r\n");
    }

    stdout.flush().unwrap();

    for evt in stdin.events() {
        if evt.unwrap() == Event::Key(Key::Ctrl('c')){
            return;
        }
        stdout.flush().unwrap();
    }
}