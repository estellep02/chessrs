use lazy_static::lazy_static;

mod bitboard;
mod chessboard;

pub use self::bitboard::*;
pub use self::chessboard::*;

lazy_static! {
    pub static ref KNIGHT_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_knight_moves();
    pub static ref KING_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_king_moves();
    pub static ref RANK_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_rank_moves();
    pub static ref FILE_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_file_moves();
    pub static ref DIAG_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_diag_moves();
    pub static ref ANTI_DIAG_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_anti_diag_moves();
}