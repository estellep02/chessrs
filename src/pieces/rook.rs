use crate::boards::Bitboard;
use crate::mapping::{A_FILE, FIRST_RANK, RankFile, Square};

pub fn rook_moves(square: Square) -> Bitboard {
  // rank needs to be shifted by multiples of 8
  // file needs to be shifted by multiples of 1

  let rf = RankFile::from(square);

  FIRST_RANK << (8 * rf.rank as i64) ^ A_FILE << (rf.file as i64)
}

pub fn gen_rook_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(rook_moves).collect()
}