
pub mod piece;
pub mod board;
pub mod board_2d;

use crate::board::Board;
use crate::piece::Piece;

fn main() {
    let mut b : Board = Board::new(4, 5);
    // Big square
    b.pieces.push(Piece::new(1, 0, 2, 2, '0'));
    // All vertical pieces
    b.pieces.push(Piece::new(0, 0, 1, 2, '1'));
    b.pieces.push(Piece::new(3, 0, 1, 2, '2'));
    b.pieces.push(Piece::new(0, 3, 1, 2, '3'));
    b.pieces.push(Piece::new(3, 3, 1, 2, '4'));
    // Horizontal piece
    b.pieces.push(Piece::new(1, 2, 2, 1, '5'));
    // Small squares
    b.pieces.push(Piece::new(1, 3, 1, 1, '6'));
    b.pieces.push(Piece::new(2, 3, 1, 1, '7'));
    b.pieces.push(Piece::new(1, 4, 1, 1, '8'));
    b.pieces.push(Piece::new(2, 4, 1, 1, '9'));

    let board2d = b.get_2d_board();
    let possible_movements = board2d.get_all_possible_movements(&b.pieces);
    board2d.print();
    println!("\r\n");
    println!("{:?}", possible_movements);
}
