use crate::{Piece, piece::{BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK}};
// use crate::Move;
pub struct Position {
    pub pieces: [Option<Piece>; 64],
    // for later
    // pub last_move: Move,
    // pub white_long_castle: bool,
    // pub white_short_castle: bool,
    // pub black_long_castle: bool,
    // pub black_short_castle: bool,
}

pub static START_POSITION: Position = Position {
    pieces: [Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK),
            Some(WHITE_PAWN) , Some(WHITE_PAWN)  , Some(WHITE_PAWN)  , Some(WHITE_PAWN) , Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), 
            None             , None              , None              , None             , None            , None, None, None, 
            None             , None              , None              , None             , None            , None, None, None, 
            None             , None              , None              , None             , None, None, None, None, 
            None             , None              , None              , None             , None, None, None, None, 
            Some(BLACK_PAWN) , Some(WHITE_PAWN)  , Some(WHITE_PAWN)  , Some(WHITE_PAWN) , Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), 
            Some(BLACK_ROOK) , Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK)]
};
