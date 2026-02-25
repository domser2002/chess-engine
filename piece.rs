#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}
#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: Color
}

pub static WHITE_PAWN: Piece = Piece {
    piece_color: Color::White,
    piece_type: PieceType::Pawn
};

pub static WHITE_KNIGHT: Piece = Piece {
    piece_color: Color::White,
    piece_type: PieceType::Knight
};

pub static WHITE_BISHOP: Piece = Piece {
    piece_color: Color::White,
    piece_type: PieceType::Bishop
};

pub static WHITE_ROOK: Piece = Piece {
    piece_color: Color::White,
    piece_type: PieceType::Rook
};

pub static WHITE_QUEEN: Piece = Piece {
    piece_color: Color::White,
    piece_type: PieceType::Queen
};

pub static WHITE_KING: Piece = Piece {
    piece_color: Color::White,
    piece_type: PieceType::King
};

pub static BLACK_PAWN: Piece = Piece {
    piece_color: Color::Black,
    piece_type: PieceType::Pawn
};

pub static BLACK_KNIGHT: Piece = Piece {
    piece_color: Color::Black,
    piece_type: PieceType::Knight
};

pub static BLACK_BISHOP: Piece = Piece {
    piece_color: Color::Black,
    piece_type: PieceType::Bishop
};

pub static BLACK_ROOK: Piece = Piece {
    piece_color: Color::Black,
    piece_type: PieceType::Rook
};

pub static BLACK_QUEEN: Piece = Piece {
    piece_color: Color::Black,
    piece_type: PieceType::Queen
};

pub static BLACK_KING: Piece = Piece {
    piece_color: Color::Black,
    piece_type: PieceType::King
};