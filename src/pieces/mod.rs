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

pub fn move_occlusion(move_origin: Square, move_bitboard: Bitboard, occupancy_bitboard: Bitboard) -> Bitboard {

    Bitboard::from(0)

}