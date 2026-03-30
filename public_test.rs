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
        let m = Move::build(WHITE_ROOK, A2 as u8, A1 as u8);
        let s = print_move(m);
        assert_eq!(s, "Ra1");
    }
    
    #[test]
    fn knight_move() {
        let m = Move::build( WHITE_KNIGHT, G1 as u8,  F3 as u8);
        let s = print_move(m);
        assert_eq!(s, "Nf3");
    }

    #[test]
    fn bishop_move() {
        let m = Move::build(BLACK_BISHOP, F1 as u8, C4 as u8);
        let s = print_move(m);
        assert_eq!(s, "Bc4");
    }

    #[test]
    fn queen_move() {
        let m = Move::build(WHITE_QUEEN, D1 as u8, H5 as u8);
        let s = print_move(m);
        assert_eq!(s, "Qh5");
    }

    #[test]
    fn king_move() {
        let m = Move::build(WHITE_KING, E1 as u8, F1 as u8);
        let s = print_move(m);
        assert_eq!(s, "Kf1");
    }

    #[test]
    fn pawn_move() {
        let m = Move::build(WHITE_PAWN, E2 as u8, E4 as u8);
        let s = print_move(m);
        assert_eq!(s, "e4");
    }

    #[test]
    fn pawn_capture() {
        let m = Move::build(WHITE_PAWN, E4 as u8, D5 as u8).capture();
        let s = print_move(m);
        assert_eq!(s, "ed5");
    }

    #[test]
    fn bishop_capture() {
        let m = Move::build(BLACK_BISHOP, F1 as u8, C4 as u8).capture();
        let s = print_move(m);
        assert_eq!(s, "Bc4");
    }


    #[test]
    fn promotion() {
        let m = Move::build(WHITE_PAWN, F7 as u8, F8 as u8).promote_to(WHITE_QUEEN);
        let s = print_move(m);
        assert_eq!(s, "f8=Q");
    }

    #[test]
    fn promotion_with_capture() {
        let m = Move::build(WHITE_PAWN, F7 as u8, E8 as u8).capture().promote_to(WHITE_QUEEN);
        let s = print_move(m);
        assert_eq!(s, "fe8=Q");
    }

    #[test]
    fn short_castling() {
        let m = Move::build_short_castling(PieceColor::White);
        let s = print_move(m);
        assert_eq!(s, "0-0");
    }    

    #[test]
    fn long_castling() {
        let m = Move::build_long_castling(PieceColor::Black);
        let s = print_move(m);
        assert_eq!(s, "0-0-0");
    }    
}

pub mod position_tests {
    use crate::piece::*;
    use crate::position;
    use crate::position::*;
    use crate::r#move::*;
    use crate::square::Squares::*;
    use crate::square::get_col_and_row;

    #[test]
    fn make_move_start_move() {
        let mut position = start_position();
        let e4 = Move::build(WHITE_PAWN, E2 as u8, E4 as u8);
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
    fn make_move_capture() {
        let mut position = start_position();
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        let (f5_col, f5_row) = get_col_and_row(F5 as u8);
        position.pieces[e4_row as usize][e4_col as usize] = Some(WHITE_PAWN);
        position.pieces[f5_row as usize][f5_col as usize] = Some(BLACK_PAWN);
        let ef5 = Move::build(WHITE_PAWN, E4 as u8, F5 as u8).capture();
        make_move(&mut position, ef5);
        let mut expected = start_position();
        expected.pieces[f5_row as usize][f5_col as usize] = Some(WHITE_PAWN);
        expected.turn = PieceColor::Black;
        assert_eq!(position, expected);
    }

    #[test]
    #[should_panic(expected = "Square is already occupied!")]
    fn make_move_capture_own_piece() {
        let mut position = start_position();
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        let (f5_col, f5_row) = get_col_and_row(F5 as u8);
        position.pieces[e4_row as usize][e4_col as usize] = Some(WHITE_PAWN);
        position.pieces[f5_row as usize][f5_col as usize] = Some(WHITE_PAWN);
        let ef5 = Move::build(WHITE_PAWN, E4 as u8, F5 as u8).capture();
        make_move(&mut position, ef5);
    }

    #[test]
    #[should_panic(expected = "No piece on source square!")]
    fn make_move_no_piece() {
        let mut position = start_position();
        let (f5_col, f5_row) = get_col_and_row(F5 as u8);
        position.pieces[f5_row as usize][f5_col as usize] = Some(BLACK_PAWN);
        let ef5 = Move::build(WHITE_PAWN, E4 as u8, F5 as u8).capture();
        make_move(&mut position, ef5);
    }

    fn start_position_expected_legal_moves() -> Vec<Move> {
        vec![
        Move::build(WHITE_PAWN, A2 as u8, A3 as u8),
        Move::build(WHITE_PAWN, A2 as u8, A4 as u8),
        Move::build(WHITE_PAWN, B2 as u8, B3 as u8),
        Move::build(WHITE_PAWN, B2 as u8, B4 as u8),
        Move::build(WHITE_PAWN, C2 as u8, C3 as u8),
        Move::build(WHITE_PAWN, C2 as u8, C4 as u8),
        Move::build(WHITE_PAWN, D2 as u8, D3 as u8),
        Move::build(WHITE_PAWN, D2 as u8, D4 as u8),
        Move::build(WHITE_PAWN, E2 as u8, E3 as u8),
        Move::build(WHITE_PAWN, E2 as u8, E4 as u8),
        Move::build(WHITE_PAWN, F2 as u8, F3 as u8),
        Move::build(WHITE_PAWN, F2 as u8, F4 as u8),
        Move::build(WHITE_PAWN, G2 as u8, G3 as u8),
        Move::build(WHITE_PAWN, G2 as u8, G4 as u8),
        Move::build(WHITE_PAWN, H2 as u8, H3 as u8),
        Move::build(WHITE_PAWN, H2 as u8, H4 as u8),
        Move::build(WHITE_KNIGHT, B1 as u8, A3 as u8),
        Move::build(WHITE_KNIGHT, B1 as u8, C3 as u8),
        Move::build(WHITE_KNIGHT, G1 as u8, F3 as u8),
        Move::build(WHITE_KNIGHT, G1 as u8, H3 as u8),
        ]
    }

    #[test]
    fn legal_moves_start_position() {
        let position = start_position();
        let mut expected = start_position_expected_legal_moves();
        let mut moves = legal_moves(position);
        expected.sort();
        moves.sort();
        assert_eq!(moves, expected);
    }

    #[test]
    fn legal_moves_all_piece_types() {
        let mut position = start_position();
        let a4 = Move::build(WHITE_PAWN, A2 as u8, A4 as u8);
        let d4 = Move::build(WHITE_PAWN, D2 as u8, D4 as u8);
        let b5 = Move::build(BLACK_PAWN, B7 as u8, B5 as u8);
        let d5 = Move::build(BLACK_PAWN, D7 as u8, D5 as u8);
        make_move(&mut position, a4);
        make_move(&mut position, b5);
        make_move(&mut position, d4);
        make_move(&mut position, d5);
        let mut expected = vec![
            Move::build(WHITE_PAWN, A4 as u8, A5 as u8),
            Move::build(WHITE_PAWN, A4 as u8, B5 as u8).capture(),
            Move::build(WHITE_PAWN, B2 as u8, B3 as u8),
            Move::build(WHITE_PAWN, B2 as u8, B4 as u8),
            Move::build(WHITE_PAWN, C2 as u8, C3 as u8),
            Move::build(WHITE_PAWN, C2 as u8, C4 as u8),
            Move::build(WHITE_PAWN, E2 as u8, E3 as u8),
            Move::build(WHITE_PAWN, E2 as u8, E4 as u8),
            Move::build(WHITE_PAWN, F2 as u8, F3 as u8),
            Move::build(WHITE_PAWN, F2 as u8, F4 as u8),
            Move::build(WHITE_PAWN, G2 as u8, G3 as u8),
            Move::build(WHITE_PAWN, G2 as u8, G4 as u8),
            Move::build(WHITE_PAWN, H2 as u8, H3 as u8),
            Move::build(WHITE_PAWN, H2 as u8, H4 as u8),
            Move::build(WHITE_KNIGHT, B1 as u8, A3 as u8),
            Move::build(WHITE_KNIGHT, B1 as u8, C3 as u8),
            Move::build(WHITE_KNIGHT, B1 as u8, D2 as u8),
            Move::build(WHITE_KNIGHT, G1 as u8, F3 as u8),
            Move::build(WHITE_KNIGHT, G1 as u8, H3 as u8),
            Move::build(WHITE_ROOK, A1 as u8, A2 as u8),
            Move::build(WHITE_ROOK, A1 as u8, A3 as u8),
            Move::build(WHITE_BISHOP, C1 as u8, D2 as u8),
            Move::build(WHITE_BISHOP, C1 as u8, E3 as u8),
            Move::build(WHITE_BISHOP, C1 as u8, F4 as u8),
            Move::build(WHITE_BISHOP, C1 as u8, G5 as u8),
            Move::build(WHITE_BISHOP, C1 as u8, H6 as u8),
            Move::build(WHITE_QUEEN, D1 as u8, D2 as u8),
            Move::build(WHITE_QUEEN, D1 as u8, D3 as u8),
            Move::build(WHITE_KING, E1 as u8, D2 as u8),
        ];
        let mut moves = legal_moves(position);
        expected.sort();
        moves.sort();
        assert_eq!(moves, expected);
    }
}