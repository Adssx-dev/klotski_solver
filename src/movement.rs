use crate::direction::Direction;

/// Describes a possible movement : the index of a piece and the direction
#[derive(Debug)]
pub struct Movement {
    pub piece_id : usize,
    pub direction : Direction
}