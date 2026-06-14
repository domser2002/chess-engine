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
    use crate::square::get_col_and_row;
    use crate::position::*;

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

    #[test]
    fn parse_uci_basic_move() {
        let pos = start_position();
        let m = parse_move_uci("e2e4", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(!m.capture);
        assert_eq!(m.from, E2 as u8);
        assert_eq!(m.to, E4 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_uci_knight_move() {
        let pos = start_position();
        let m = parse_move_uci("g1f3", &pos).unwrap();
        assert_eq!(m.piece, WHITE_KNIGHT);
        assert!(!m.capture);
        assert_eq!(m.from, G1 as u8);
        assert_eq!(m.to, F3 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_uci_capture() {
        let mut pos = start_position();
        // Move pawns to set up capture
        make_move(&mut pos, Move::build(WHITE_PAWN, D2 as u8, D4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, E7 as u8, E5 as u8));
        let m = parse_move_uci("d4e5", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(m.capture);
        assert_eq!(m.from, D4 as u8);
        assert_eq!(m.to, E5 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_uci_promotion() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (f7_col, f7_row) = get_col_and_row(F7 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[f7_row as usize][f7_col as usize] = Some(WHITE_PAWN);

        let m = parse_move_uci("f7f8q", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(!m.capture);
        assert_eq!(m.from, F7 as u8);
        assert_eq!(m.to, F8 as u8);
        assert_eq!(m.new_piece, Some(WHITE_QUEEN));
    }

    #[test]
    fn parse_uci_en_passant() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (e5_col, e5_row) = get_col_and_row(E5 as u8);
        let (d5_col, d5_row) = get_col_and_row(D5 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[e5_row as usize][e5_col as usize] = Some(WHITE_PAWN);
        pos.pieces[d5_row as usize][d5_col as usize] = Some(BLACK_PAWN);
        pos.last_move = Some(Move::build(BLACK_PAWN, D7 as u8, D5 as u8));

        let m = parse_move_uci("e5d6", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(m.capture);
        assert_eq!(m.from, E5 as u8);
        assert_eq!(m.to, D6 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_uci_invalid_square() {
        let pos = start_position();
        assert!(parse_move_uci("e2e9", &pos).is_none());
    }

    #[test]
    fn parse_uci_no_piece() {
        let pos = start_position();
        assert!(parse_move_uci("e4e5", &pos).is_none());
    }

    #[test]
    fn parse_uci_wrong_turn() {
        // White's turn, try to move a black piece
        let pos = start_position();
        assert!(parse_move_uci("e7e5", &pos).is_none());
    }

    #[test]
    fn parse_algebraic_basic() {
        let pos = start_position();
        let m = parse_move_algebraic("e4", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(!m.capture);
        assert_eq!(m.from, E2 as u8);
        assert_eq!(m.to, E4 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_algebraic_knight() {
        let pos = start_position();
        let m = parse_move_algebraic("Nf3", &pos).unwrap();
        assert_eq!(m.piece, WHITE_KNIGHT);
        assert!(!m.capture);
        assert_eq!(m.from, G1 as u8);
        assert_eq!(m.to, F3 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_algebraic_pawn_capture() {
        let mut pos = start_position();
        make_move(&mut pos, Move::build(WHITE_PAWN, E2 as u8, E4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, D7 as u8, D5 as u8));
        let m = parse_move_algebraic("ed5", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(m.capture);
        assert_eq!(m.from, E4 as u8);
        assert_eq!(m.to, D5 as u8);
        assert_eq!(m.new_piece, None);
    }

    #[test]
    fn parse_algebraic_promotion() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (f7_col, f7_row) = get_col_and_row(F7 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[f7_row as usize][f7_col as usize] = Some(WHITE_PAWN);

        let m = parse_move_algebraic("f8=Q", &pos).unwrap();
        assert_eq!(m.piece, WHITE_PAWN);
        assert!(!m.capture);
        assert_eq!(m.from, F7 as u8);
        assert_eq!(m.to, F8 as u8);
        assert_eq!(m.new_piece, Some(WHITE_QUEEN));
    }

    #[test]
    fn parse_algebraic_short_castling() {
        let pos = start_position();
        let m = parse_move_algebraic("0-0", &pos).unwrap();
        assert_eq!(m.from, SHORT_CASTLING_SQUARE);
        assert_eq!(m.to, SHORT_CASTLING_SQUARE);
    }

    #[test]
    fn parse_algebraic_long_castling() {
        let pos = start_position();
        let m = parse_move_algebraic("0-0-0", &pos).unwrap();
        assert_eq!(m.from, LONG_CASTLING_SQUARE);
        assert_eq!(m.to, LONG_CASTLING_SQUARE);
    }

    #[test]
    fn parse_disambiguation_two_rooks_file() {
        // Two white rooks, on f6 and c3, both can reach c6. King on e8 doesn't block.
        // "Rfc6" should pick the one from f6 (file disambiguation).
        let mut pos = empty_position(PieceColor::White);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (f6_col, f6_row) = get_col_and_row(F6 as u8);
        let (c3_col, c3_row) = get_col_and_row(C3 as u8);
        let (_c6_col, _c6_row) = get_col_and_row(C6 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[f6_row as usize][f6_col as usize] = Some(WHITE_ROOK);
        pos.pieces[c3_row as usize][c3_col as usize] = Some(WHITE_ROOK);

        let m = parse_move_algebraic("Rfc6", &pos).unwrap();
        assert_eq!(m.from, F6 as u8);
        assert_eq!(m.to, C6 as u8);
        assert!(!m.capture);
    }

    #[test]
    fn parse_disambiguation_two_rooks_rank() {
        // Two white rooks, on a3 and h3, both can reach c3.
        // "R3c3" should pick the one from rank 3 (only rank differs).
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (c1_col, c1_row) = get_col_and_row(C1 as u8);
        let (c4_col, c4_row) = get_col_and_row(C4 as u8);
        let (_c3_col, _c3_row) = get_col_and_row(C3 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[c1_row as usize][c1_col as usize] = Some(WHITE_ROOK);
        pos.pieces[c4_row as usize][c4_col as usize] = Some(WHITE_ROOK);

        let m = parse_move_algebraic("R1c3", &pos).unwrap();
        assert_eq!(m.from, C1 as u8);
        assert_eq!(m.to, C3 as u8);
        assert!(!m.capture);
    }

    #[test]
    fn parse_disambiguation_two_knights_file() {
        // Two white knights on g1 and b1, both can reach f3.
        // "Ngf3" picks the one from the g-file.
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (g1_col, g1_row) = get_col_and_row(G1 as u8);
        let (d2_col, d2_row) = get_col_and_row(D2 as u8);
        let (_f3_col, _f3_row) = get_col_and_row(F3 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[g1_row as usize][g1_col as usize] = Some(WHITE_KNIGHT);
        pos.pieces[d2_row as usize][d2_col as usize] = Some(WHITE_KNIGHT);

        let m = parse_move_algebraic("Ngf3", &pos).unwrap();
        assert_eq!(m.from, G1 as u8);
        assert_eq!(m.to, F3 as u8);
        assert!(!m.capture);
    }

    #[test]
    fn parse_disambiguation_pawn_capture_without_x() {
        // Pawns on a2 and c2 can both capture on b3.
        // User types "cb3" (shorthand for cxb3).
        // The parser should pick the pawn on c2, not a2.
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (a2_col, a2_row) = get_col_and_row(A2 as u8);
        let (c2_col, c2_row) = get_col_and_row(C2 as u8);
        let (b3_col, b3_row) = get_col_and_row(B3 as u8);
        
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[a2_row as usize][a2_col as usize] = Some(WHITE_PAWN);
        pos.pieces[c2_row as usize][c2_col as usize] = Some(WHITE_PAWN);
        // Put a black piece on b3 so it's a valid capture square
        pos.pieces[b3_row as usize][b3_col as usize] = Some(BLACK_KNIGHT);

        let m = parse_move_algebraic("cb3", &pos).unwrap();
        assert_eq!(m.from, C2 as u8);
        assert_eq!(m.to, B3 as u8);
        assert!(m.capture);
    }
}

pub mod position_tests {
    use crate::piece::*;
    use crate::position::*;
    use crate::r#move::*;
    use crate::square::Squares::*;
    use crate::square::get_col_and_row;
    use pretty_assertions::assert_eq;

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
        expected.last_move = Some(e4);
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
        expected.last_move = Some(ef5);
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

    #[test]
    fn make_move_promotion() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (f7_col, f7_row) = get_col_and_row(F7 as u8);
        let (f8_col, f8_row) = get_col_and_row(F8 as u8);

        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[f7_row as usize][f7_col as usize] = Some(WHITE_PAWN);

        let promotion = Move::build(WHITE_PAWN, F7 as u8, F8 as u8).promote_to(WHITE_QUEEN);
        make_move(&mut pos, promotion);

        assert!(pos.pieces[f7_row as usize][f7_col as usize].is_none());
        assert_eq!(pos.pieces[f8_row as usize][f8_col as usize], Some(WHITE_QUEEN));
        assert_eq!(pos.turn, PieceColor::Black);
    }

    #[test]
    fn make_move_en_passant() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (e5_col, e5_row) = get_col_and_row(E5 as u8);
        let (d5_col, d5_row) = get_col_and_row(D5 as u8);
        let (d6_col, d6_row) = get_col_and_row(D6 as u8);

        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[e5_row as usize][e5_col as usize] = Some(WHITE_PAWN);
        pos.pieces[d5_row as usize][d5_col as usize] = Some(BLACK_PAWN);
        pos.last_move = Some(Move::build(BLACK_PAWN, D7 as u8, D5 as u8));

        let exd6 = Move::build(WHITE_PAWN, E5 as u8, D6 as u8).capture();
        make_move(&mut pos, exd6);

        assert!(pos.pieces[e5_row as usize][e5_col as usize].is_none());
        assert_eq!(pos.pieces[d6_row as usize][d6_col as usize], Some(WHITE_PAWN));
        assert!(pos.pieces[d5_row as usize][d5_col as usize].is_none());
        assert_eq!(pos.turn, PieceColor::Black);
    }

    #[test]
    fn is_in_check_start_position() {
        let pos = start_position();
        assert!(!is_in_check(pos, PieceColor::White));
        assert!(!is_in_check(pos, PieceColor::Black));
    }

    #[test]
    fn is_in_check_by_bishop() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (h4_col, h4_row) = get_col_and_row(H4 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[h4_row as usize][h4_col as usize] = Some(BLACK_BISHOP);
        assert!(is_in_check(pos, PieceColor::White));
        assert!(!is_in_check(pos, PieceColor::Black));
    }

    #[test]
    fn is_in_check_by_rook() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        // Rook on same file (e4) attacks king on e1
        pos.pieces[e4_row as usize][e4_col as usize] = Some(BLACK_ROOK);
        assert!(is_in_check(pos, PieceColor::White));
    }

    #[test]
    fn is_in_check_by_knight() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (f3_col, f3_row) = get_col_and_row(F3 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[f3_row as usize][f3_col as usize] = Some(BLACK_KNIGHT);
        assert!(is_in_check(pos, PieceColor::White));
    }

    #[test]
    fn is_in_check_by_pawn() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (d2_col, d2_row) = get_col_and_row(D2 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        // Black pawn attacks diagonally upward (toward higher row index)
        pos.pieces[d2_row as usize][d2_col as usize] = Some(BLACK_PAWN);
        assert!(is_in_check(pos, PieceColor::White));
    }

    #[test]
    fn is_in_check_blocked_by_friendly_piece() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        let (e3_col, e3_row) = get_col_and_row(E3 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        // Rook on e4 would attack king on e1...
        pos.pieces[e4_row as usize][e4_col as usize] = Some(BLACK_ROOK);
        // ...but white pawn on e3 blocks the line
        pos.pieces[e3_row as usize][e3_col as usize] = Some(WHITE_PAWN);
        assert!(!is_in_check(pos, PieceColor::White));
    }

        #[test]
    fn start_position_has_castling_rights() {
        let pos = start_position();
        assert!(pos.white_short_castle);
        assert!(pos.white_long_castle);
        assert!(pos.black_short_castle);
        assert!(pos.black_long_castle);
    }

    #[test]
    fn castling_not_possible_with_pieces_in_way() {
        let pos = start_position();
        let moves = legal_moves(pos);
        // No castling possible in start position because knights and bishops block
        assert!(!moves.iter().any(|m| m.from == SHORT_CASTLING_SQUARE));
        assert!(!moves.iter().any(|m| m.from == LONG_CASTLING_SQUARE));
    }

    #[test]
    fn short_castling_works_after_clear() {
        let mut pos = start_position();
        // Clear path for short castle: move g1 knight, f1 bishop, e2 pawn
        make_move(&mut pos, Move::build(WHITE_PAWN, E2 as u8, E4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A7 as u8, A6 as u8));
        make_move(&mut pos, Move::build(WHITE_KNIGHT, G1 as u8, F3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, B7 as u8, B6 as u8));
        make_move(&mut pos, Move::build(WHITE_BISHOP, F1 as u8, E2 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, C7 as u8, C6 as u8));
        // Now white can castle short
        let moves = legal_moves(pos);
        assert!(moves.iter().any(|m| m.from == SHORT_CASTLING_SQUARE));
    }

    #[test]
    fn long_castling_works_after_clear() {
        let mut pos = start_position();
        // Clear path for long castle: move b1 knight, c1 bishop, d1 queen, d2 pawn, c2 pawn, b2 pawn
        make_move(&mut pos, Move::build(WHITE_PAWN, D2 as u8, D4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A7 as u8, A6 as u8));
        make_move(&mut pos, Move::build(WHITE_PAWN, C2 as u8, C3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, B7 as u8, B6 as u8));
        make_move(&mut pos, Move::build(WHITE_PAWN, B2 as u8, B3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, C7 as u8, C6 as u8));
        make_move(&mut pos, Move::build(WHITE_KNIGHT, B1 as u8, D2 as u8));
        // Black move
        make_move(&mut pos, Move::build(BLACK_PAWN, D7 as u8, D6 as u8));
        make_move(&mut pos, Move::build(WHITE_BISHOP, C1 as u8, B2 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, E7 as u8, E6 as u8));
        make_move(&mut pos, Move::build(WHITE_QUEEN, D1 as u8, C2 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, F7 as u8, F6 as u8));
        // Now white can castle long
        let moves = legal_moves(pos);
        assert!(moves.iter().any(|m| m.from == LONG_CASTLING_SQUARE));
    }

    #[test]
    fn castle_rights_lost_after_king_move() {
        let mut pos = empty_position(PieceColor::White);
        pos.white_short_castle = true;
        pos.white_long_castle = true;
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);

        make_move(&mut pos, Move::build(WHITE_KING, E1 as u8, E2 as u8));
        assert!(!pos.white_short_castle);
        assert!(!pos.white_long_castle);
    }

    #[test]
    fn castle_rights_lost_after_rook_move() {
        let mut pos = start_position();
        // First move the knight out of g1, then move the rook
        make_move(&mut pos, Move::build(WHITE_KNIGHT, G1 as u8, F3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A7 as u8, A6 as u8));
        make_move(&mut pos, Move::build(WHITE_ROOK, H1 as u8, G1 as u8));
        assert!(!pos.white_short_castle);
        assert!(pos.white_long_castle);
    }

    #[test]
    fn castle_rights_lost_after_rook_captured() {
        let mut pos = empty_position(PieceColor::White);
        let (h1_col, h1_row) = get_col_and_row(H1 as u8);
        let (h8_col, h8_row) = get_col_and_row(H8 as u8);
        pos.pieces[h1_row as usize][h1_col as usize] = Some(WHITE_ROOK);
        pos.pieces[h8_row as usize][h8_col as usize] = Some(BLACK_ROOK);
        pos.white_short_castle = true;

        // Capture the black rook on h8
        make_move(&mut pos, Move::build(WHITE_ROOK, H1 as u8, H8 as u8).capture());
        assert!(!pos.white_short_castle);
        assert!(!pos.black_short_castle);
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

    #[test]
    fn make_unmake_normal_move() {
        let mut pos = start_position();
        let original = pos;
        let e4 = Move::build(WHITE_PAWN, E2 as u8, E4 as u8);
        let undo = make_move(&mut pos, e4);
        assert_ne!(pos, original);
        unmake_move(&mut pos, e4, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn make_unmake_capture() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        let (d5_col, d5_row) = get_col_and_row(D5 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[e4_row as usize][e4_col as usize] = Some(WHITE_PAWN);
        pos.pieces[d5_row as usize][d5_col as usize] = Some(BLACK_PAWN);
        let original = pos;
        let capture = Move::build(WHITE_PAWN, E4 as u8, D5 as u8).capture();
        let undo = make_move(&mut pos, capture);
        assert_ne!(pos, original);
        unmake_move(&mut pos, capture, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn make_unmake_en_passant() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (e5_col, e5_row) = get_col_and_row(E5 as u8);
        let (d5_col, d5_row) = get_col_and_row(D5 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[e5_row as usize][e5_col as usize] = Some(WHITE_PAWN);
        pos.pieces[d5_row as usize][d5_col as usize] = Some(BLACK_PAWN);
        pos.last_move = Some(Move::build(BLACK_PAWN, D7 as u8, D5 as u8));
        let original = pos;
        let exd6 = Move::build(WHITE_PAWN, E5 as u8, D6 as u8).capture();
        let undo = make_move(&mut pos, exd6);
        assert_ne!(pos, original);
        unmake_move(&mut pos, exd6, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn make_unmake_short_castle() {
        let mut pos = start_position();
        // Clear path: e4, Nf3, Bc4
        make_move(&mut pos, Move::build(WHITE_PAWN, E2 as u8, E4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A7 as u8, A6 as u8));
        make_move(&mut pos, Move::build(WHITE_KNIGHT, G1 as u8, F3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, B7 as u8, B6 as u8));
        make_move(&mut pos, Move::build(WHITE_BISHOP, F1 as u8, C4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, C7 as u8, C6 as u8));
        // Now White can castle short. Save state before castling.
        let original = pos;
        let castle = Move::build_short_castling(PieceColor::White);
        let undo = make_move(&mut pos, castle);
        assert_ne!(pos, original);
        unmake_move(&mut pos, castle, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn make_unmake_long_castle() {
        let mut pos = start_position();
        // Clear path: d4, Qd3, Bd2, Nc3
        make_move(&mut pos, Move::build(WHITE_PAWN, D2 as u8, D4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A7 as u8, A6 as u8));
        make_move(&mut pos, Move::build(WHITE_QUEEN, D1 as u8, D3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, B7 as u8, B6 as u8));
        make_move(&mut pos, Move::build(WHITE_BISHOP, C1 as u8, D2 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, C7 as u8, C6 as u8));
        make_move(&mut pos, Move::build(WHITE_KNIGHT, B1 as u8, C3 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, D7 as u8, D6 as u8));
        // Now White can castle long
        let original = pos;
        let castle = Move::build_long_castling(PieceColor::White);
        let undo = make_move(&mut pos, castle);
        assert_ne!(pos, original);
        unmake_move(&mut pos, castle, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn make_unmake_promotion() {
        let mut pos = empty_position(PieceColor::White);
        let (a1_col, a1_row) = get_col_and_row(A1 as u8);
        let (a8_col, a8_row) = get_col_and_row(A8 as u8);
        let (f7_col, f7_row) = get_col_and_row(F7 as u8);
        let (_f8_col, _f8_row) = get_col_and_row(F8 as u8);
        pos.pieces[a1_row as usize][a1_col as usize] = Some(WHITE_KING);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_KING);
        pos.pieces[f7_row as usize][f7_col as usize] = Some(WHITE_PAWN);
        let original = pos;
        let promo = Move::build(WHITE_PAWN, F7 as u8, F8 as u8).promote_to(WHITE_QUEEN);
        let undo = make_move(&mut pos, promo);
        assert_ne!(pos, original);
        unmake_move(&mut pos, promo, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn make_unmake_castle_rights_restored() {
        let mut pos = empty_position(PieceColor::White);
        pos.white_short_castle = true;
        pos.white_long_castle = true;
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        let original = pos;
        // Move king: loses both castle rights
        let kg1 = Move::build(WHITE_KING, E1 as u8, E2 as u8);
        let undo = make_move(&mut pos, kg1);
        assert!(!pos.white_short_castle);
        assert!(!pos.white_long_castle);
        unmake_move(&mut pos, kg1, undo);
        assert_eq!(pos, original);
    }

    #[test]
    fn legal_moves_with_en_passant() {
        let mut position = start_position();
        let e4 = Move::build(WHITE_PAWN, E2 as u8, E4 as u8);
        let e5 = Move::build(WHITE_PAWN, E4 as u8, E5 as u8);
        let b6 = Move::build(BLACK_PAWN, B7 as u8, B6 as u8);
        let d5 = Move::build(BLACK_PAWN, D7 as u8, D5 as u8);
        make_move(&mut position, e4);
        make_move(&mut position, b6);
        make_move(&mut position, e5);
        make_move(&mut position, d5);
        let mut expected = vec![
            Move::build(WHITE_PAWN, A2 as u8, A3 as u8),
            Move::build(WHITE_PAWN, A2 as u8, A4 as u8),
            Move::build(WHITE_PAWN, B2 as u8, B3 as u8),
            Move::build(WHITE_PAWN, B2 as u8, B4 as u8),
            Move::build(WHITE_PAWN, C2 as u8, C3 as u8),
            Move::build(WHITE_PAWN, C2 as u8, C4 as u8),
            Move::build(WHITE_PAWN, D2 as u8, D3 as u8),
            Move::build(WHITE_PAWN, D2 as u8, D4 as u8),
            Move::build(WHITE_PAWN, E5 as u8, E6 as u8),
            Move::build(WHITE_PAWN, E5 as u8, D6 as u8).capture(),
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
            Move::build(WHITE_KNIGHT, G1 as u8, E2 as u8),
            Move::build(WHITE_BISHOP, F1 as u8, E2 as u8),
            Move::build(WHITE_BISHOP, F1 as u8, D3 as u8),
            Move::build(WHITE_BISHOP, F1 as u8, C4 as u8),
            Move::build(WHITE_BISHOP, F1 as u8, B5 as u8),
            Move::build(WHITE_BISHOP, F1 as u8, A6 as u8),
            Move::build(WHITE_QUEEN, D1 as u8, E2 as u8),
            Move::build(WHITE_QUEEN, D1 as u8, F3 as u8),
            Move::build(WHITE_QUEEN, D1 as u8, G4 as u8),
            Move::build(WHITE_QUEEN, D1 as u8, H5 as u8),
            Move::build(WHITE_KING, E1 as u8, E2 as u8),
        ];
        let mut moves = legal_moves(position);
        expected.sort();
        moves.sort();
        assert_eq!(moves, expected);
    }
}

pub mod eval_tests {
    use crate::eval::evaluate;
    use crate::piece::*;
    use crate::position::*;
    use crate::square::get_col_and_row;

    #[test]
    fn start_position_even() {
        assert_eq!(evaluate(start_position()), 0);
    }

    #[test]
    fn white_up_one_pawn() {
        let mut pos = start_position();
        let (e7_col, e7_row) = get_col_and_row(crate::square::Squares::E7 as u8);
        pos.pieces[e7_row as usize][e7_col as usize] = None;
        assert_eq!(evaluate(pos), 100);
    }

    #[test]
    fn black_up_a_knight() {
        let mut pos = start_position();
        let (g1_col, g1_row) = get_col_and_row(crate::square::Squares::G1 as u8);
        pos.pieces[g1_row as usize][g1_col as usize] = None;
        pos.turn = PieceColor::Black;
        assert_eq!(evaluate(pos), 320);
    }

    #[test]
    fn white_down_a_queen() {
        let mut pos = start_position();
        let (d1_col, d1_row) = get_col_and_row(crate::square::Squares::D1 as u8);
        pos.pieces[d1_row as usize][d1_col as usize] = None;
        assert_eq!(evaluate(pos), -900);
    }
}

pub mod fen_tests {
    use crate::piece::*;
    use crate::position::*;
    use crate::square::Squares::*;
    use crate::square::get_col_and_row;
    use pretty_assertions::assert_eq;

    #[test]
    fn start_position_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = parse_fen(fen).unwrap();
        let expected = start_position();
        assert_eq!(pos.pieces, expected.pieces);
        assert_eq!(pos.turn, PieceColor::White);
        assert!(pos.white_short_castle);
        assert!(pos.white_long_castle);
        assert!(pos.black_short_castle);
        assert!(pos.black_long_castle);
        assert!(pos.last_move.is_none());
    }

    #[test]
    fn black_to_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let pos = parse_fen(fen).unwrap();
        assert_eq!(pos.turn, PieceColor::Black);
    }

    #[test]
    fn pieces_after_e4() {
        // After 1. e4: pawn on e4, e2 empty
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let pos = parse_fen(fen).unwrap();
        let (e2_col, e2_row) = get_col_and_row(E2 as u8);
        let (e4_col, e4_row) = get_col_and_row(E4 as u8);
        assert!(pos.pieces[e2_row as usize][e2_col as usize].is_none());
        assert_eq!(pos.pieces[e4_row as usize][e4_col as usize], Some(WHITE_PAWN));
    }

    #[test]
    fn no_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
        let pos = parse_fen(fen).unwrap();
        assert!(!pos.white_short_castle);
        assert!(!pos.white_long_castle);
        assert!(!pos.black_short_castle);
        assert!(!pos.black_long_castle);
    }

    #[test]
    fn partial_castling_white_kingside_only() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K - 0 1";
        let pos = parse_fen(fen).unwrap();
        assert!(pos.white_short_castle);
        assert!(!pos.white_long_castle);
        assert!(!pos.black_short_castle);
        assert!(!pos.black_long_castle);
    }

    #[test]
    fn partial_castling_black_queenside_only() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w q - 0 1";
        let pos = parse_fen(fen).unwrap();
        assert!(!pos.white_short_castle);
        assert!(!pos.white_long_castle);
        assert!(!pos.black_short_castle);
        assert!(pos.black_long_castle);
    }

    #[test]
    fn en_passant_white_push() {
        // After 1. e4, en passant target is e3
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let pos = parse_fen(fen).unwrap();
        // last_move should synthesize a white pawn double push e2→e4
        let lm = pos.last_move.unwrap();
        assert_eq!(lm.piece, WHITE_PAWN);
        assert_eq!(lm.from, E2 as u8);
        assert_eq!(lm.to, E4 as u8);
    }

    #[test]
    fn en_passant_black_push() {
        // After 1. e4 d5 2. e5 f5, en passant target is f6
        let fen = "rnbqkbnr/ppppp1pp/8/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3";
        let pos = parse_fen(fen).unwrap();
        // last_move should synthesize a black pawn double push f7→f5
        let lm = pos.last_move.unwrap();
        assert_eq!(lm.piece, BLACK_PAWN);
        assert_eq!(lm.from, F7 as u8);
        assert_eq!(lm.to, F5 as u8);
    }

    #[test]
    fn en_passant_capture_available() {
        // White pawn on e5, black just played f7-f5. En passant exf6 should be legal.
        let fen = "rnbqkbnr/ppppp1pp/8/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3";
        let pos = parse_fen(fen).unwrap();
        let moves = legal_moves(pos);
        // There should be an en passant capture: e5 pawn captures on f6
        let ep = moves.iter().find(|m| {
            m.piece == WHITE_PAWN && m.from == E5 as u8 && m.to == F6 as u8 && m.capture
        });
        assert!(ep.is_some(), "En passant capture e5xf6 should be legal");
    }

    #[test]
    fn middlegame_position() {
        // Italian Game position after 1.e4 e5 2.Nf3 Nc6 3.Bc4
        let fen = "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3";
        let pos = parse_fen(fen).unwrap();
        assert_eq!(pos.turn, PieceColor::Black);
        // White bishop on c4
        let (c4_col, c4_row) = get_col_and_row(C4 as u8);
        assert_eq!(pos.pieces[c4_row as usize][c4_col as usize], Some(WHITE_BISHOP));
        // White knight on f3
        let (f3_col, f3_row) = get_col_and_row(F3 as u8);
        assert_eq!(pos.pieces[f3_row as usize][f3_col as usize], Some(WHITE_KNIGHT));
        // Black knight on c6
        let (c6_col, c6_row) = get_col_and_row(C6 as u8);
        assert_eq!(pos.pieces[c6_row as usize][c6_col as usize], Some(BLACK_KNIGHT));
        // Black pawn on e5
        let (e5_col, e5_row) = get_col_and_row(E5 as u8);
        assert_eq!(pos.pieces[e5_row as usize][e5_col as usize], Some(BLACK_PAWN));
        // e2 is empty (pawn moved to e4)
        let (e2_col, e2_row) = get_col_and_row(E2 as u8);
        assert!(pos.pieces[e2_row as usize][e2_col as usize].is_none());
    }

    #[test]
    fn empty_board() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        let pos = parse_fen(fen).unwrap();
        for row in 0..8 {
            for col in 0..8 {
                assert!(pos.pieces[row][col].is_none());
            }
        }
    }

    #[test]
    fn single_king_each() {
        let fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
        let pos = parse_fen(fen).unwrap();
        let (e1_col, e1_row) = get_col_and_row(E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(E8 as u8);
        assert_eq!(pos.pieces[e1_row as usize][e1_col as usize], Some(WHITE_KING));
        assert_eq!(pos.pieces[e8_row as usize][e8_col as usize], Some(BLACK_KING));
    }

    #[test]
    fn fen_without_move_counters() {
        // FEN with only 4 fields (no halfmove/fullmove) should still parse
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let pos = parse_fen(fen).unwrap();
        assert_eq!(pos.turn, PieceColor::White);
    }

    #[test]
    fn invalid_fen_too_few_fields() {
        assert!(parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w").is_none());
    }

    #[test]
    fn invalid_fen_bad_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert!(parse_fen(fen).is_none());
    }

    #[test]
    fn invalid_fen_too_few_ranks() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_none());
    }

    #[test]
    fn invalid_fen_bad_piece_char() {
        let fen = "rnbqkbnr/ppppXppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_none());
    }

    #[test]
    fn invalid_fen_rank_too_wide() {
        // 9 squares in rank 8
        let fen = "rnbqkbnrr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_none());
    }

    #[test]
    fn invalid_fen_rank_too_narrow() {
        // Only 7 squares in rank 8
        let fen = "rnbqkbn/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_none());
    }

    #[test]
    fn fen_roundtrip_matches_start_position() {
        // Parsing the standard start FEN should produce the same board as start_position()
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let from_fen = parse_fen(fen).unwrap();
        let from_code = start_position();
        assert_eq!(from_fen.pieces, from_code.pieces);
        assert_eq!(from_fen.turn, from_code.turn);
        assert_eq!(from_fen.white_short_castle, from_code.white_short_castle);
        assert_eq!(from_fen.white_long_castle, from_code.white_long_castle);
        assert_eq!(from_fen.black_short_castle, from_code.black_short_castle);
        assert_eq!(from_fen.black_long_castle, from_code.black_long_castle);
    }
}

pub mod uci_tests {
    use crate::piece::*;
    use crate::r#move::*;
    use crate::uci::move_to_uci;
    use crate::square::Squares::*;

    #[test]
    fn uci_pawn_move() {
        let m = Move::build(WHITE_PAWN, E2 as u8, E4 as u8);
        assert_eq!(move_to_uci(m), "e2e4");
    }

    #[test]
    fn uci_knight_move() {
        let m = Move::build(WHITE_KNIGHT, G1 as u8, F3 as u8);
        assert_eq!(move_to_uci(m), "g1f3");
    }

    #[test]
    fn uci_capture() {
        let m = Move::build(WHITE_PAWN, E4 as u8, D5 as u8).capture();
        assert_eq!(move_to_uci(m), "e4d5");
    }

    #[test]
    fn uci_promotion_queen() {
        let m = Move::build(WHITE_PAWN, E7 as u8, E8 as u8).promote_to(WHITE_QUEEN);
        assert_eq!(move_to_uci(m), "e7e8q");
    }

    #[test]
    fn uci_promotion_knight() {
        let m = Move::build(BLACK_PAWN, A2 as u8, A1 as u8).promote_to(BLACK_KNIGHT);
        assert_eq!(move_to_uci(m), "a2a1n");
    }

    #[test]
    fn uci_white_short_castle() {
        let m = Move::build_short_castling(PieceColor::White);
        assert_eq!(move_to_uci(m), "e1g1");
    }

    #[test]
    fn uci_black_short_castle() {
        let m = Move::build_short_castling(PieceColor::Black);
        assert_eq!(move_to_uci(m), "e8g8");
    }

    #[test]
    fn uci_white_long_castle() {
        let m = Move::build_long_castling(PieceColor::White);
        assert_eq!(move_to_uci(m), "e1c1");
    }

    #[test]
    fn uci_black_long_castle() {
        let m = Move::build_long_castling(PieceColor::Black);
        assert_eq!(move_to_uci(m), "e8c8");
    }
}

pub mod search_tests {
    use crate::piece::*;
    use crate::position::*;
    use crate::r#move::*;
    use crate::search::*;
    use crate::square::get_col_and_row;
    use crate::square::Squares::*;

    #[test]
    fn best_move_from_start() {
        let pos = start_position();
        let m = best_move(pos);
        assert!(m.is_some());
    }

    #[test]
    fn node_counter_is_nonzero() {
        let pos = start_position();
        let config = SearchConfig { max_depth: 2, time_per_move_ms: 60_000, uci_output: false };
        let result = search_with_time(pos, config);
        assert!(result.nodes > 0, "Search must count at least one node");
    }

    #[test]
    fn uci_output_does_not_affect_result() {
        let pos = start_position();
        let config_silent = SearchConfig { max_depth: 3, time_per_move_ms: 60_000, uci_output: false };
        let config_uci = SearchConfig { max_depth: 3, time_per_move_ms: 60_000, uci_output: true };
        let result_silent = search_with_time(pos, config_silent);
        let result_uci = search_with_time(pos, config_uci);
        assert_eq!(result_silent.best_move, result_uci.best_move);
        assert_eq!(result_silent.score, result_uci.score);
        assert_eq!(result_silent.nodes, result_uci.nodes);
        assert_eq!(result_silent.depth_reached, result_uci.depth_reached);
    }

    #[test]
    fn node_counter_zero_when_no_moves() {
        // Black king on a8, white queen on b6, white king on c8 — black is stalemated
        let pos = parse_fen("k7/8/1Q6/8/8/8/8/2K5 b - - 0 1").unwrap();
        let moves = legal_moves(pos);
        // Verify it's actually stalemate (no legal moves for black)
        assert!(moves.is_empty());
        let config = SearchConfig { max_depth: 5, time_per_move_ms: 60_000, uci_output: false };
        let result = search_with_time(pos, config);
        assert_eq!(result.nodes, 0);
    }

    #[test]
    fn node_counter_increases_with_depth() {
        let pos = parse_fen("4k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1").unwrap();
        let result_d1 = search_with_time(pos, SearchConfig { max_depth: 1, time_per_move_ms: 60_000, uci_output: false });
        let result_d3 = search_with_time(pos, SearchConfig { max_depth: 3, time_per_move_ms: 60_000, uci_output: false });
        assert!(
            result_d3.nodes > result_d1.nodes,
            "Depth 3 ({} nodes) should search more nodes than depth 1 ({} nodes)",
            result_d3.nodes, result_d1.nodes
        );
    }

    #[test]
    fn captures_hanging_queen() {
        // White pawn on d4, black queen on e5 — pawn can capture diagonally to e5.
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(crate::square::Squares::E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(crate::square::Squares::E8 as u8);
        let (e5_col, e5_row) = get_col_and_row(crate::square::Squares::E5 as u8);
        let (d4_col, d4_row) = get_col_and_row(crate::square::Squares::D4 as u8);

        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[e5_row as usize][e5_col as usize] = Some(BLACK_QUEEN);
        pos.pieces[d4_row as usize][d4_col as usize] = Some(WHITE_PAWN);

        let m = best_move(pos).unwrap();
        assert_eq!(m.from, crate::square::Squares::D4 as u8);
        assert_eq!(m.to, crate::square::Squares::E5 as u8);
        assert!(m.capture);
    }

    #[test]
    fn avoids_moving_into_bishop() {
        // White king on e1, black bishop on a5 (diagonal to e1).
        // White should not move the pawn on d2 to d3 (would expose king).
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(crate::square::Squares::E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(crate::square::Squares::E8 as u8);
        let (a5_col, a5_row) = get_col_and_row(crate::square::Squares::A5 as u8);
        let (d2_col, d2_row) = get_col_and_row(crate::square::Squares::D2 as u8);

        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[a5_row as usize][a5_col as usize] = Some(BLACK_BISHOP);
        pos.pieces[d2_row as usize][d2_col as usize] = Some(WHITE_PAWN);

        let m = best_move(pos).unwrap();
        // The pawn at d2 should NOT move to d3 (that opens the diagonal)
        // It should move to d4 or stay put — but legal_moves already filters.
        // Since d3 is illegal, the best move should not be d3.
        assert!(!(m.from == crate::square::Squares::D2 as u8
            && m.to == crate::square::Squares::D3 as u8));
    }

    #[test]
    fn knight_fork() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(crate::square::Squares::E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(crate::square::Squares::E8 as u8);
        let (b5_col, b5_row) = get_col_and_row(crate::square::Squares::B5 as u8);
        let (a8_col, a8_row) = get_col_and_row(crate::square::Squares::A8 as u8);
        let (_c7_col, _c7_row) = get_col_and_row(crate::square::Squares::C7 as u8);

        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[b5_row as usize][b5_col as usize] = Some(WHITE_KNIGHT);
        pos.pieces[a8_row as usize][a8_col as usize] = Some(BLACK_ROOK);

        let m = best_move_depth(pos, 3).unwrap();
        assert_eq!(m.from, crate::square::Squares::B5 as u8);
        assert_eq!(m.to, crate::square::Squares::C7 as u8);
    }

    #[test]
    fn bishop_fork() {
        let mut pos = empty_position(PieceColor::White);
        let (e1_col, e1_row) = get_col_and_row(crate::square::Squares::E1 as u8);
        let (e8_col, e8_row) = get_col_and_row(crate::square::Squares::E8 as u8);
        let (_b5_col, _b5_row) = get_col_and_row(crate::square::Squares::B5 as u8);
        let (a4_col, a4_row) = get_col_and_row(crate::square::Squares::A4 as u8);
        let (c4_col, c4_row) = get_col_and_row(crate::square::Squares::C4 as u8);

        pos.pieces[e1_row as usize][e1_col as usize] = Some(WHITE_KING);
        pos.pieces[e8_row as usize][e8_col as usize] = Some(BLACK_KING);
        pos.pieces[c4_row as usize][c4_col as usize] = Some(WHITE_BISHOP);
        pos.pieces[a4_row as usize][a4_col as usize] = Some(BLACK_ROOK);

        let m = best_move_depth(pos, 3).unwrap();
        assert_eq!(m.from, crate::square::Squares::C4 as u8);
        assert_eq!(m.to, crate::square::Squares::B5 as u8);
    }

    #[test]
    fn black_saves_queen_after_de7() {
        // Reproduce the game sequence where the engine blundered the queen.
        // White played 6.de7, the pawn on e7 attacks Black's queen on d8.
        // At ANY depth, the engine must save the queen.
        let mut pos = start_position();

        // 1. d4 b6
        make_move(&mut pos, Move::build(WHITE_PAWN, D2 as u8, D4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, B7 as u8, B6 as u8));
        // 2. e4 a6
        make_move(&mut pos, Move::build(WHITE_PAWN, E2 as u8, E4 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A7 as u8, A6 as u8));
        // 3. d5 a5
        make_move(&mut pos, Move::build(WHITE_PAWN, D4 as u8, D5 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A6 as u8, A5 as u8));
        // 4. e5 a4
        make_move(&mut pos, Move::build(WHITE_PAWN, E4 as u8, E5 as u8));
        make_move(&mut pos, Move::build(BLACK_PAWN, A5 as u8, A4 as u8));
        // 5. d6 Ra7
        make_move(&mut pos, Move::build(WHITE_PAWN, D5 as u8, D6 as u8));
        make_move(&mut pos, Move::build(BLACK_ROOK, A8 as u8, A7 as u8));
        // 6. de7 — pawn on d6 captures e7, now attacks d8
        make_move(&mut pos, Move::build(WHITE_PAWN, D6 as u8, E7 as u8).capture());

        // Black to move. Queen under attack by pawn on e7.
        // Depth 3 is instant and should see the hanging queen.
        let result = search_with_time(pos, SearchConfig { max_depth: 3, time_per_move_ms: 30_000, uci_output: false });
        let m = result.best_move.unwrap();
        // The engine must save the queen. It can either move the queen away,
        // or capture the attacking pawn on e7 with any piece (K, Q, B, or N).
        assert!(
            m.piece.piece_type == PieceType::Queen || m.to == E7 as u8,
            "Engine blundered! Should save queen but played {} from {} to {}",
            crate::r#move::print_move(m), m.from, m.to
        );
    }
}
