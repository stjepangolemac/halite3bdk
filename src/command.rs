use crate::direction::Direction;
use std::fmt::Display;

pub enum Command {
    Spawn,
    Move(u32, Direction),
    TransformDropoff(u32),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Spawn => write!(f, "g"),
            Command::Move(ship_id, direction) => {
                let direction_char = match direction {
                    Direction::North => "n",
                    Direction::East => "e",
                    Direction::South => "s",
                    Direction::West => "w",
                    Direction::Still => "o",
                };

                write!(f, "m {} {}", ship_id, direction_char)
            }
            Command::TransformDropoff(ship_id) => write!(f, "c {}", ship_id),
        }
    }
}
