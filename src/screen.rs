
pub fn move_cursor(row: usize, col: usize) {
    print!("\x1B7\x1B[{row};{col}f", row = row, col = col);
}
