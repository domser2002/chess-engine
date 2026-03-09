pub static ROW_NUM: u8 = 8;
pub static COL_NUM: u8 = 8;
use num_enum::TryFromPrimitive;
use strum_macros::Display;

#[repr(u8)]
#[derive(TryFromPrimitive, Display)]
pub enum Squares {
    A1 = 1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}

pub fn get_col_and_row(square: u8) -> (u8, u8) {
    if square < 1 || square > 64 {
        panic!("Invalid square!");
    }
    let column_idx: u8 = square.wrapping_sub(1) / ROW_NUM as u8;
    let row_idx = square.wrapping_sub(1) % COL_NUM;
    (column_idx, row_idx)
}

pub fn get_square_name(square: u8) -> String {
    let sq = Squares::try_from(square).expect("Invalid square!");
    sq.to_string().to_lowercase()
}