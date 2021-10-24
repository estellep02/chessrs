use crate::boards::Bitboard;

fn knight_moves(square: i32) -> Bitboard {
    let mut offsets = Vec::new();
    if square % 8 > 0 && square > 16 {
        offsets.push(-17);
    }
    if square % 8 > 1 && square > 8 {
        offsets.push(-10);
    }
    if square % 8 > 1 && square < 55 {
        offsets.push(6);
    }
    if square % 8 > 0 && square < 47 {
        offsets.push(15);
    }
    if square % 8 < 7 && square < 47 {
        offsets.push(17);
    }
    if square % 8 < 6 && square < 55 {
        offsets.push(10);
    }
    if square % 8 < 6 && square > 8 {
        offsets.push(-6);
    }
    if square % 8 < 7 && square > 16 {
        offsets.push(-15);
    }
    offsets.iter().fold(Bitboard::new(), |bb, off| {
        let mask = 1 << (square + off);
        Bitboard::from(bb.get_bb() | mask)
    })
}

pub fn gen_knight_moves() -> Vec<Bitboard> {
    (0..64).map(knight_moves).collect()
}