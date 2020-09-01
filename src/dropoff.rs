use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Dropoff {
    pub id: u32,
    pub owner_id: u32,
    pub position: Position,
}

impl Dropoff {
    pub fn new(id: u32, owner_id: u32, position: Position) -> Self {
        Dropoff {
            id,
            owner_id,
            position,
        }
    }
}
