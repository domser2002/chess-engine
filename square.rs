pub static ROW_NUM: u8 = 8;
pub static COL_NUM: u8 = 8;

pub fn get_square_name(square: u8) -> String {
    if square < 1 || square > 64 {
        panic!("Invalid square!");
    }
    let mut s = String::new();
    let column_idx: u8 = square.wrapping_sub(1) / ROW_NUM as u8;
    let column = (('a' as u8) + (column_idx)) as char;
    s.push(column);
    let row_idx = square.wrapping_sub(1) % COL_NUM + 1;
    s.push_str(&row_idx.to_string());
    s
}