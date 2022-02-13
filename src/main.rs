
pub mod piece;
pub mod board;
pub mod board_2d;

use crate::board::Board;
use crate::piece::Piece;

fn main() {
    let mut b : Board = Board::new(4, 5);
    b.pieces.push(Piece::new(1, 1, 2, 2, 'a'));
    let board2d = b.get_2d_board();
    board2d.print();
}
