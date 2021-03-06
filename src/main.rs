
pub mod piece;
pub mod board;
pub mod board_2d;
pub mod solver;
pub mod direction;
pub mod movement;

use std::io;

use crate::board::Board;
use crate::piece::Piece;
use crate::solver::Solver;

fn main() {

    let mut pieces : Vec<Piece>= Vec::new();
    // Big square
    pieces.push(Piece::new(1, 0, 2, 2, 'G'));
    // All vertical pieces
    pieces.push(Piece::new(0, 0, 1, 2, 'V'));
    pieces.push(Piece::new(3, 0, 1, 2, 'V'));
    pieces.push(Piece::new(0, 3, 1, 2, 'V'));
    pieces.push(Piece::new(3, 3, 1, 2, 'V'));
    // Horizontal piece
    pieces.push(Piece::new(1, 2, 2, 1, 'H'));
    // Small squares
    pieces.push(Piece::new(1, 3, 1, 1, 'S'));
    pieces.push(Piece::new(2, 3, 1, 1, 'S'));
    pieces.push(Piece::new(1, 4, 1, 1, 'S'));
    pieces.push(Piece::new(2, 4, 1, 1, 'S'));

    let b : Board = Board::new(4, 5, &pieces);

    let mut s = Solver::new(b);
    let mut result_board = s.solve();
    let mut _discard = String::new();
    let stdin = io::stdin();
    while let Some(v) = result_board.pop() {
        v.get_2d_board().print();
        let _r = stdin.read_line(&mut _discard);
    }

}
