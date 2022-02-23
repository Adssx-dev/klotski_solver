use crate::piece::Piece;
use crate::board_2d::{Board2D};
use crate::movement::Movement;

#[derive(Debug)]
pub struct Board {
    width : usize,
    height : usize,
    pieces : Vec<Piece>,
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

    pub fn get_2d_board(& self) -> Board2D {
        self.board_2d.as_ref().unwrap().clone()
    }

    pub fn move_piece(&mut self, mov : Movement) {
        self.pieces[mov.piece_id].move_piece_direction(mov.direction);
        self.update_2d_board();
    }

    pub fn get_all_possible_movements(&self) -> Vec<Movement> {
        self.get_2d_board().get_all_possible_movements(&self.pieces)
    }

    fn update_2d_board(&mut self) {
        self.board_2d = Some(Board2D::new(self.width, self.height, &self.pieces));
    }

}

