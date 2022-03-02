use std::collections::HashMap;
use std::string;
use std::thread::current;

use crate::board::Board;
use crate::movement::Movement;

/// Represents the state of the board at an instant
struct State {
    /// String used as an ID to get the parent of this state to reconstruct the path to solution when it is found
    pub parent : String,

    ///Complete board (pieces, etc.) at the current state
    pub board : Board,
}

pub struct Solver {
    /// HashMap used as a tree for states.
    /// 
    /// Each node has a string as key, and the value contains the key to the parent, 
    /// thus is can be seen as a tree
    states : HashMap<String, State>, 

    /// Possible values for next solving iteration
    candidates : Vec<String>, 

    /// If the (possibily unoptimised) solution is found, it is stored here to stop searching for worse solutions
    solution_state : Option<String>
}

impl Solver {

    pub fn new(initial_board : Board) -> Solver {
        let mut s = Solver { 
            states: HashMap::new(),
            candidates : Vec::new(),
            solution_state : None
        };
        let initial_board_str = initial_board.get_2d_board().get_board_identifier();
        s.states.insert(initial_board_str.clone(), State { 
            parent: "".to_string(), 
            board: initial_board, 
        });
        s.candidates.push(initial_board_str.clone());
        s
    }

    /// Solves the board given in the constructor
    pub fn solve(&mut self) -> Vec<Board> {

        while self.solution_state.is_none() {
            self.clear_bad_candidates();
            self.check_next_candidate();
        }
        let mut solution : Vec<Board> = Vec::new();

        let mut t = self.solution_state.clone();

        while let Some(state) = t {
            let node = self.states.get(&state).unwrap();
            solution.push(node.board.clone());
            t = match node.parent.clone().as_str() {
                "" => None,
                _ => Some(node.parent.clone())
            };
            
        }
        solution
    }

    /// Generates all possible moves for next candidate, generate the associated children
    /// and adds them to the candidate list
    fn check_next_candidate(&mut self) {
        let candidate = self.get_best_candidate();

        let state = self.states.get(&candidate).unwrap();
        let state_depth = self.get_state_depth(&candidate);
        let movements = state.board.get_all_possible_movements();

        // First clone here because if in the for loop, the compiler shouts at me
        let original_board_clone = state.board.clone();

        for mov in movements {
            let mut new_board = original_board_clone.clone();
            new_board.move_piece(mov);

            let new_board_key = new_board.get_2d_board().get_board_identifier();
            if new_board.is_solved() {
                self.solution_state = Some(new_board_key.clone());
            }

            let new_state_depth = self.get_state_depth(&new_board_key);
            let new_board_state = self.states.get_mut(&new_board_key);
            
            match new_board_state {
                None => {
                    self.states.insert(new_board_key.clone(), State { parent: candidate.clone(), board: new_board });
                    self.candidates.push(new_board_key);
                }
                Some(s) => {
                    if new_state_depth.unwrap() > state_depth.unwrap() + 1 {
                        s.parent = candidate.clone();
                    }
                }
            }
        }
    }

    /// Removes the candidates for which the depth is greater than the knows solution (if there is one)
    /// 
    /// These candidates are not worth investigating as they would generate only worse solutions
    fn clear_bad_candidates(&mut self) {
        if let Some(solution) = &self.solution_state {
            // depth of the known solution, no need to investigate candidates with a worse depth
            let max_depth = self.get_state_depth(solution); 
            
            let mut i : usize= 0;
            while i < self.candidates.len() {
                if self.get_state_depth(&self.candidates[i]) >= max_depth {
                    self.candidates.remove(i);
                }
                else {
                    i += 1;
                }
            };
            
        }
    }

    /// Gets the best candidate for next iteration
    /// 
    /// The best candidate is the one with the greatest value for its heuristic
    fn get_best_candidate(&mut self) -> String {
        let list : Vec<u32> = self.candidates.iter().map(|c | self.states.get(c).unwrap().board.solved_heuristic()).collect();
        let max = list.iter().max().unwrap();
        let index = list.iter().position(|element| element == max).unwrap();
        self.candidates.remove(index)
    }

    /// Calculates the depth of the state given by the ID passed as argument
    /// 
    /// This function goes through each parent, parent of parent, etc.
    /// It takes more time the deeper the element in the tree
    fn get_state_depth (&self, state_key : &str) -> Option<u32> {
        let mut depth = 0;
        let current_state = self.get_parent_key(state_key);
        match current_state {
            None => None,
            Some(state) => {
                let mut s = state;
                while s != "" {
                    s = self.get_parent_key(s).unwrap();
                    depth += 1;
                };
                Some(depth)
            }
        }
    }

    /// Get the key to the parent of the element associated with the given key
    fn get_parent_key(&self, state_key : &str) -> Option<&str> {
        let key = self.states.get(state_key);
        match key {
            None => None,
            Some(v) => Some(v.parent.as_str())
        }        
    }
}