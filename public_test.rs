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
    use crate::r#move::*;
    use crate::piece::*;
    use crate::square::Squares::*;

    #[test]
    fn rook_move_simple_square() {
        let m = Move {piece: WHITE_ROOK, from: A2 as u8, to: A1 as u8};
        let s = print_move(m);
        assert_eq!(s, "Ra1");
    }
    
    #[test]
    fn knight_move() {
        let m = Move {piece: WHITE_KNIGHT, from: G1 as u8, to: F3 as u8};
        let s = print_move(m);
        assert_eq!(s, "Nf3");
    }

    #[test]
    fn bishop_move() {
        let m = Move {piece: BLACK_BISHOP, from: F1 as u8, to: C4 as u8};
        let s = print_move(m);
        assert_eq!(s, "Bc4");
    }

    #[test]
    fn queen_move() {
        let m = Move {piece: WHITE_QUEEN, from: D1 as u8, to: H5 as u8};
        let s = print_move(m);
        assert_eq!(s, "Qh5");
    }

    #[test]
    fn king_move() {
        let m = Move {piece: WHITE_KING, from: E1 as u8, to: F1 as u8};
        let s = print_move(m);
        assert_eq!(s, "Kf1");
    }

    #[test]
    fn pawn_move() {
        let m = Move {piece: BLACK_PAWN, from: E2 as u8, to: E4 as u8};
        let s = print_move(m);
        assert_eq!(s, "e4");
    }
}

pub mod position_tests {
    use crate::piece::*;
    use crate::position::*;
    use crate::r#move::*;
    use crate::square::Squares::*;
    use crate::square::get_col_and_row;

    #[test]
    fn start_move() {
        let mut position = start_position();
        let e4 = Move { piece: WHITE_PAWN, from: E2 as u8, to: E4 as u8 };
        make_move(&mut position, e4);
    

        let mut expected = start_position();
        let (from_col, from_row) = get_col_and_row(e4.from);
        expected.pieces[from_row as usize][from_col as usize] = None;
        let (to_col, to_row) = get_col_and_row(e4.to);
        expected.pieces[to_row as usize][to_col as usize] = Some(WHITE_PAWN);
        expected.turn = PieceColor::Black;
        assert_eq!(position, expected);
    }

    #[test]
    fn capture() {
        let mut position = start_position();
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        let (f5_col, f5_row) = get_col_and_row(F5 as u8);
        position.pieces[e4_row as usize][e4_col as usize] = Some(WHITE_PAWN);
        position.pieces[f5_row as usize][f5_col as usize] = Some(BLACK_PAWN);
        let ef5 = Move { piece: WHITE_PAWN, from: E4 as u8, to: F5 as u8 };
        make_move(&mut position, ef5);
        let mut expected = start_position();
        expected.pieces[f5_row as usize][f5_col as usize] = Some(WHITE_PAWN);
        expected.turn = PieceColor::Black;
        assert_eq!(position, expected);
    }

    #[test]
    #[should_panic(expected = "Square is already occupied!")]
    fn capture_own_piece() {
        let mut position = start_position();
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        let (f5_col, f5_row) = get_col_and_row(F5 as u8);
        position.pieces[e4_row as usize][e4_col as usize] = Some(WHITE_PAWN);
        position.pieces[f5_row as usize][f5_col as usize] = Some(WHITE_PAWN);
        let ef5 = Move { piece: WHITE_PAWN, from: E4 as u8, to: F5 as u8 };
        make_move(&mut position, ef5);
    }

    #[test]
    #[should_panic(expected = "No piece on source square!")]
    fn no_piece() {
        let mut position = start_position();
        let (f5_col, f5_row) = get_col_and_row(F5 as u8);
        position.pieces[f5_row as usize][f5_col as usize] = Some(BLACK_PAWN);
        let ef5 = Move { piece: WHITE_PAWN, from: E4 as u8, to: F5 as u8 };
        make_move(&mut position, ef5);
    }
}