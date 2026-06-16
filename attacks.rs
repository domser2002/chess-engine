use crate::bitboard::Bitboard;

const NOT_A: u64 = 0xfefefefefefefefe;
const NOT_AB: u64 = 0xfcfcfcfcfcfcfcfc;
const NOT_H: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_GH: u64 = 0x3f3f3f3f3f3f3f3f;

const fn king_attacks(square: u8) -> u64 {
    let bb = 1u64 << square;
    let mut attacks = 0;
    
    attacks |= (bb << 8) | (bb >> 8); // N, S
    attacks |= (bb << 1) & NOT_A; // E
    attacks |= (bb >> 1) & NOT_H; // W
    attacks |= (bb << 9) & NOT_A; // NE
    attacks |= (bb << 7) & NOT_H; // NW
    attacks |= (bb >> 7) & NOT_A; // SE
    attacks |= (bb >> 9) & NOT_H; // SW
    
    attacks
}

const fn knight_attacks(square: u8) -> u64 {
    let bb = 1u64 << square;
    let mut attacks = 0;

    attacks |= (bb << 17) & NOT_A; // NNE
    attacks |= (bb << 15) & NOT_H; // NNW
    attacks |= (bb << 10) & NOT_AB; // EEN
    attacks |= (bb << 6)  & NOT_GH; // WWN
    
    attacks |= (bb >> 15) & NOT_A; // SSE
    attacks |= (bb >> 17) & NOT_H; // SSW
    attacks |= (bb >> 6)  & NOT_AB; // EES
    attacks |= (bb >> 10) & NOT_GH; // WWS

    attacks
}

/// Precalculated king attacks for every square
pub const KING_ATTACKS: [Bitboard; 64] = {
    let mut table = [Bitboard::new(0); 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = Bitboard::new(king_attacks(sq as u8));
        sq += 1;
    }
    table
};

/// Precalculated knight attacks for every square
pub const KNIGHT_ATTACKS: [Bitboard; 64] = {
    let mut table = [Bitboard::new(0); 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = Bitboard::new(knight_attacks(sq as u8));
        sq += 1;
    }
    table
};

const fn pawn_attacks(square: u8, is_white: bool) -> u64 {
    let bb = 1u64 << square;
    let mut attacks = 0;
    if is_white {
        attacks |= (bb << 9) & NOT_A; // NE
        attacks |= (bb << 7) & NOT_H; // NW
    } else {
        attacks |= (bb >> 7) & NOT_A; // SE
        attacks |= (bb >> 9) & NOT_H; // SW
    }
    attacks
}

/// Precalculated white pawn attacks for every square
pub const WHITE_PAWN_ATTACKS: [Bitboard; 64] = {
    let mut table = [Bitboard::new(0); 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = Bitboard::new(pawn_attacks(sq as u8, true));
        sq += 1;
    }
    table
};

/// Precalculated black pawn attacks for every square
pub const BLACK_PAWN_ATTACKS: [Bitboard; 64] = {
    let mut table = [Bitboard::new(0); 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = Bitboard::new(pawn_attacks(sq as u8, false));
        sq += 1;
    }
    table
};
