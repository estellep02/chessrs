use crate::boards::Bitboard;

pub use self::rankfile::*;
pub use self::square::*;

mod square;
mod rankfile;

// LERF mapping
// A1 = 0, H1 = 7, A8 = 56, H8 = 63
pub const FIRST_RANK: Bitboard = Bitboard::from_u64(0x00_00_00_00_00_00_00_FF);
pub const A_FILE: Bitboard = Bitboard::from_u64(0x01_01_01_01_01_01_01_01);
pub const H_FILE: Bitboard = Bitboard::from_u64(0x80_80_80_80_80_80_80_80);
pub const MAIN_DIAG: Bitboard = Bitboard::from_u64(0x80_40_20_10_08_04_02_01);
pub const MAIN_ANTIDIAG: Bitboard = Bitboard::from_u64(0x01_02_04_08_10_20_40_80);