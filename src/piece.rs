
use crate::direction::Direction;

/// Represents a piece on the board
#[derive(Debug, Clone)]
pub struct Piece {
    /// Width of the piece
    pub width : usize, 

    /// height of the piece
    pub height : usize,
    
    /// X position
    pub x_pos : usize, 

    /// Y position
    pub y_pos : usize,
    
    /// Character used to display the piece in the console
    pub marker : char 
}

impl Piece {
    pub fn new(x_pos : usize, y_pos : usize, width : usize, height : usize, marker : char) -> Piece {
        Piece { width, height, x_pos, y_pos, marker }
    }

    /// Move the piece X and Y according the the given directions
    /// 
    /// # Panics
    /// Panics if the piece position becomes negative
    fn move_piece_coordinates(&mut self, x_delta : i32, y_delta : i32) {
        let new_x = self.x_pos as i32 + x_delta;
        let new_y = self.y_pos as i32 + y_delta;
        assert!(new_x >= 0, "Piece movement set it outside of board");
        assert!(new_y >= 0, "Piece movement set it outside of board");
        self.x_pos = new_x as usize;
        self.y_pos = new_y as usize;
    }

    /// Moves the piece in the specified direction
    pub fn move_piece_direction(&mut self, direction : Direction) {
        let (x  , y  )  = match direction {
            Direction::Down => (0i32, 1i32),
            Direction::Left => (-1i32, 0i32),
            Direction::Up => (0i32, -1i32),
            Direction::Right=> (1i32, 0i32),
        };
        self.move_piece_coordinates(x, y);
    }

    /// Get the X position of the piece
    pub fn get_x_pos(&self) -> usize {
        self.x_pos
    }
    
    /// Get the Y position of the piece
    pub fn get_y_pos(&self) -> usize {
        self.y_pos
    }
}

