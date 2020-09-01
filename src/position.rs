use crate::direction::Direction;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn with_direction(&self, d: &Direction) -> Position {
        let (x, y) = match d {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::Still => (0, 0),
        };

        Position::new(self.x + x, self.y + y)
    }

    pub fn surrounding(&self) -> Vec<Position> {
        vec![
            self.with_direction(&Direction::North),
            self.with_direction(&Direction::East),
            self.with_direction(&Direction::South),
            self.with_direction(&Direction::West),
        ]
    }

    pub fn distance_to(&self, p: &Position) -> i32 {
        let dx = (self.x - p.x).abs();
        let dy = (self.y - p.y).abs();

        dx + dy
    }
}
