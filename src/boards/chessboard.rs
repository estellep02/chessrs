use crate::boards::Bitboard;

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
    let w_pawns = Bitboard::from(0xFF << 8);
    let w_knights = Bitboard::from((1 << 1) | (1 << 6));
    let w_bishops = Bitboard::from((1 << 2) | (1 << 5));
    let w_rooks = Bitboard::from(1 | (1 << 7));
    let w_queens = Bitboard::from(1 << 3);
    let w_king = Bitboard::from(1 << 4);

    let b_pawns = Bitboard::from(0xFF << 48);
    let b_knights = Bitboard::from((1 << 62) | (1 << 57));
    let b_bishops = Bitboard::from((1 << 61) | (1 << 58));
    let b_rooks = Bitboard::from((1 << 63) | (1 << 56));
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
      b_king,
    }
  }

  pub fn w_pawns(&self) -> Bitboard {
    self.w_pawns
  }

  pub fn w_knights(&self) -> Bitboard {
    self.w_knights
  }

  pub fn w_bishops(&self) -> Bitboard {
    self.w_bishops
  }

  pub fn w_rooks(&self) -> Bitboard {
    self.w_rooks
  }

  pub fn w_queens(&self) -> Bitboard {
    self.w_queens
  }

  pub fn w_king(&self) -> Bitboard {
    self.w_king
  }

  pub fn w_pieces(&self) -> Bitboard {
    self.w_pawns() |
        self.w_knights() |
        self.w_bishops() |
        self.w_rooks() |
        self.w_queens() |
        self.w_king()
  }

  pub fn b_pawns(&self) -> Bitboard {
    self.b_pawns
  }

  pub fn b_knights(&self) -> Bitboard {
    self.b_knights
  }

  pub fn b_bishops(&self) -> Bitboard {
    self.b_bishops
  }

  pub fn b_rooks(&self) -> Bitboard {
    self.b_rooks
  }

  pub fn b_queens(&self) -> Bitboard {
    self.b_queens
  }

  pub fn b_king(&self) -> Bitboard {
    self.b_king
  }

  pub fn b_pieces(&self) -> Bitboard {
    self.b_pawns() |
        self.b_knights() |
        self.b_bishops() |
        self.b_rooks() |
        self.b_queens() |
        self.b_king()
  }

  pub fn pieces(&self) -> Bitboard {
    self.w_pieces() | self.b_pieces()
  }
}