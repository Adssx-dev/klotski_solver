use std::os::windows::prelude::OsStrExt;

use crate::piece::Piece;

// Used to construct the "visual" board in 2D based on pieces positions and sizes
pub struct Board2D {
    width : usize,
    height : usize,
    board : Vec<char>
}

impl Board2D {
    pub fn new(width : usize, height : usize, pieces : &Vec<Piece>) -> Board2D {
        let mut result = Board2D {
            width,
            height,
            board : vec!('.'; width * height)
        };
        result.fill_board(pieces);
        result
    }

    fn fill_board(&mut self, pieces : &Vec<Piece>) {
        for piece in pieces {
            assert!(piece.x_pos + piece.width < self.width, "Piece getting over the edge horizontally. Piece : {:?}", piece);
            assert!(piece.y_pos + piece.height < self.height, "Piece getting over the edge vertically. Piece : {:?}", piece);

            for i in piece.x_pos..(piece.x_pos + piece.width) {
                for j in piece.y_pos..(piece.y_pos + piece.height) {
                    self.board[j * self.width + i] = piece.marker;
                }
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.board.len() {
            if i % self.width == 0 {
                println!("");
            }
            print!("{}", self.board[i]);
        }
    }

}