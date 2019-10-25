extern crate termion;

use std::cmp::min;
use std::fs::metadata;
use std::io::Write;
use termion::color;

pub fn draw_text(
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

pub fn draw_file(
    stdout: &mut termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
    file_view: &mut Vec<String>,
    target: &str,
    rows: usize,
    path: &String,
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

    write!(stdout, "{}{} ", color::Fg(color::Blue), path);
    write!(
        stdout,
        "{}{}{}{}{}{}",
        color::Fg(color::Black),
        color::Bg(color::White),
        target,
        "\r\n",
        color::Bg(color::Reset),
        color::Fg(color::Reset)
    );

    for i in rows..terminal_row {
        let path = file_view.get(i).unwrap();
        let md = metadata(&path).unwrap();
        let path_repace = path.replacen(target, "", 1);
        if md.is_dir() {
            write!(stdout, "{}>{}", color::Fg(color::Green), path_repace);
        } else {
            write!(stdout, "{}-{}", color::Fg(color::Reset), path_repace);
        }

        if i != terminal_row - 1 {
            write!(stdout, "\r\n");
        }
    }
}
