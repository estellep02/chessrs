use crate::boards::Bitboard;
use crate::mapping::Square;
use crate::pieces::bb_from_offsets;

fn king_moves(square: Square) -> Bitboard {
  let mut offsets: Vec<i64> = Vec::new();
  let square_val = square as i64;
  if square_val % 8 > 0 && square_val <= 55 {  // not a file, not 8th rank
    offsets.push(7);
  }
  if square_val <= 55 {                        // not 8th rank
    offsets.push(8);
  }
  if square_val % 8 < 7 && square_val <= 55 {  // not h file, not 8th rank
    offsets.push(9);
  }
  if square_val % 8 < 7 {                      // not h file
    offsets.push(1);
  }
  if square_val % 8 < 7 && square_val >= 8 {   // not h file, not 1st rank
    offsets.push(-7);
  }
  if square_val >= 8 {                         // not 1st rank
    offsets.push(-8);
  }
  if square_val % 8 > 0 && square_val >= 8 {   // not a file, not 1st rank
    offsets.push(-9);
  }
  if square_val % 8 > 0 {                      // not a file
    offsets.push(-1);
  }
  bb_from_offsets(offsets, square_val)
}

pub fn gen_king_moves() -> Vec<Bitboard> {
  (0..64).map(Square::from).map(king_moves).collect()
}