use crate::boards::Bitboard;
use crate::mapping::Square;

fn knight_moves(square: Square) -> Bitboard {
  let mut offsets: Vec<i64> = Vec::new();
  let square_val = square as i64;
  if square_val % 8 > 0 && square_val >= 16 {  // not a file, not 1st/2nd rank
    offsets.push(-17);
  }
  if square_val % 8 > 1 && square_val >= 8 {   // not a/b files, not 1st rank
    offsets.push(-10);
  }
  if square_val % 8 > 1 && square_val <= 55 {  // not a/b files, not 8th rank
    offsets.push(6);
  }
  if square_val % 8 > 0 && square_val <= 47 {  // not a file, not 7th/8th rank
    offsets.push(15);
  }
  if square_val % 8 < 7 && square_val <= 47 {  // not h file, not 7th/8th rank
    offsets.push(17);
  }
  if square_val % 8 < 6 && square_val <= 55 {  // not g/h files, not 8th rank
    offsets.push(10);
  }
  if square_val % 8 < 6 && square_val >= 8 {   // not g/h files, not 1st rank
    offsets.push(-6);
  }
  if square_val % 8 < 7 && square_val >= 16 {  // not h file, not 1st/2nd rank
    offsets.push(-15);
  }
  offsets.iter().fold(Bitboard::new(), |bb, off| {
    let mask = 1 << (square_val + off);
    Bitboard::from(bb.bb() | mask)
  })
}

pub fn gen_knight_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(knight_moves).collect()
}