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

pub const WHITE_PAWN: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Pawn
};

pub const WHITE_KNIGHT: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Knight
};

pub const WHITE_BISHOP: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Bishop
};

pub const WHITE_ROOK: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Rook
};

pub const WHITE_QUEEN: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::Queen
};

pub const WHITE_KING: Piece = Piece {
    piece_color: PieceColor::White,
    piece_type: PieceType::King
};

pub const BLACK_PAWN: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Pawn
};

pub const BLACK_KNIGHT: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Knight
};

pub const BLACK_BISHOP: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Bishop
};

pub const BLACK_ROOK: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Rook
};

pub const BLACK_QUEEN: Piece = Piece {
    piece_color: PieceColor::Black,
    piece_type: PieceType::Queen
};

pub const BLACK_KING: Piece = Piece {
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

/// Returns Unicode chess symbols for board display.
/// All pieces (including pawns) return a character.
pub fn get_unicode_char(p: Piece) -> char {
    match (p.piece_color, p.piece_type) {
        (PieceColor::White, PieceType::King)   => '♔',
        (PieceColor::White, PieceType::Queen)  => '♕',
        (PieceColor::White, PieceType::Rook)   => '♖',
        (PieceColor::White, PieceType::Bishop) => '♗',
        (PieceColor::White, PieceType::Knight) => '♘',
        (PieceColor::White, PieceType::Pawn)   => '♙',
        (PieceColor::Black, PieceType::King)   => '♚',
        (PieceColor::Black, PieceType::Queen)  => '♛',
        (PieceColor::Black, PieceType::Rook)   => '♜',
        (PieceColor::Black, PieceType::Bishop) => '♝',
        (PieceColor::Black, PieceType::Knight) => '♞',
        (PieceColor::Black, PieceType::Pawn)   => '♟',
    }
}
