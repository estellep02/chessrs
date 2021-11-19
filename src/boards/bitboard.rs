use crate::mapping::Square;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::fmt::Formatter;


#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bitboard {
    bb: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard { bb: 0 }
    }

    pub fn merge(self, other: Bitboard) -> Bitboard {
        self | other
    }

    fn is_square_occupied(&self, sq: Square) -> bool {
        (self.bb >> (sq as u64) & 1) == 1
    }

    pub(crate) fn all_squares_occupied(&self) -> Vec<Square> {
        let mut res = Vec::new();
        for i in 0..64 {
            let square = Square::from(i);
            if self.is_square_occupied(square) {
                res.push(square);
            }
        }
        res
    }

    pub(crate) fn get_bb(&self) -> u64 {
        self.bb
    }

    pub fn is_single_square(&self) -> bool {
        let bb = self.bb;
        bb != 0 && (bb & (bb - 1) == 0)
    }
}

impl From<u64> for Bitboard {
    fn from(bb: u64) -> Self {
        Bitboard { bb }
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard::from(self.bb | rhs.bb)
    }
}

impl BitOr<Square> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Square) -> Self::Output {
        Bitboard::from(self.bb | 1 << (rhs as u64))
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard::from(self.bb & rhs.bb)
    }
}

impl BitAnd<Square> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Square) -> Self::Output {
        Bitboard::from(self.bb & 1 << (rhs as u64))
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard::from(self.bb ^ rhs.bb)
    }
}

impl BitXor<Square> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Square) -> Self::Output {
        Bitboard::from(self.bb ^ 1 << (rhs as u64))
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard::from(!self.bb)
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  ABCDEFGH");
        for i in 0..=7 {
            let shift = (7 - i) * 8;
            let mask = 0xff << shift;
            let bits = ((self.bb & mask) >> shift) as u8;
            writeln!(f, "{}|{:08b}|{}", 8-i, bits.reverse_bits(), 8-i)?;
        }
        writeln!(f, "  ABCDEFGH");
        Ok(())
    }
}