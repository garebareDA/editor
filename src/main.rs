extern crate termion;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::cmp::min;
use std::io::{stdin, stdout, Write};
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use termion::clear;
use termion::screen::AlternateScreen;

struct Cursor {
    row: usize,
    column: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file_buffer = BufReader::new(&file);
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut buffer: Vec<Vec<char>> = Vec::new();
    let mut cursor = Cursor{row: 0, column: 0};
    let mut row_offset = 0;
    let mut col_offset = 0;

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    for line in file_buffer.lines() {
        let mut line_vec:Vec<char> = Vec::new();
        for c in line.expect("error").chars() {
            line_vec.push(c);
        }
        buffer.push(line_vec);
    }

    draw(&buffer, row_offset, col_offset, &mut stdout);
    write!(stdout, "{}", cursor::Goto(1, 1));
    stdout.flush().unwrap();

    for evt in stdin.events() {
        let (terminal_col, terminal_row) = termion::terminal_size().unwrap();
        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            }

            Event::Key(Key::Up) => {

                if cursor.row > 0 {
                    cursor.row -= 1;
                    cursor.column = min(buffer[cursor.row].len(), cursor.column);
                    if cursor.row + 2 > terminal_row as usize && row_offset > 0{
                        row_offset -= 1;
                    }
                }
            }

            Event::Key(Key::Down) => {

                if cursor.row + 1 < buffer.len() {
                    cursor.row += 1;
                    let before_cursor_column = cursor.column;
                    cursor.column = min(cursor.column, buffer[cursor.row].len());

                    if  cursor.row + 1 > terminal_row as usize  && row_offset <= buffer.len() {
                        row_offset += 1;
                    }

                    if terminal_col as usize > buffer[cursor.row].len(){
                        col_offset = 0;
                    }else if cursor.column < buffer[cursor.row - 1].len() && before_cursor_column > buffer[cursor.row].len(){
                        col_offset = buffer[cursor.row].len() - terminal_col as usize;
                    }
                }
            }

            Event::Key(Key::Left) => {
                if cursor.column > 0 {
                    cursor.column -= 1;
                    if cursor.column + 2 > terminal_col as usize && col_offset > 0{
                        col_offset -= 1;
                    }
                }
            }

            Event::Key(Key::Right) => {
                if cursor.column < buffer[cursor.row].len() {
                    cursor.column = min(cursor.column + 1, buffer[cursor.row].len());
                    if cursor.column + 1 > terminal_col as usize{
                        col_offset += 1;
                    }
                }
            }

            Event::Key(Key::Char(c)) => {
                let new_line = insert(& mut buffer, c, & mut cursor);
                if cursor.row + 2 > terminal_row as usize && new_line == true{
                    row_offset += 1;
                }
            }

            Event::Key(Key::Backspace) => {
                let new_line = backspace(& mut buffer, & mut cursor);
                if cursor.row + 2 > terminal_row as usize && row_offset > 0 && new_line == true{
                    row_offset -= 1;
                }
            }


            _ => {}
        }

        draw(&buffer, row_offset,col_offset, &mut stdout);
        write!(stdout, "{}", cursor::Goto(cursor.column as u16 +1, cursor.row as u16 + 1));
        stdout.flush().unwrap();
    }
}

fn draw(buffer:&Vec<Vec<char>>, rows:usize, cols:usize, stdout: &mut termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>) {
    let (terminal_col, terminal_row) = termion::terminal_size().unwrap();
    let mut terminal_row = min(terminal_row as usize, buffer.len());
    terminal_row += rows;
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    for i in rows..terminal_row {
        for j in 0..buffer[i].len() {

            if j + 1 > terminal_col as usize || j + cols + 1 > buffer[i].len() {
                continue;
            }

            let c = buffer[i].get(j + cols).unwrap();
            write!(stdout, "{}", c);
        }

        if i != terminal_row -1{
            write!(stdout, "{}", "\r\n");
        }
    }
}

fn insert(buffer:& mut Vec<Vec<char>>, c:char, cursor:& mut Cursor) -> bool {
    if c == '\n'{
        let rest: Vec<char> = buffer[cursor.row]
            .drain(cursor.column..)
            .collect();

        buffer.insert(cursor.row + 1, rest);

        if cursor.column == buffer[cursor.row].len(){
            cursor.row += 1;
            cursor.column = 0;
        }else{
            cursor.column = 0;
        }

        return true;
    }else if !c.is_control(){
        buffer[cursor.row].insert(cursor.column, c);
        cursor.column += 1;
    }
    return false;
}

fn backspace(buffer:& mut Vec<Vec<char>>, cursor:& mut Cursor) -> bool {
    if cursor.column == 0 &&
        cursor.row == 0
    {
        return false;
    }

    if cursor.column == 0 {
        let line = buffer.remove(cursor.row);
        cursor.row -= 1;
        buffer[cursor.row].extend(line.into_iter());
        cursor.column = buffer[cursor.row].len();
        return true;
    }else{
        cursor.column -= 1;
        buffer[cursor.row].remove(cursor.column);
        return false;
    }
}