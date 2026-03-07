pub static ROW_NUM: u8 = 8;
pub static COL_NUM: u8 = 8;

pub fn get_col_and_row(square: u8) -> (u8, u8) {
    if square < 1 || square > 64 {
        panic!("Invalid square!");
    }
    let column_idx: u8 = square.wrapping_sub(1) / ROW_NUM as u8;
    let row_idx = square.wrapping_sub(1) % COL_NUM;
    (column_idx, row_idx)
}

pub fn get_square_name(square: u8) -> String {
    let (column_idx, row_idx) = get_col_and_row(square);
    let mut s = String::new();
    let column = (('a' as u8) + (column_idx)) as char;
    s.push(column);
    let row = row_idx + 1;
    s.push_str(&row.to_string());
    s
}