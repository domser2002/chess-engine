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

// --- MAGIC BITBOARDS ---

use std::sync::OnceLock;
use crate::magics::{ROOK_MAGICS, ROOK_MASKS, BISHOP_MAGICS, BISHOP_MASKS};

static ROOK_TABLE: OnceLock<Vec<Vec<Bitboard>>> = OnceLock::new();
static BISHOP_TABLE: OnceLock<Vec<Vec<Bitboard>>> = OnceLock::new();

fn set_occupancy(index: usize, bits_in_mask: u32, mut mask: u64) -> u64 {
    let mut occupancy = 0;
    for i in 0..bits_in_mask {
        let square = mask.trailing_zeros();
        mask &= mask - 1;
        if (index & (1 << i)) != 0 {
            occupancy |= 1u64 << square;
        }
    }
    occupancy
}

fn classic_rook_attacks(sq: u8, block: u64) -> u64 {
    let r = sq / 8;
    let c = sq % 8;
    let mut attacks = 0u64;
    for i in (r + 1)..8 { attacks |= 1u64 << (i * 8 + c); if (block & (1u64 << (i * 8 + c))) != 0 { break; } }
    for i in (0..r).rev() { attacks |= 1u64 << (i * 8 + c); if (block & (1u64 << (i * 8 + c))) != 0 { break; } }
    for j in (c + 1)..8 { attacks |= 1u64 << (r * 8 + j); if (block & (1u64 << (r * 8 + j))) != 0 { break; } }
    for j in (0..c).rev() { attacks |= 1u64 << (r * 8 + j); if (block & (1u64 << (r * 8 + j))) != 0 { break; } }
    attacks
}

fn classic_bishop_attacks(sq: u8, block: u64) -> u64 {
    let r = sq as i32 / 8;
    let c = sq as i32 % 8;
    let mut attacks = 0u64;
    for (tr, tc) in [(1,1), (1,-1), (-1,1), (-1,-1)].iter() {
        let mut nr = r + tr;
        let mut nc = c + tc;
        while nr >= 0 && nr <= 7 && nc >= 0 && nc <= 7 {
            attacks |= 1u64 << (nr * 8 + nc);
            if (block & (1u64 << (nr * 8 + nc))) != 0 { break; }
            nr += tr;
            nc += tc;
        }
    }
    attacks
}

fn init_rook_table() -> Vec<Vec<Bitboard>> {
    let mut table = vec![vec![Bitboard::new(0); 4096]; 64];
    for sq in 0..64 {
        let mask = ROOK_MASKS[sq];
        let magic = ROOK_MAGICS[sq];
        let bits_in_mask = mask.count_ones();
        let num_occupancies = 1 << bits_in_mask;
        for i in 0..num_occupancies {
            let occ = set_occupancy(i, bits_in_mask, mask);
            let index = (occ.wrapping_mul(magic)) >> (64 - bits_in_mask);
            table[sq][index as usize] = Bitboard::new(classic_rook_attacks(sq as u8, occ));
        }
    }
    table
}

fn init_bishop_table() -> Vec<Vec<Bitboard>> {
    let mut table = vec![vec![Bitboard::new(0); 512]; 64];
    for sq in 0..64 {
        let mask = BISHOP_MASKS[sq];
        let magic = BISHOP_MAGICS[sq];
        let bits_in_mask = mask.count_ones();
        let num_occupancies = 1 << bits_in_mask;
        for i in 0..num_occupancies {
            let occ = set_occupancy(i, bits_in_mask, mask);
            let index = (occ.wrapping_mul(magic)) >> (64 - bits_in_mask);
            table[sq][index as usize] = Bitboard::new(classic_bishop_attacks(sq as u8, occ));
        }
    }
    table
}

pub fn get_rook_attacks(sq: usize, occupancy: Bitboard) -> Bitboard {
    let table = ROOK_TABLE.get_or_init(init_rook_table);
    let mask = ROOK_MASKS[sq];
    let magic = ROOK_MAGICS[sq];
    let occ = occupancy.0 & mask;
    let index = (occ.wrapping_mul(magic)) >> (64 - mask.count_ones());
    table[sq][index as usize]
}

pub fn get_bishop_attacks(sq: usize, occupancy: Bitboard) -> Bitboard {
    let table = BISHOP_TABLE.get_or_init(init_bishop_table);
    let mask = BISHOP_MASKS[sq];
    let magic = BISHOP_MAGICS[sq];
    let occ = occupancy.0 & mask;
    let index = (occ.wrapping_mul(magic)) >> (64 - mask.count_ones());
    table[sq][index as usize]
}

pub fn get_queen_attacks(sq: usize, occupancy: Bitboard) -> Bitboard {
    get_rook_attacks(sq, occupancy) | get_bishop_attacks(sq, occupancy)
}
