use crate::boards::Bitboard;
use crate::mapping::{MAIN_ANTIDIAG, MAIN_DIAG, Square};

fn rank_mask(sq: Square) -> u64 {
  0xFF << (sq as u64 & 56)
}

fn file_mask(sq: Square) -> u64 {
  0xFF << (sq as u64 & 7)
}

fn diag_ray(sq: Square) -> Bitboard {
  let diag: i64 = (8 * (sq as i64 & 7) - (sq as i64 & 56)) as i64;
  let north = -diag & (diag >> 31);
  let south = diag & (-diag >> 31);
  (MAIN_DIAG >> south) << north
}

fn anti_diag_ray(sq: Square) -> Bitboard {
  let diag: i64 = (56 - 8 * (sq as i64 & 7) - (sq as i64 & 56)) as i64;
  let north = -diag & (diag >> 31);
  let south = diag & (-diag >> 31);
  (MAIN_ANTIDIAG >> south) << north
}


pub fn bishop_moves(square: Square) -> Bitboard {
  (diag_ray(square) ^ anti_diag_ray(square)).exclude_square(square)
}

pub fn gen_bishop_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(bishop_moves).collect()
}