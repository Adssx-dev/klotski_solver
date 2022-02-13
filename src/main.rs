
pub mod piece;
pub mod board;
pub mod board_2d;

use crate::board::Board;
use crate::piece::Piece;

fn main() {
    let mut b : Board = Board::new(4, 5);
    // Big square
    b.pieces.push(Piece::new(1, 0, 2, 2, 'G'));
    // All vertical pieces
    b.pieces.push(Piece::new(0, 0, 1, 2, 'V'));
    b.pieces.push(Piece::new(3, 0, 1, 2, 'V'));
    b.pieces.push(Piece::new(0, 3, 1, 2, 'V'));
    b.pieces.push(Piece::new(3, 3, 1, 2, 'V'));
    // Horizontal piece
    b.pieces.push(Piece::new(1, 2, 2, 1, 'H'));
    // Small squares
    b.pieces.push(Piece::new(1, 3, 1, 1, 'S'));
    b.pieces.push(Piece::new(2, 3, 1, 1, 'S'));
    b.pieces.push(Piece::new(1, 4, 1, 1, 'S'));
    b.pieces.push(Piece::new(2, 4, 1, 1, 'S'));

    let board2d = b.get_2d_board();
    board2d.print();
}
