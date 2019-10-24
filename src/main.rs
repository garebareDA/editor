//TODO　ファイルを分割する

extern crate termion;

use std::cmp::min;
use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::io::{BufRead, BufReader};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;


struct Cursor {
    row: usize,
    column: usize,
}

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
    let mut cursor = Cursor {
        row: 0,
        column: terminal_col_resize,
    };
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

    draw(&buffer, row_offset, col_offset, &mut stdout, clear);
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
                    let new_line = insert(&mut buffer, c, &mut cursor);
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
                    let new_line = backspace(&mut buffer, &mut cursor);
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
                    save(&buffer, path);
                }

                Event::Key(Key::Ctrl('w')) => {
                    if mode == "text" {
                        mode = "file";
                        cursor.column = 0;
                        cursor.row = 1;
                        col_offset = 0;
                        row_offset = 0;
                        file_view = files(&mut target);
                    }
                }

                _ => {}
            }
        } else {

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
                        let (file_view_tmp, target_tmp) = file_open(&mut file_view, &mut cursor);
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
                        file_view = files(&mut target);
                        cursor.row = 1;
                        cursor.column = 0;
                    }
                }

                _ => {}
            }
        }

        if mode == "text" {
            draw(&buffer, row_offset, col_offset, &mut stdout, clear);
            write!(
                stdout,
                "{}",
                cursor::Goto(cursor.column as u16 + 1, cursor.row as u16 + 1)
            );
        } else {
            draw_file(&mut stdout, &mut file_view, &mut target , row_offset, path);
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

fn draw(
    buffer: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    stdout: &mut termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
    clear: bool,
) {
    let (terminal_col, terminal_row) = termion::terminal_size().unwrap();
    let terminal_col_resize = 6;
    let terminal_col = terminal_col - terminal_col_resize;
    let mut terminal_row = min(terminal_row as usize, buffer.len());
    terminal_row += rows;

    if clear {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
    } else {
        write!(
            stdout,
            "{}{}",
            termion::clear::CurrentLine,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
    }

    for i in rows..terminal_row {
        let digit = i + 1;
        let digit_index = digit.to_string().len();
        let st: &str;

        match digit_index {
            1 => st = "   ",
            2 => st = "  ",
            3 => st = " ",
            _ => st = "",
        }

        write!(stdout, "{}{}{}| ", color::Fg(color::LightGreen), st, i + 1);

        for j in 0..buffer[i].len() {
            if j + 1 > terminal_col as usize || j + cols + 1 > buffer[i].len() {
                continue;
            }

            let c = buffer[i].get(j + cols).unwrap();
            write!(stdout, "{}{}", color::Fg(color::Reset), c);
        }

        if i != terminal_row - 1 {
            write!(stdout, "{}", "\r\n");
        }
    }
}

fn insert(buffer: &mut Vec<Vec<char>>, c: char, cursor: &mut Cursor) -> bool {
    let terminal_col_resize = 6;
    if c == '\n' {
        let rest: Vec<char> = buffer[cursor.row]
            .drain(cursor.column - terminal_col_resize..)
            .collect();
        buffer.insert(cursor.row + 1, rest);

        if cursor.column - terminal_col_resize == buffer[cursor.row].len() {
            cursor.row += 1;
            cursor.column = terminal_col_resize;
        } else {
            cursor.column = terminal_col_resize;
        }

        return true;
    } else if !c.is_control() {
        if buffer.len() != 0 {
            buffer[cursor.row].insert(cursor.column - terminal_col_resize, c);
            cursor.column += 1;
        } else {
            let mut rest: Vec<char> = Vec::new();
            rest.push(c);
            buffer.insert(0, rest);
            cursor.column += 1;
        }
    }
    return false;
}

fn backspace(buffer: &mut Vec<Vec<char>>, cursor: &mut Cursor) -> bool {
    let terminal_col_resize = 6;
    if cursor.column == terminal_col_resize && cursor.row == 0 {
        return false;
    }

    if cursor.column == terminal_col_resize {
        let line = buffer.remove(cursor.row);
        cursor.row -= 1;
        buffer[cursor.row].extend(line.into_iter());
        cursor.column = buffer[cursor.row].len() + terminal_col_resize;
        return true;
    } else {
        cursor.column -= 1;
        buffer[cursor.row].remove(cursor.column - terminal_col_resize);
        return false;
    }
}

fn draw_file(
    stdout: &mut termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
    file_view: &mut Vec<String>,
    target: &str,
    rows: usize,
    path:&String
) {
    let (_, terminal_row) = termion::terminal_size().unwrap();
    let mut terminal_row = min((terminal_row - 1) as usize, file_view.len());
    terminal_row += rows;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    write!(stdout, "{}{} " , color::Fg(color::Blue), path);
    write!(stdout,"{}{}{}{}{}{}", color::Fg(color::Black),color::Bg(color::White),target, "\r\n",color::Bg(color::Reset),color::Fg(color::Reset));

    for i in rows..terminal_row{
        let path = file_view.get(i).unwrap();
        let md = metadata(&path).unwrap();
        let path_repace = path.replacen(target, "", 1);
        if md.is_dir() {
            write!(stdout, "{}>{}", color::Fg(color::Green),path_repace);
        } else {
            write!(stdout, "{}-{}", color::Fg(color::Reset),path_repace);
        }

        if i != terminal_row - 1{
            write!(stdout,"\r\n");
        }
    }
}

fn files(target:&str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for path in fs::read_dir(target).unwrap() {
        files.push(path.unwrap().path().display().to_string())
    }
    files
}

fn file_open(file_view: &mut Vec<String>, cursor:&mut Cursor) -> (Vec<String>, String) {
    let target = file_view[cursor.row - 1].to_string();
    let file_opend = files(&target);
    cursor.row = 1;
    return (file_opend, target);
}

fn save(buffer: &Vec<Vec<char>>, path: &std::string::String) {
    if let Ok(mut file) = File::create(path) {
        for line in buffer {
            for c in line {
                write!(file, "{}", c).unwrap();
            }
            writeln!(file).unwrap();
        }
    }
}
