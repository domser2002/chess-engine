use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

/// Represents a chess board using 64 bits.
/// Square 0 is A1, Square 63 is H8.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct Bitboard(pub u64);

/// Little-Endian Rank-File (LERF) Square indices for Bitboards
#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Index {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Bitboard {
    /// Create a new bitboard from a raw u64.
    #[inline]
    pub const fn new(val: u64) -> Self {
        Bitboard(val)
    }

    /// Create an empty bitboard.
    #[inline]
    pub const fn empty() -> Self {
        Bitboard(0)
    }

    /// Return true if the bitboard is completely empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Set a bit at the given square index (0..64).
    #[inline]
    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    /// Clear a bit at the given square index (0..64).
    #[inline]
    pub fn clear_bit(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }

    /// Test if a bit is set at the given square index (0..64).
    #[inline]
    pub fn test_bit(&self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    /// Returns the number of bits set to 1.
    #[inline]
    pub fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the index (0..64) of the least significant bit.
    /// Warning: Calling this on an empty bitboard returns 64.
    #[inline]
    pub fn lsb(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    /// Returns the index of the least significant bit, and clears it.
    /// Warning: Calling this on an empty bitboard returns 64.
    #[inline]
    pub fn pop_lsb(&mut self) -> u8 {
        let sq = self.lsb();
        self.0 &= self.0.wrapping_sub(1);
        sq
    }
}

// Allows using a `for sq in bitboard { ... }` loop.
impl Iterator for Bitboard {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            Some(self.pop_lsb())
        }
    }
}

// Bitwise operations for Bitboard

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Bitboard(!self.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self {
        Bitboard(self.0 >> rhs)
    }
}

