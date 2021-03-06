use crate::{piece::Piece, direction::Direction, movement::Movement};

const EMPTY_CELL_CHAR : char = '.';


/// Used to construct the "visual" board in 2D based on pieces positions and sizes
#[derive(Debug, Clone)]
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
            board : vec!(EMPTY_CELL_CHAR; width * height)
        };
        result.fill_board(pieces);
        result
    }

    /// Fills the board according to the position of pieces
    /// 
    /// # Panics
    /// Panics if one piece goes outside the board
    fn fill_board(&mut self, pieces : &Vec<Piece>) {
        for piece in pieces {
            assert!(piece.x_pos + piece.width - 1 < self.width, "Piece getting over the edge horizontally. Piece : {:?}", piece);
            assert!(piece.y_pos + piece.height - 1< self.height, "Piece getting over the edge vertically. Piece : {:?}", piece);

            for i in piece.x_pos..(piece.x_pos + piece.width) {
                for j in piece.y_pos..(piece.y_pos + piece.height) {
                    assert!(self.board[j * self.width + i] == EMPTY_CELL_CHAR, "Two pieces are at the same place. Piece trying to be placed : {:?}, marker already at this place : {}", piece, self.board[j * self.width + i]);
                    self.set_cell(i, j, piece.marker);
                }
            }
        }
    }

    /// Displays the board in the console
    pub fn print(&self) {
        for i in 0..self.board.len() {
            if i % self.width == 0 {
                println!("");
            }
            print!("{}", self.board[i]);
        }
        println!("");
    }

    /// Calculates all legal movements for all pieces and returns the list of possibilities
    pub fn get_all_possible_movements(&self, pieces : &Vec<Piece>) -> Vec<Movement> {
        let mut result : Vec<Movement>= Vec::new();

        for i in 0..pieces.len() {
            let possible_movements = self.get_piece_possible_movements(&pieces[i]);
            for movement in possible_movements {
                result.push(Movement { 
                    piece_id : i,
                    direction : movement
                });
            }
        }

        result
    }

    /// Calculate all possible movements for one piece
    fn get_piece_possible_movements(&self, piece : &Piece) -> Vec<Direction> {
        let mut movements : Vec<Direction> = Vec::new();
        if self.can_piece_move_left(piece) {
            movements.push(Direction::Left);
        }
        if self.can_piece_move_right(piece) {
            movements.push(Direction::Right);
        }
        if self.can_piece_move_up(piece) {
            movements.push(Direction::Up);
        }
        if self.can_piece_move_down(piece) {
            movements.push(Direction::Down);
        }
        movements
    }

    /// Determines if the given piece can move to its left
    fn can_piece_move_left(&self, piece : &Piece) -> bool {
        if piece.x_pos == 0 {
            false
        }
        else {
            self.check_empty_cells(piece.x_pos - 1, piece.x_pos - 1, piece.y_pos, piece.y_pos + piece.height - 1)
        }
    }

    /// Determines if the given piece can move to its right
    fn can_piece_move_right(&self, piece : &Piece) -> bool {
        if piece.x_pos + piece.width >= self.width {
            false
        }
        else {
            self.check_empty_cells(piece.x_pos + piece.width, piece.x_pos + piece.width, piece.y_pos, piece.y_pos + piece.height - 1)
        }
    }

    /// Determines if the given piece can move up
    fn can_piece_move_up(&self, piece : &Piece) -> bool {
        if piece.y_pos == 0 {
            false
        }
        else {
            self.check_empty_cells(piece.x_pos, piece.x_pos + piece.width - 1, piece.y_pos - 1, piece.y_pos - 1)
        }
    }

    /// Determines if the given piece can move down
    fn can_piece_move_down(&self, piece : &Piece) -> bool {
        if piece.y_pos + piece.height >= self.height {
            false
        }
        else {
            self.check_empty_cells(piece.x_pos, piece.x_pos + piece.width - 1, piece.y_pos + piece.height, piece.y_pos + piece.height)
        }
    }

    /// Determines if all the cells in the specified rectangle are empty
    fn check_empty_cells(&self, xstart : usize, xstop : usize, ystart : usize, ystop : usize) -> bool {
        let mut result : bool = true;

        for x in xstart..xstop + 1 {
            for y in ystart..ystop + 1 {
                result = result && self.get_cell(x, y) == EMPTY_CELL_CHAR;
            }
        }
        result
    }

    /// Get the content of the cell at specified coordinates
    fn get_cell(&self, x : usize, y: usize) -> char {
        self.board[y * self.width + x]
    }

    /// Set the content of the cell at the specified coordinates
    /// 
    /// # Panics
    /// Panic if the coordinates are outside the board
    fn set_cell(&mut self, x : usize, y: usize, value : char) {
        assert!(x < self.width, "Trying to set a cell above maximum width");
        assert!(y < self.height, "Trying to set a cell above maximum height");
        self.board[y * self.width + x] = value;
    }

    /// Get the string representing the bord uniquely
    pub fn get_board_identifier(&self) -> String {
        self.board.clone().into_iter().collect()
    }

}