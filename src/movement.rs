use crate::direction::Direction;

#[derive(Debug)]
pub struct Movement {
    pub piece_id : usize,
    pub direction : Direction
}