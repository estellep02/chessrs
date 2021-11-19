use std::fmt::Formatter;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use lazy_static::lazy_static;

mod bitboard;
mod chessboard;

pub use self::bitboard::*;
pub use self::chessboard::*;

lazy_static! {
    pub static ref KNIGHT_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_knight_moves();
    pub static ref ROOK_PREMOVE_TBL: Vec<Bitboard> = super::pieces::gen_rook_moves();
    pub static ref BISHOP_PREMOVE_TBL: Vec<Bitboard> = super::pieces::gen_bishop_moves();
    pub static ref QUEEN_PREMOVE_TBL: Vec<Bitboard> = super::pieces::gen_queen_moves();
}