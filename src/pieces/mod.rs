use crate::boards::Bitboard;
use crate::mapping::Square;

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

pub fn sliding_move_occlusion(move_origin: Square, move_bitboard: Bitboard, w_occ_bb: Bitboard, b_occ_bb: Bitboard) -> Bitboard {
  Bitboard::from(0)
}