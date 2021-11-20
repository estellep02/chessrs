use crate::boards::Bitboard;
use lazy_static::lazy_static;

pub use self::bishop::*;
pub use self::bishop::*;
pub use self::king::*;
pub use self::knight::*;
pub use self::knight::*;
pub use self::pawn::*;
pub use self::queen::*;
pub use self::queen::*;
pub use self::rays::*;
pub use self::rook::*;
pub use self::rook::*;

mod pawn;
mod knight;
mod bishop;
mod rook;
mod queen;
mod king;
mod rays;
mod occlusion;

lazy_static! {
    pub static ref KNIGHT_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_knight_moves();
    pub static ref KING_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_king_moves();
    pub static ref RANK_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_rank_moves();
    pub static ref FILE_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_file_moves();
    pub static ref DIAG_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_diag_moves();
    pub static ref ANTI_DIAG_MOVE_ARR: Vec<Bitboard> = super::pieces::gen_anti_diag_moves();
}