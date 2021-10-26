use crate::boards::{Bitboard};
use crate::mapping::Square;
use crate::mapping::Square::{A1, H7};

fn knight_moves(square: Square) -> Bitboard {
    let mut offsets: Vec<i64> = Vec::new();
    let square_val = square as i64;
    if square_val % 8 > 0 && square_val > 16 {
        offsets.push(-17);
    }
    if square_val % 8 > 1 && square_val > 8 {
        offsets.push(-10);
    }
    if square_val % 8 > 1 && square_val < 55 {
        offsets.push(6);
    }
    if square_val % 8 > 0 && square_val < 47 {
        offsets.push(15);
    }
    if square_val % 8 < 7 && square_val < 47 {
        offsets.push(17);
    }
    if square_val % 8 < 6 && square_val < 55 {
        offsets.push(10);
    }
    if square_val % 8 < 6 && square_val > 8 {
        offsets.push(-6);
    }
    if square_val % 8 < 7 && square_val > 16 {
        offsets.push(-15);
    }
    offsets.iter().fold(Bitboard::new(), |bb, off| {
        let mask = 1 << (square_val + off);
        Bitboard::from(bb.get_bb() | mask)
    })
}

pub fn gen_knight_moves() -> Vec<Bitboard> {
    (0..64).map(Square::from).map(knight_moves).collect()
}