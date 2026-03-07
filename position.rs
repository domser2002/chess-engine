use crate::{Piece, r#move::Move, piece::*, square::*};
use colored::*;
// use crate::Move;
#[derive(Copy, Clone)]
pub struct Position {
    pub pieces: [[Option<Piece>; COL_NUM as usize]; ROW_NUM as usize],
    // for later
    // pub last_move: Move,
    // pub white_long_castle: bool,
    // pub white_short_castle: bool,
    // pub black_long_castle: bool,
    // pub black_short_castle: bool,
}

pub static START_POSITION: Position = Position {
    pieces: [
        [Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK)],
        [Some(WHITE_PAWN); 8], 
        [None;8], [None;8], [None; 8], [None; 8],
        [Some(BLACK_PAWN); 8], 
        [Some(BLACK_ROOK) , Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK)]]
};

fn print_wrapper(c: char, pc: PieceColor) {
    let text = c.to_string();
    let colored = match pc {
        PieceColor::Black => text.black(),
        PieceColor::White => text.yellow(),
    };

    print!("{colored}");
}

// this will not be unit tested as it is only for UI purpose
pub fn print_position(position: Position) {
    let horizontal_line: String = std::iter::repeat('-').take(16).collect();
    let vertical_line: String = "|".to_string();
    for row_idx in (0..ROW_NUM).rev() {
        println!("{horizontal_line}");
        for col_idx in 0..COL_NUM {
            print!("{vertical_line}");
            let maybe_piece: Option<Piece> = position.pieces[row_idx as usize][col_idx as usize];
            match maybe_piece {
                Some(piece) => {
                    let piece_char = get_char(piece);
                    match piece_char {
                        Some(c) => {
                            print_wrapper(c, piece.piece_color);
                        }
                        None => {
                            print_wrapper('p', piece.piece_color);
                        }
                    }
                }
                None => {
                    print!(" ");
                }
            }
        }
        println!("{vertical_line}");
    }
    println!("{horizontal_line}");
}


// TODO:
// 1. Allow captures.
// 2. Check if move is legal.
// 3. Allow special moves- castling, promotion, en passant.
pub fn make_move(position: &mut Position, m: Move) {
    let (from_col_idx, from_row_idx) = get_col_and_row(m.from);
    let (to_col_idx, to_row_idx) = get_col_and_row(m.to);
    let maybe_piece_on_src: Option<Piece> = position.pieces[from_row_idx as usize][from_col_idx as usize];
    match maybe_piece_on_src {
        Some(src_piece) => {
            let maybe_piece_on_dest: Option<Piece> = position.pieces[to_row_idx as usize][to_col_idx as usize];
            match maybe_piece_on_dest {
                Some(_dst_piece) => {
                    panic!("Square is already occupied!");
                }
                None => {
                    position.pieces[from_row_idx as usize][from_col_idx as usize] = None;
                    position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(src_piece);
                } 
            }
        }
        None => {
            panic!("No piece on source square!");
        }
    }
}