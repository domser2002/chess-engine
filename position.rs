use crate::{Piece, PieceColor, r#move::Move, piece::*, square::{self, *}};
use colored::*;
// use crate::Move;
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub pieces: [[Option<Piece>; COL_NUM as usize]; ROW_NUM as usize],
    pub turn: PieceColor,
    // for later
    // pub last_move: Move,
    // pub white_long_castle: bool,
    // pub white_short_castle: bool,
    // pub black_long_castle: bool,
    // pub black_short_castle: bool,
}

pub fn start_position() -> Position {
 Position {
    pieces: [
        [Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK)],
        [Some(WHITE_PAWN); 8], 
        [None;8], [None;8], [None; 8], [None; 8],
        [Some(BLACK_PAWN); 8], 
        [Some(BLACK_ROOK) , Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK)]],
        turn: PieceColor::White
    }
}

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

pub fn square_free(square: u8, position: Position) -> bool
{
    let (col,row) = get_col_and_row(square);
    let maybe_piece = position.pieces[row as usize][col as usize];
    match maybe_piece {
        None => {return true;}
        Some(_p) => { return false; }
    }
}

pub fn legal_moves(position: Position) -> Vec<Move> {
    let v = Vec::new();
    for square in 0..(ROW_NUM * COL_NUM) {
        let (col_idx, row_idx) = get_col_and_row(square);
        let maybe_piece = position.pieces[row_idx as usize][col_idx as usize];
        match maybe_piece {
            None => {}
            Some(piece) => {
                if position.turn == piece.piece_color {
                    match piece.piece_type {
                        PieceType::Pawn => {
                            if square_free(square + 1, position) { // not +1

                            }
                        }
                        PieceType::Knight => {}
                        PieceType::Bishop => {}
                        PieceType::Rook => {}
                        PieceType::Queen => {}
                        PieceType::King => {}
                    }
                }
            }
        }
    }
    v
}

// TODO:
// 1. Check if move is legal.
// 2. Allow special moves- castling, promotion, en passant.
pub fn make_move(position: &mut Position, m: Move) {
    let (from_col_idx, from_row_idx) = get_col_and_row(m.from);
    let (to_col_idx, to_row_idx) = get_col_and_row(m.to);
    let maybe_piece_on_src: Option<Piece> = position.pieces[from_row_idx as usize][from_col_idx as usize];
    match maybe_piece_on_src {
        Some(src_piece) => {
            let maybe_piece_on_dest: Option<Piece> = position.pieces[to_row_idx as usize][to_col_idx as usize];
            match maybe_piece_on_dest {
                Some(dst_piece) => {
                    if dst_piece.piece_color == src_piece.piece_color {
                        panic!("Square is already occupied!");
                    }
                    else {}
                }
                None => {} 
            }
            position.pieces[from_row_idx as usize][from_col_idx as usize] = None;
            position.pieces[to_row_idx as usize][to_col_idx as usize] = Some(src_piece);
        }
        None => {
            panic!("No piece on source square!");
        }
    }
}