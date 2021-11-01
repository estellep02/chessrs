use crate::boards::Bitboard;
use crate::mapping::Square;
use crate::pieces::{bishop_moves, rook_moves};

pub fn queen_moves(square: Square) -> Bitboard {
    let rook_move = rook_moves(square);
    let bishop_move = bishop_moves(square);

    rook_move.merge(bishop_move)
}

pub fn gen_queen_moves() -> Vec<Bitboard> {
    (0..64).map(Square::from).map(queen_moves).collect()
}