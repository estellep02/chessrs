use chessrs::boards;
use chessrs::boards::Board;
use chessrs::mapping::Square;
use chessrs::pieces::sliding_move_occlusion;

fn main() {
    let board = Board::init();
    let rma = &boards::ROOK_PREMOVE_TBL;
    println!("{}", rma[0]);
    println!("{}", sliding_move_occlusion(Square::A1, rma[0], board.w_pieces(), board.b_pieces()));
}
