use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

/// Represents a chess board using 64 bits.
/// Square 0 is A1, Square 63 is H8.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct Bitboard(pub u64);

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

