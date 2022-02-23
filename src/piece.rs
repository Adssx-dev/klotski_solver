
use crate::direction::Direction;

//Represents a piece on the board
#[derive(Debug, Clone)]
pub struct Piece {
    pub width : usize, // Width of the piece
    pub height : usize, // height of the piece
    pub x_pos : usize, // X position
    pub y_pos : usize, // Y position
    pub marker : char // Character used to display the piece in the console
}

impl Piece {
    pub fn new(x_pos : usize, y_pos : usize, width : usize, height : usize, marker : char) -> Piece {
        Piece { width, height, x_pos, y_pos, marker }
    }

    fn move_piece_coordinates(&mut self, x_delta : i32, y_delta : i32) {
        let new_x = self.x_pos as i32 + x_delta;
        let new_y = self.y_pos as i32 + y_delta;
        assert!(new_x >= 0, "Piece movement set it outside of board");
        assert!(new_y >= 0, "Piece movement set it outside of board");
        self.x_pos = new_x as usize;
        self.y_pos = new_y as usize;
    }

    pub fn move_piece_direction(&mut self, direction : Direction) {
        let (x  , y  )  = match direction {
            Direction::Down => (0i32, 1i32),
            Direction::Left => (-1i32, 0i32),
            Direction::Up => (0i32, -1i32),
            Direction::Right=> (1i32, 0i32),
        };
        self.move_piece_coordinates(x, y);
    }
}

