#[cfg(test)]
pub mod square_tests {
    use crate::{square::get_square_name};

    #[test]
    #[should_panic(expected = "Invalid square!")]
    fn too_small_square_test() {
        get_square_name(0);
    }

    #[test]
    #[should_panic(expected = "Invalid square!")]
    fn too_big_square_test() {
        get_square_name(65);
    }


    #[test]
    fn a1_square_test() {
        let s = get_square_name(1);
        assert_eq!(s, "a1");
    }

    #[test]
    fn e4_square_test() {
        let s = get_square_name(36);
        assert_eq!(s, "e4");
    }

    #[test]
    fn h8_square_test() {
        let s = get_square_name(64);
        assert_eq!(s, "h8");
    }
}

pub mod piece_test {
    use crate::piece::*;

    #[test]
    fn black_pawn() {
        let c = get_char(BLACK_PAWN);
        assert_eq!(c, None);
    }

    #[test]
    fn black_knight() {
        let c = get_char(BLACK_KNIGHT);
        assert_eq!(c, Some('N'));
    }

    #[test]
    fn black_bishop() {
        let c = get_char(BLACK_BISHOP);
        assert_eq!(c, Some('B'));
    }

    #[test]
    fn black_rook() {
        let c = get_char(BLACK_ROOK);
        assert_eq!(c, Some('R'));
    }

    #[test]
    fn black_queen() {
        let c = get_char(BLACK_QUEEN);
        assert_eq!(c, Some('Q'));
    }

    #[test]
    fn black_king() {
        let c = get_char(BLACK_KING);
        assert_eq!(c, Some('K'));
    }

    #[test]
    fn white_pawn() {
        let c = get_char(WHITE_PAWN);
        assert_eq!(c, None);
    }

    #[test]
    fn white_knight() {
        let c = get_char(WHITE_KNIGHT);
        assert_eq!(c, Some('N'));
    }

    #[test]
    fn white_bishop() {
        let c = get_char(WHITE_BISHOP);
        assert_eq!(c, Some('B'));
    }

    #[test]
    fn white_rook() {
        let c = get_char(WHITE_ROOK);
        assert_eq!(c, Some('R'));
    }

    #[test]
    fn white_queen() {
        let c = get_char(WHITE_QUEEN);
        assert_eq!(c, Some('Q'));
    }

    #[test]
    fn white_king() {
        let c = get_char(WHITE_KING);
        assert_eq!(c, Some('K'));
    }
}

pub mod move_tests {
    use crate::r#move::Move;
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