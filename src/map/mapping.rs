use std::ops::{BitAnd, BitOr, BitXor, Not};
use crate::boards::Bitboard;

// LERF mapping
// A1 = 0, H1 = 7, A8 = 56, H8 = 63
pub const FIRST_RANK: u64 = 0x00000000000000FF;
pub const A_FILE: u64 = 0x0101010101010101;
pub const MAIN_DIAG: u64 = 0x8040201008040201;
pub const MAIN_ANTIDIAG: u64 = 0x0102040810204080;