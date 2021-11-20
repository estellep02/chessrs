use crate::boards::Bitboard;
use crate::mapping::{A_FILE, FIRST_RANK, MAIN_ANTIDIAG, MAIN_DIAG, Square};

fn rank_ray(sq: Square) -> Bitboard {
  FIRST_RANK << (8 * sq.rank())
}

fn file_ray(sq: Square) -> Bitboard {
  A_FILE << (8 * sq.file())
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

pub fn gen_rank_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(rank_ray).collect()
}

pub fn gen_file_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(file_ray).collect()
}

pub fn gen_diag_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(diag_ray).collect()
}

pub fn gen_anti_diag_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(anti_diag_ray).collect()
}