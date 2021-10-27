use crate::boards::Bitboard;
use crate::mapping::{A_FILE, FIRST_RANK, MAIN_ANTIDIAG, MAIN_DIAG, RankFile, Square};
use crate::pieces::rook_moves;


fn rank_mask(sq: Square) -> u64 {
    0xFF << (sq as u64 & 56)
}

fn file_mask(sq: Square) -> u64 {
    0xFF << (sq as u64 & 7)
}

fn diag_ray(sq: Square) -> u64 {
    let diag: i64 = (8 * (sq as i64 & 7) - (sq as i64 & 56)) as i64;
    let nort = -diag & ( diag >> 31);
    let sout =  diag & (-diag >> 31);
    ((MAIN_DIAG >> sout) << nort) as u64
}

fn anti_diag_ray(sq: Square) -> u64 {
    let diag: i64 = (56 - 8 * (sq as i64 & 7) - (sq as i64 & 56)) as i64;
    let nort = -diag & (diag >> 31);
    let sout =  diag & (-diag >> 31);
    ((MAIN_ANTIDIAG >> sout) << nort) as u64
}


pub fn bishop_moves(square: Square) -> Bitboard {
    Bitboard::from((diag_ray(square) ^ anti_diag_ray(square)) & !(1 << square as u64))
}

pub fn gen_bishop_moves() -> Vec<Bitboard> {
    (0..64).map(Square::from).map(bishop_moves).collect()
}