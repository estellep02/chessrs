use chessrs::boards;
use chessrs::boards::Board;
use chessrs::pieces::rook_moves;

fn main() {
    let board = Board::init();
    let kma = &boards::KNIGHT_MOVE_ARR;
    let rma = &boards::ROOK_PREMOVE_TBL;
    let bma = &boards::BISHOP_PREMOVE_TBL;
    let qma = &boards::QUEEN_PREMOVE_TBL;
    println!("{}", kma[0]);
    println!("{}", rma[0]);
    println!("{}", bma[0]);
    println!("{}", qma[0]);
    println!("{}", board.occupancy());
}
