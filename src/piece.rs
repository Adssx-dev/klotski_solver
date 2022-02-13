
//Represents a piece on the board
#[derive(Debug)]
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
}

