use crate::piece::Piece;
use crate::board_2d::Board2D;

pub struct Board {
    pub width : usize,
    pub height : usize,
    pub pieces : Vec<Piece>
}

impl Board {

    pub fn new(width : usize, height : usize) -> Board {
        Board { 
            width, 
            height, 
            pieces: Vec::new()
        }
    }

    pub fn get_2d_board(&self) -> Board2D {
        Board2D::new(self.width, self.height, &self.pieces)
    }
}

