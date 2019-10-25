pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

pub fn new(row: usize, column: usize) -> Cursor {
        return Cursor {row, column};
}