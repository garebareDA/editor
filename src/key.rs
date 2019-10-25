use super::cursors;

pub fn insert(buffer: &mut Vec<Vec<char>>, c: char, cursor: &mut cursors::Cursor) -> bool {
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

pub fn backspace(buffer: &mut Vec<Vec<char>>, cursor: &mut cursors::Cursor) -> bool {
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