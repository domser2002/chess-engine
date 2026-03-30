use std::char;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum PieceColor {
    White,
    Black
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor
}

pub static WHITE_PAWN: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Pawn
};

pub static WHITE_KNIGHT: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Knight
};

pub static WHITE_BISHOP: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Bishop
};

pub static WHITE_ROOK: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Rook
};

pub static WHITE_QUEEN: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Queen
};

pub static WHITE_KING: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::King
};

pub static BLACK_PAWN: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Pawn
};

pub static BLACK_KNIGHT: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Knight
};

pub static BLACK_BISHOP: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Bishop
};

pub static BLACK_ROOK: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Rook
};

pub static BLACK_QUEEN: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Queen
};

pub static BLACK_KING: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::King
};

pub fn get_char(p: Piece) -> Option<char> {
    match p.piece_type {
        PieceType::Pawn => return None,
        PieceType::Knight => return Some('N'),
        PieceType::Bishop => return Some('B'),
        PieceType::Rook => return Some('R'),
        PieceType::Queen => return Some('Q'),
        PieceType::King => return Some('K')
    }
}