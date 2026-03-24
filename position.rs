use crate::{Piece, PieceColor, r#move::Move, piece::{self, *}, square::{self, *}};
use colored::*;
use crate::print_move;
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

pub fn square_free(row: u8, col: u8, position: Position) -> bool
{
    let maybe_piece = position.pieces[row as usize][col as usize];
    match maybe_piece {
        None => {return true;}
        Some(_p) => { return false; }
    }
}

pub fn square_occupied_not_by_color(row: u8, col: u8, position: Position, piece_color: PieceColor) -> bool
{
    let maybe_piece = position.pieces[row as usize][col as usize];
    match maybe_piece {
        None => {return false;}
        Some(p) => { return p.piece_color != piece_color; }
    }
}

fn try_move(dest_row_idx: i8, dest_col_idx: i8, square:u8, position: Position, piece: Piece, v: &mut Vec<Move>) -> bool {
    if dest_row_idx < (ROW_NUM as i8) && dest_col_idx < (COL_NUM as i8) && dest_row_idx >= 0 && dest_col_idx >= 0 {
        if square_free(dest_row_idx as u8, dest_col_idx as u8, position) {
            let possible_move = Move::build(piece, square, get_square_idx(dest_row_idx as u8, dest_col_idx as u8));
            v.push(possible_move);
            return true;
        }
        else if square_occupied_not_by_color(dest_row_idx as u8, dest_col_idx as u8, position, piece.piece_color) {
            let possible_move = Move::build(piece, square, get_square_idx(dest_row_idx as u8, dest_col_idx as u8));
            v.push(possible_move);
            return true;
        }
    }
    false  
}

fn try_diagonal_moves(row_idx: u8, col_idx: u8, square:u8, position: Position, piece: Piece, v: &mut Vec<Move>) {
    let mut success = true;
    let mut i= 1;
    while success {
        success = try_move((row_idx as i8) + i, (col_idx as i8) + i, square, position, piece, v);
        i += 1;
    }
    success = true;
    i = 1; 
    while success {
        success = try_move((row_idx as i8) + i, (col_idx as i8) - i, square, position, piece, v);
        i += 1;
    }

    success = true;
    i = 1; 
    while success {
        success = try_move((row_idx as i8) - i, (col_idx as i8) + i, square, position, piece, v);
        i += 1;
    }

    success = true;
    i = 1; 
    while success {
        success = try_move((row_idx as i8) - i, (col_idx as i8) - i, square, position, piece, v);
        i += 1;
    }
}

fn try_straight_moves(row_idx: u8, col_idx: u8, square:u8, position: Position, piece: Piece, v: &mut Vec<Move>) {
    let mut success = true;
    let mut i= 1;
    while success {
        success = try_move((row_idx as i8) + i, col_idx as i8, square, position, piece, v);
        i += 1;
    }
    success = true;
    i = 1; 
    while success {
        success = try_move((row_idx as i8) - i, col_idx as i8, square, position, piece, v);
        i += 1;
    }

    success = true;
    i = 1; 
    while success {
        success = try_move(row_idx as i8, (col_idx as i8) + i, square, position, piece, v);
        i += 1;
    }

    success = true;
    i = 1; 
    while success {
        success = try_move(row_idx as i8, (col_idx as i8) - i, square, position, piece, v);
        i += 1;
    }
}

pub fn legal_moves(position: Position) -> Vec<Move> {
    let mut v = Vec::new();
    for square in 1..=(ROW_NUM * COL_NUM) {
        let (col_idx, row_idx) = get_col_and_row(square);
        let maybe_piece = position.pieces[row_idx as usize][col_idx as usize];
        match maybe_piece {
            None => {}
            Some(piece) => {
                if position.turn == piece.piece_color {
                    match piece.piece_type {
                        PieceType::Pawn => {
                            let mut direction: i8 = 1;
                            if piece.piece_color == PieceColor::Black {
                                direction = -1;
                            }
                            let dest_row_idx = ((row_idx as i8) + (1 as i8) * direction) as u8;
                            let dest_col_idx = col_idx;
                            if square_free(dest_row_idx, dest_col_idx, position) {
                                let possible_move: Move = Move::build(piece, square, get_square_idx(dest_row_idx, dest_col_idx));
                                v.push(possible_move);
                            }
                            if (row_idx == 1 && direction == 1) || (row_idx == 6 && direction == -1) {
                                let dest_row_idx = ((row_idx as i8) + (2 as i8) * direction) as u8;
                                let dest_col_idx = col_idx;
                                if square_free(dest_row_idx, dest_col_idx, position) {
                                    let possible_move: Move = Move::build(piece, square, get_square_idx(dest_row_idx, dest_col_idx));
                                    v.push(possible_move);
                                }
                            }
                            if col_idx < COL_NUM - 1 {
                                let dest_row_idx = ((row_idx as i8) + (1 as i8) * direction) as u8;
                                let dest_col_idx = col_idx + 1;
                                if square_occupied_not_by_color(dest_row_idx, dest_col_idx, position, piece.piece_color) {
                                    let possible_move: Move = Move::build(piece, square, get_square_idx(dest_row_idx, dest_col_idx)).capture();
                                    v.push(possible_move);
                                }
                            }
                            if col_idx > 0 {
                                let dest_row_idx = ((row_idx as i8) + (1 as i8) * direction) as u8;
                                let dest_col_idx = col_idx - 1;
                                if square_occupied_not_by_color(dest_row_idx, dest_col_idx, position, piece.piece_color) {
                                    let possible_move: Move = Move::build(piece, square, get_square_idx(dest_row_idx, dest_col_idx)).capture();
                                    v.push(possible_move);
                                }
                            }
                        }
                        PieceType::Knight => {
                            try_move((row_idx as i8) + 2, (col_idx as i8) + 1, square, position, piece, &mut v);
                            try_move((row_idx as i8) + 1, (col_idx as i8) + 2, square, position, piece, &mut v);
                            try_move((row_idx as i8) - 2, (col_idx as i8) + 1, square, position, piece, &mut v);
                            try_move((row_idx as i8) - 1, (col_idx as i8) + 2, square, position, piece, &mut v);
                            try_move((row_idx as i8) + 2, (col_idx as i8) - 1, square, position, piece, &mut v);
                            try_move((row_idx as i8) + 1, (col_idx as i8) - 2, square, position, piece, &mut v);
                            try_move((row_idx as i8) - 2, (col_idx as i8) - 1, square, position, piece, &mut v);
                            try_move((row_idx as i8) - 1, (col_idx as i8) - 2, square, position, piece, &mut v);
                        }
                        PieceType::Bishop => {
                            try_diagonal_moves(row_idx, col_idx, square, position, piece, &mut v);
                        }
                        PieceType::Rook => {
                            try_straight_moves(row_idx, col_idx, square, position, piece, &mut v);
                        }
                        PieceType::Queen => {
                            try_diagonal_moves(row_idx, col_idx, square, position, piece, &mut v);
                            try_straight_moves(row_idx, col_idx, square, position, piece, &mut v);
                        }
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
    if position.turn == PieceColor::White {
        position.turn = PieceColor::Black;
    }
    else {
        position.turn = PieceColor::White;
    }
}