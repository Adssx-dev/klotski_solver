
pub mod piece;
pub mod board;
pub mod board_2d;
pub mod solver;
pub mod direction;
pub mod movement;

use crate::board::Board;
use crate::piece::Piece;
use crate::solver::Solver;

fn main() {

    let mut pieces : Vec<Piece>= Vec::new();
    // Big square
    pieces.push(Piece::new(1, 0, 2, 2, '0'));
    // All vertical pieces
    pieces.push(Piece::new(0, 0, 1, 2, '1'));
    pieces.push(Piece::new(3, 0, 1, 2, '2'));
    pieces.push(Piece::new(0, 3, 1, 2, '3'));
    pieces.push(Piece::new(3, 3, 1, 2, '4'));
    // Horizontal piece
    pieces.push(Piece::new(1, 2, 2, 1, '5'));
    // Small squares
    pieces.push(Piece::new(1, 3, 1, 1, '6'));
    pieces.push(Piece::new(2, 3, 1, 1, '7'));
    pieces.push(Piece::new(1, 4, 1, 1, '8'));
    pieces.push(Piece::new(2, 4, 1, 1, '9'));

    let mut b : Board = Board::new(4, 5, &pieces);

    let board2d = b.get_2d_board();
    let possible_movements = b.get_all_possible_movements();
    board2d.print();
    println!("\r\n");
    println!("{:?}", possible_movements);
}
