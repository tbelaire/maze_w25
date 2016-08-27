
// The screen in 1 indexed, not zero indexed.
pub fn move_cursor(row: usize, col: usize) {
    print!("\x1B[{row};{col}f", row = 1 + row, col = 1 + col);
}
