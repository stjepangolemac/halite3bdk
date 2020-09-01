use crate::command::Command;
use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Shipyard {
    pub owner_id: u32,
    pub position: Position,
}

impl Shipyard {
    pub fn new(owner_id: u32, position: Position) -> Self {
        Shipyard { owner_id, position }
    }

    pub fn spawn(&self) -> Command {
        Command::Spawn
    }
}
