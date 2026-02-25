#[cfg(test)]
pub mod tests {
    use crate::r#move::Move;
    use crate::Piece;
    use crate::r#move::print_move;
    use crate::piece::{BLACK_BISHOP, BLACK_PAWN, WHITE_KING, WHITE_KNIGHT, WHITE_QUEEN, WHITE_ROOK};
    // use ntest::timeout;

    #[test]
    fn rook_move_simple_square() {
        let m = Move {piece: WHITE_ROOK, square: 1};
        let s = print_move(m);
        assert_eq!(s, "Ra1");
    }
    
    #[test]
    fn knight_move() {
        let m = Move {piece: WHITE_KNIGHT, square: 43};
        let s = print_move(m);
        assert_eq!(s, "Nf3");
    }

    #[test]
    fn bishop_move() {
        let m = Move {piece: BLACK_BISHOP, square: 20};
        let s = print_move(m);
        assert_eq!(s, "Bc4");
    }

    #[test]
    fn queen_move() {
        let m = Move {piece: WHITE_QUEEN, square: 61};
        let s = print_move(m);
        assert_eq!(s, "Qh5");
    }

    #[test]
    fn king_move() {
        let m = Move {piece: WHITE_KING, square: 41};
        let s = print_move(m);
        assert_eq!(s, "Kf1");
    }

    #[test]
    fn pawn_move() {
        let m = Move {piece: BLACK_PAWN, square: 36};
        let s = print_move(m);
        assert_eq!(s, "e4");
    }
}