use std::collections::HashMap;

use crate::board::Board;

struct State {
    pub parent : String,
    pub board : Board,
    pub depth : u32
}

pub struct Solver {
    States : HashMap<String, State>
}

impl Solver {

    pub fn new(initial_board : Board) -> Solver {
        let mut s = Solver { States: HashMap::new() };
        s.States.insert(initial_board.get_2d_board().get_board_identifier(), State 
        { 
            parent: "".to_string(), 
            board: initial_board, 
            depth: 0 });
        s
    }


    
}