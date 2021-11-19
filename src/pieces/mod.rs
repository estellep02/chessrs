mod knight;
mod rook;
mod bishop;
mod queen;

use crate::boards::Bitboard;
use crate::mapping::Square;
pub use self::knight::*;
pub use self::rook::*;
pub use self::bishop::*;
pub use self::queen::*;

pub fn sliding_move_occlusion(move_origin: Square, move_bitboard: Bitboard, w_occ_bb: Bitboard, b_occ_bb: Bitboard) -> Bitboard {


  Bitboard::from(0)
}