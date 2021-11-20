use crate::boards::{Bitboard, Board};
use crate::mapping::Square;

use super::{ANTI_DIAG_MOVE_ARR, DIAG_MOVE_ARR, FILE_MOVE_ARR, RANK_MOVE_ARR};

fn rank_ray(sq: Square) -> Bitboard {
  RANK_MOVE_ARR[sq as usize]
}

fn file_ray(sq: Square) -> Bitboard {
  FILE_MOVE_ARR[sq as usize]
}

fn diag_ray(sq: Square) -> Bitboard {
  DIAG_MOVE_ARR[sq as usize]
}

fn anti_diag_ray(sq: Square) -> Bitboard {
  ANTI_DIAG_MOVE_ARR[sq as usize]
}

fn pos_moves_occluded<F>(board: &Board, sq: Square, dir: F) -> Bitboard
  where F: Fn(Square) -> Bitboard {
  let mask = dir(sq);
  let pieces = board.pieces();
  let occupied = mask & pieces;
  let diff = occupied - Bitboard::from(sq) * 2;
  let changed = diff ^ pieces;
  changed & mask
}

fn neg_moves_occluded<F>(board: &Board, sq: Square, dir: F) -> Bitboard
  where F: Fn(Square) -> Bitboard {
  let mask = dir(sq);
  let pieces = board.pieces();
  let occupied = (mask & pieces).swap_bytes();
  let diff = occupied - Bitboard::from(sq).swap_bytes() * 2;
  let changed = diff ^ pieces;
  (changed & mask).swap_bytes()
}

fn moves_occluded<F>(board: &Board, sq: Square, dir: &F) -> Bitboard
  where F: Fn(Square) -> Bitboard {
  pos_moves_occluded(board, sq, dir) | neg_moves_occluded(board, sq, dir)
}
