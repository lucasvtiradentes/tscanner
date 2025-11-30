pub fn get_line_col(source: &str, byte_pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in source.char_indices() {
        if i >= byte_pos {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

pub fn get_span_positions(
    source: &str,
    start_byte: usize,
    end_byte: usize,
) -> (usize, usize, usize) {
    let start = start_byte.saturating_sub(1);
    let end = end_byte.saturating_sub(1);
    let (line, col) = get_line_col(source, start);
    let (_, end_col) = get_line_col(source, end);
    (line, col, end_col)
}
