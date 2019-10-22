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
use termion::color;

struct Cursor {
    row: usize,
    column: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("file not found");
    let file_buffer = BufReader::new(&file);
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut buffer: Vec<Vec<char>> = Vec::new();
    let mut cursor = Cursor{row: 0, column: 6};
    let mut row_offset = 0;
    let mut col_offset = 0;
    let mut clear:bool = true;

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    for line in file_buffer.lines() {
        let mut line_vec:Vec<char> = Vec::new();
        for c in line.expect("error").chars() {
            line_vec.push(c);
        }
        buffer.push(line_vec);
    }

    if buffer.len() == 0{
        let rest: Vec<char> = Vec::new();
        buffer.insert(0, rest);
    }

    draw(&buffer, row_offset, col_offset, &mut stdout, clear);
    write!(stdout, "{}", cursor::Goto(7, 1));
    stdout.flush().unwrap();

    for evt in stdin.events() {
        let (terminal_col, terminal_row) = termion::terminal_size().unwrap();
        let terminal_col = terminal_col as usize;
        let terminal_row = terminal_row as usize;

        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            }

            Event::Key(Key::Up) => {

                if cursor.row > 0 {
                    clear = false;
                    cursor.row -= 1;
                    let before_cursor_column = cursor.column;
                    cursor.column = min(buffer[cursor.row].len() + 6, cursor.column);

                    let buffer_row_len = buffer[cursor.row].len() + 6;

                    if cursor.row + 2 > terminal_row && row_offset > 0{
                        row_offset -= 1;
                        clear = true;
                    }

                    if terminal_col > buffer_row_len{
                        col_offset = 0;
                        clear = true;
                    }else if cursor.column < buffer[cursor.row + 1].len() + 6 && before_cursor_column> buffer_row_len{
                        col_offset = (buffer_row_len - terminal_col) + 1;
                        clear = true;
                    }
                }
            }

            Event::Key(Key::Down) => {
                if cursor.row + 1 < buffer.len() {
                    clear = false;
                    cursor.row += 1;
                    let before_cursor_column = cursor.column;
                    cursor.column = min(cursor.column, buffer[cursor.row].len() + 6);
                    let buffer_row_len = buffer[cursor.row].len() + 6;

                    if  cursor.row + 1 > terminal_row   && row_offset <= buffer.len() {
                        row_offset += 1;
                        clear = true;
                    }

                    if terminal_col  > buffer_row_len{
                        col_offset = 0;
                        clear = true;
                    }else if cursor.column < buffer[cursor.row - 1].len() + 6 && before_cursor_column > buffer_row_len{
                        col_offset = (buffer_row_len - terminal_col) + 1;
                        clear = true;
                    }
                }
            }

            Event::Key(Key::Left) => {
                if cursor.column > 6 {
                    clear = false;
                    cursor.column -= 1;
                    if cursor.column + 2 > terminal_col  && col_offset > 0{
                        col_offset -= 1;
                        clear = true;
                    }
                }
            }

            Event::Key(Key::Right) => {
                clear = false;
                let buffer_row_len = buffer[cursor.row].len() + 6;

                if  buffer.len() + 6 > 0 && cursor.column < buffer_row_len{
                    cursor.column = min(cursor.column + 1, buffer_row_len);
                    if cursor.column + 1 > terminal_col {
                        col_offset += 1;
                        clear = true;
                    }
                }
            }

            Event::Key(Key::Char(c)) => {
                let new_line = insert(& mut buffer, c, & mut cursor);
                clear = true;

                if cursor.row + 1 > terminal_row  && new_line == true{
                    row_offset += 1;
                }else if new_line == true{
                    col_offset = 0;
                }else{
                    clear = false;
                }

                if cursor.column + 1 > terminal_col {
                    col_offset += 1;
                    clear = true;
                }
            }

            Event::Key(Key::Backspace) => {
                let new_line = backspace(& mut buffer, & mut cursor);
                clear = true;
                if cursor.row + 2 > terminal_row  && row_offset > 0 && new_line == true{
                    row_offset -= 1;
                }

                if cursor.column + 2> terminal_col  && new_line == false{
                    col_offset -= 1;
                }else if new_line == true && cursor.column + 2> terminal_col {
                    col_offset = (buffer[cursor.row].len() + 6 - terminal_col ) + 1;
                }
            }

            Event::Key(Key::Ctrl('s')) => {
                save(& buffer, &args[1]);
            }

            _ => {}
        }

        draw(&buffer, row_offset,col_offset, &mut stdout, clear);
        write!(stdout, "{}", cursor::Goto(cursor.column as u16 +1, cursor.row as u16 + 1));
        stdout.flush().unwrap();
    }
}

fn draw(buffer:&Vec<Vec<char>>, rows:usize, cols:usize, stdout: &mut termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>, clear: bool) {
    let (terminal_col, terminal_row) = termion::terminal_size().unwrap();
    let terminal_col = terminal_col - 6;
    let mut terminal_row = min(terminal_row as usize, buffer.len());
    terminal_row += rows;

    if clear {
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    }else{
         write!(stdout, "{}{}", termion::clear::CurrentLine, termion::cursor::Goto(1, 1)).unwrap();
    }

    for i in rows..terminal_row {

        let digit = i + 1;
        let digit_index = digit.to_string().len();
        let st:&str;

        match digit_index {
            1 => {st = "   "}
            2 => {st = "  "}
            3 => {st = " "}
            _ => {st = ""}
        }

        write!(stdout,"{}{}{}  ", color::Bg(color::LightBlack), st, i + 1) ;

        for j in 0..buffer[i].len() {

            if j + 1 > terminal_col as usize || j + cols + 1 > buffer[i].len() {
                continue;
            }

            let c = buffer[i].get(j + cols).unwrap();
            write!(stdout, "{}{}",color::Bg(color::Reset), c);
        }

        if i != terminal_row -1{
            write!(stdout, "{}", "\r\n");
        }
    }
}

fn insert(buffer:& mut Vec<Vec<char>>, c:char, cursor:& mut Cursor) -> bool {
    if c == '\n'{
        let rest: Vec<char> = buffer[cursor.row]
        .drain(cursor.column - 6..)
        .collect();
        buffer.insert(cursor.row + 1, rest);

        if cursor.column - 6 == buffer[cursor.row].len() {
            cursor.row += 1;
            cursor.column = 6;
        }else{
            cursor.column = 6;
        }

        return true;
    }else if !c.is_control(){

        if buffer.len() != 0 {
            buffer[cursor.row].insert(cursor.column - 6, c);
            cursor.column += 1;
        }else{
            let mut rest: Vec<char> = Vec::new();
            rest.push(c);
            buffer.insert(0, rest);
            cursor.column += 1;
        }
    }
    return false;
}

fn backspace(buffer:& mut Vec<Vec<char>>, cursor:& mut Cursor) -> bool {
    if cursor.column == 6 &&
        cursor.row == 0
    {
        return false;
    }

    if cursor.column == 6 {
        let line = buffer.remove(cursor.row);
        cursor.row -= 1;
        buffer[cursor.row].extend(line.into_iter());
        cursor.column = buffer[cursor.row].len() + 6;
        return true;
    }else{
        cursor.column -= 1;
        buffer[cursor.row].remove(cursor.column - 6);
        return false;
    }
}

fn save(buffer:& Vec<Vec<char>>, path:&std::string::String) {
    if let Ok(mut file) = File::create(path) {
        for line in buffer {
            for c in line {
                write!(file, "{}", c).unwrap();
            }
            writeln!(file).unwrap();
        }
    }
}