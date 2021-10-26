use chessrs::boards;
use chessrs::pieces::rook_moves;

fn main() {
    let kma = &boards::KNIGHT_MOVE_ARR;
    let rma = &boards::ROOK_PREMOVE_TBL;
    println!("{}", kma[0]);
    println!("{}", rma[32]);
}
