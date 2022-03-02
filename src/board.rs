use crate::piece::Piece;
use crate::board_2d::{Board2D};
use crate::movement::Movement;

/// Represents a board state 
#[derive(Debug, Clone)]
pub struct Board {
    /// Width of the board
    width : usize,

    /// Height of the board
    height : usize,

    /// List of pieces on the board
    pieces : Vec<Piece>,

    /// 2D board constructed from the list of pieces
    board_2d : Option<Board2D>,
}

impl Board {

    pub fn new(width : usize, height : usize, pieces : &[Piece]) -> Board {
        let mut b = Board { 
            width, 
            height, 
            pieces : pieces.to_vec(),
            board_2d : None
        };
        b.update_2d_board();
        b
    }

    /// Get the 2D board
    pub fn get_2d_board(& self) -> Board2D {
        self.board_2d.as_ref().unwrap().clone()
    }

    /// Move the piece according to the information in the "mov" parameter
    pub fn move_piece(&mut self, mov : Movement) {
        self.pieces[mov.piece_id].move_piece_direction(mov.direction);
        self.update_2d_board();
    }

    /// Get all possible movements for all pieces on the board
    pub fn get_all_possible_movements(&self) -> Vec<Movement> {
        self.get_2d_board().get_all_possible_movements(&self.pieces)
    }

    /// Heuristic to determine the board with the most potential in a collection
    pub fn solved_heuristic(&self) -> u32 {
        // The further it is , the closer we are to the solution
        self.pieces.first().unwrap().y_pos as u32
    }

    /// Determines if this board is solved
    pub fn is_solved(&self) -> bool {
        // The further it is , the closer we are to the solution
        self.pieces.first().unwrap().y_pos  == 3 && self.pieces.first().unwrap().x_pos  == 1
    }

    /// Recalculates the 2D board
    fn update_2d_board(&mut self) {
        self.board_2d = Some(Board2D::new(self.width, self.height, &self.pieces));
    }

}

