use crate::command::Command;
use crate::direction::Direction;
use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Ship {
    pub id: u32,
    pub owner_id: u32,
    pub position: Position,
    pub halite: u32,
}

impl Ship {
    pub fn new(id: u32, owner_id: u32, position: Position, halite: u32) -> Self {
        Ship {
            id,
            owner_id,
            position,
            halite,
        }
    }

    pub fn r#move(&self, direction: Direction) -> Command {
        Command::Move(self.id, direction)
    }
}
