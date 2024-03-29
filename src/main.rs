extern crate termion;

use std::cmp::min;
use std::env;
use std::fs::metadata;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::io::{BufRead, BufReader};
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

mod draw;
mod file;
mod key;
mod cursors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let history = Vec::new();
    let (mut path_conti, mut conti, mut history_conti) = editor(path, history);

    loop {
        if conti{
           let (path_tmp, conti_tmp, history_tmp) = editor(&path_conti, history_conti);
           path_conti = path_tmp;
           conti = conti_tmp;
           history_conti = history_tmp;
        }else{
            return
        }
    }
}

fn editor(path:&String, history: Vec<String>) -> (String, bool, Vec<String>) {
    let file = File::open(path).expect("file not found");
    let file_buffer = BufReader::new(&file);
    let stdin = stdin();
    let mut target = "./".to_string();
    let terminal_col_resize = 6;
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut buffer: Vec<Vec<char>> = Vec::new();
    let mut cursor = cursors::new(0,terminal_col_resize);
    let mut row_offset = 0;
    let mut col_offset = 0;
    let mut clear: bool = true;
    let mut mode = "text";
    let mut file_view:Vec<String> = Vec::new();
    let mut file_history:Vec<String>;

    if history.len() < 1{
        file_history = Vec::new();
        file_history.push("./".to_string());
    }else{
        file_history = history.clone();
        target = history[history.len() - 1].clone();
    }

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    for line in file_buffer.lines() {
        let mut line_vec: Vec<char> = Vec::new();
        for c in line.expect("error").chars() {
            line_vec.push(c);
        }
        buffer.push(line_vec);
    }

    if buffer.len() == 0 {
        let rest: Vec<char> = Vec::new();
        buffer.insert(0, rest);
    }

    draw::draw_text(&buffer, row_offset, col_offset, &mut stdout, clear);
    write!(stdout, "{}", cursor::Goto(7, 1));
    stdout.flush().unwrap();

    for evt in stdin.events() {
        let (terminal_col, terminal_row) = termion::terminal_size().unwrap();
        let terminal_col = terminal_col as usize;
        let terminal_row = terminal_row as usize;

        if mode == "text" {
            match evt.unwrap() {
                Event::Key(Key::Ctrl('c')) => {
                    let st = " ".to_string();
                    let vec:Vec<String> = Vec::new();
                    return (st,  false, vec);
                }

                Event::Key(Key::Up) => {
                    if cursor.row > 0 {
                        cursor.row -= 1;
                        let before_cursor_column = cursor.column;
                        cursor.column = min(
                            buffer[cursor.row].len() + terminal_col_resize,
                            cursor.column,
                        );

                        let buffer_row_len = buffer[cursor.row].len() + terminal_col_resize;
                        clear = false;

                        if cursor.row + 2 > terminal_row && row_offset > 0 {
                            row_offset -= 1;
                            clear = true;
                        }

                        if terminal_col > buffer_row_len {
                            col_offset = 0;
                            clear = true;
                        } else if cursor.column < buffer[cursor.row + 1].len() + terminal_col_resize
                            && before_cursor_column > buffer_row_len
                        {
                            col_offset = (buffer_row_len - terminal_col) + 1;
                            clear = true;
                        }
                    }
                }

                Event::Key(Key::Down) => {
                    if cursor.row + 1 < buffer.len() {
                        cursor.row += 1;
                        let before_cursor_column = cursor.column;
                        cursor.column = min(
                            cursor.column,
                            buffer[cursor.row].len() + terminal_col_resize,
                        );
                        let buffer_row_len = buffer[cursor.row].len() + terminal_col_resize;
                        clear = false;

                        if cursor.row + 1 > terminal_row && row_offset <= buffer.len() {
                            row_offset += 1;
                            clear = true;
                        }

                        if terminal_col > buffer_row_len {
                            col_offset = 0;
                            clear = true;
                        } else if cursor.column < buffer[cursor.row - 1].len() + 6
                            && before_cursor_column > buffer_row_len
                        {
                            col_offset = (buffer_row_len - terminal_col) + 1;
                            clear = true;
                        }
                    }
                }

                Event::Key(Key::Left) => {
                    if cursor.column > terminal_col_resize {
                        cursor.column -= 1;
                        clear = false;
                        if cursor.column + 2 > terminal_col && col_offset > 0 {
                            col_offset -= 1;
                            clear = true;
                        }
                    }
                }

                Event::Key(Key::Right) => {
                    let buffer_row_len = buffer[cursor.row].len() + terminal_col_resize;
                    clear = false;

                    if buffer.len() + terminal_col_resize > 0 && cursor.column < buffer_row_len {
                        cursor.column = min(cursor.column + 1, buffer_row_len);
                        if cursor.column + 1 > terminal_col {
                            col_offset += 1;
                            clear = true;
                        }
                    }
                }

                Event::Key(Key::Char(c)) => {
                    let new_line = key::insert(&mut buffer, c, &mut cursor);
                    clear = new_line;

                    if cursor.row + 1 > terminal_row && new_line == true {
                        row_offset += 1;
                    } else if new_line == true {
                        col_offset = 0;
                    }
                    if cursor.column + 1 > terminal_col {
                        col_offset += 1;
                        clear = true;
                    }
                }

                Event::Key(Key::Backspace) => {
                    let new_line = key::backspace(&mut buffer, &mut cursor);
                    clear = new_line;
                    if cursor.row + 2 > terminal_row && row_offset > 0 && new_line == true {
                        row_offset -= 1;
                    }

                    if cursor.column + 2 > terminal_col && new_line == false {
                        col_offset -= 1;
                    } else if new_line == true && cursor.column + 2 > terminal_col {
                        col_offset =
                            (buffer[cursor.row].len() + terminal_col_resize - terminal_col) + 1;
                    }
                }

                Event::Key(Key::Ctrl('s')) => {
                    file::save(&buffer, path);
                }

                Event::Key(Key::Ctrl('w')) => {
                    if mode == "text" {
                        mode = "file";
                        cursor.column = 0;
                        cursor.row = 1;
                        col_offset = 0;
                        row_offset = 0;
                        file_view = file::files_view(&mut target);
                    }
                }

                _ => {}
            }
        } else if mode == "file"{

            match evt.unwrap() {
                Event::Key(Key::Ctrl('w')) => {
                    if mode == "file" {
                        mode = "text";
                        cursor.row = 0;
                        cursor.column = terminal_col_resize;
                        col_offset = 0;
                        row_offset = 0;
                        clear = true;
                    }
                }

                Event::Key(Key::Up) => {
                    if cursor.row > 1 {
                        cursor.row -= 1;
                        if cursor.row + 2 > terminal_row - 1 && row_offset > 0 {
                            row_offset -= 1;

                        }
                    }
                }

                Event::Key(Key::Down) => {
                    if cursor.row < file_view.len() {
                        cursor.row += 1;
                        if cursor.row + 2 > terminal_row + 1 && row_offset <= file_view.len() - 1{
                            row_offset += 1;
                         }
                    }
                }

                Event::Key(Key::Char(c)) => {
                    let md = metadata(&file_view[cursor.row - 1].to_string()).unwrap();
                    if c == '\n' && md.is_dir() == true {
                        let (file_view_tmp, target_tmp) = file::file_open(&mut file_view, &mut cursor);
                        let tmp = target_tmp.clone();
                        file_history.push(target_tmp);
                        file_view = file_view_tmp;
                        target = tmp;
                        row_offset = 0;
                    } else if c=='\n' && md.is_dir() == false {
                        let file_path_history = file_history.clone();
                        let file_path_view = file_view[cursor.row - 1].clone();

                        return (file_path_view, true, file_path_history);
                    }
                }

                Event::Key(Key::Backspace) => {

                    if file_history.len() > 1 {
                        file_history.remove(file_history.len() - 1);
                        target = file_history[file_history.len() - 1].clone();
                        file_view = file::files_view(&mut target);
                        cursor.row = 1;
                        cursor.column = 0;
                    }
                }

                _ => {}
            }
        }

        if mode == "text" {
            draw::draw_text(&buffer, row_offset, col_offset, &mut stdout, clear);
            write!(
                stdout,
                "{}",
                cursor::Goto(cursor.column as u16 + 1, cursor.row as u16 + 1)
            );
        } else if mode == "file"{
            draw::draw_file(&mut stdout, &mut file_view, &mut target , row_offset, path);
            write!(
                stdout,
                "{}",
                cursor::Goto(cursor.column as u16 + 1, cursor.row as u16 + 1)
            );
        }

        stdout.flush().unwrap();
    }

    let st = " ".to_string();
    let vec:Vec<String> = Vec::new();
    return (st,  false, vec);
}