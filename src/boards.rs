use std::borrow::Cow::Borrowed;
use std::fmt::Formatter;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KNIGHT_MOVE_ARR: Vec<Bitboard> = gen_knight_moves();
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bitboard {
    bb: u64,
}

impl Bitboard {
    // pub fn new(bb: u64) -> Bitboard {
    //     Bitboard { bb }
    // }
}

impl From<u64> for Bitboard {
    fn from(bb: u64) -> Self {
        Bitboard { bb }
    }
}

pub struct Board {
    w_pawns: Bitboard,
    w_knights: Bitboard,
    w_bishops: Bitboard,
    w_rooks: Bitboard,
    w_queens: Bitboard,
    w_king: Bitboard,
    
    b_pawns: Bitboard,
    b_knights: Bitboard,
    b_bishops: Bitboard,
    b_rooks: Bitboard,
    b_queens: Bitboard,
    b_king: Bitboard,
}

impl Board {
    pub fn init() -> Board {
        let w_pawns = Bitboard::from(0x7 << 8);
        let w_knights = Bitboard::from(((1 << 1) | (1 << 6)));
        let w_bishops = Bitboard::from((1 << 2) | (1 << 5));
        let w_rooks = Bitboard::from((1 | (1 << 7)));
        let w_queens = Bitboard::from(1 << 3);
        let w_king = Bitboard::from(1 << 4);

        let b_pawns = Bitboard::from(0x7 << 48);
        let b_knights = Bitboard::from((1 << 62) | (1 << 57));
        let b_bishops = Bitboard::from((1 << 61) | (1 << 58));
        let b_rooks= Bitboard::from((1 << 63) | (1 << 56));
        let b_queens = Bitboard::from(1 << 59);
        let b_king = Bitboard::from(1 << 60);
        Board {
            w_pawns,
            w_knights,
            w_bishops,
            w_rooks,
            w_queens,
            w_king,
            b_pawns,
            b_knights,
            b_bishops,
            b_rooks,
            b_queens,
            b_king
        }
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..=7 {
            let shift = (7 - i) * 8;
            let mask = 0xff << shift;
            let bits = ((self.bb & mask) >> shift) as u8;
            writeln!(f, "{:08b}", bits.reverse_bits())?;
        }
        Ok(())
    }
}

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
    offsets.iter().fold(Bitboard { bb: 0 }, |bb, off| {
        let mask = 1 << (square + off);
        Bitboard { bb: bb.bb | mask }
    })
}

fn gen_knight_moves() -> Vec<Bitboard> {
    (0..64).map(knight_moves).collect()
}

pub fn is_single_square(bitboard: Bitboard) -> bool {
    let bb = bitboard.bb;
    bb != 0 && (bb & (bb - 1) == 0)
}