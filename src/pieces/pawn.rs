use crate::boards::Bitboard;
use crate::boards::Board;
use crate::mapping::{A_FILE, H_FILE};

fn w_pawn_moves(board: Board) -> Bitboard {
  (board.w_pawns() << 8) & !board.pieces()
}

fn w_pawn_attacks(board: &Board) -> Bitboard {
  let left_attacks = (board.w_pawns() & !A_FILE) << 7;
  let right_attacks = (board.w_pawns() & !H_FILE) << 9;
  left_attacks | right_attacks
}

fn w_pawn_captures(board: &Board) -> Bitboard {
  w_pawn_attacks(board) & board.b_pieces()
}

fn b_pawn_moves(board: &Board) -> Bitboard {
  (board.b_pawns() >> 8) & !board.pieces()
}

fn b_pawn_attacks(board: &Board) -> Bitboard {
  let left_attacks = (board.b_pawns() & !A_FILE) >> 9;
  let right_attacks = (board.b_pawns() & !H_FILE) >> 7;
  left_attacks | right_attacks
}

fn b_pawn_captures(board: &Board) -> Bitboard {
  b_pawn_attacks(board) & board.w_pieces()
}