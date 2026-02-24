#[cfg(test)]
pub mod tests {
    use crate::domain::Move;
    use crate::domain::Piece;
    use crate::domain::print_move;
    // use ntest::timeout;

    #[test]
    fn rook_move_simple_square() {
        let m = Move {piece: Piece::Rook, square: 1};
        let s = print_move(m);
        assert_eq!(s, "Ra1");
    }
    
    #[test]
    fn knight_move() {
        let m = Move {piece: Piece::Knight, square: 43};
        let s = print_move(m);
        assert_eq!(s, "Nf3");
    }

    #[test]
    fn bishop_move() {
        let m = Move {piece: Piece::Bishop, square: 20};
        let s = print_move(m);
        assert_eq!(s, "Bc4");
    }

    #[test]
    fn queen_move() {
        let m = Move {piece: Piece::Queen, square: 61};
        let s = print_move(m);
        assert_eq!(s, "Qh5");
    }

    #[test]
    fn king_move() {
        let m = Move {piece: Piece::King, square: 41};
        let s = print_move(m);
        assert_eq!(s, "Kf1");
    }

    #[test]
    fn pawn_move() {
        let m = Move {piece: Piece::Pawn, square: 36};
        let s = print_move(m);
        assert_eq!(s, "e4");
    }
}