use crate::communication::MapData;
use crate::direction::Direction;
use crate::position::Position;

pub enum Structure {
    Shipyard(u32),
    Dropoff(u32),
    Nothing,
}

pub struct Cell {
    pub position: Position,
    pub halite: u32,
    structure: Structure,
    occupied: bool,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(data: MapData) -> Self {
        let mut cells = vec![];
        for y in 0..data.height {
            let mut row = vec![];
            for x in 0..data.width {
                row.push(Cell {
                    position: Position::new(x as i32, y as i32),
                    structure: Structure::Nothing,
                    occupied: false,
                    halite: data.halite[y as usize][x as usize],
                });
            }

            cells.push(row);
        }

        Map {
            width: data.width,
            height: data.height,
            cells,
        }
    }

    pub fn get_cell(&self, position: &Position) -> &Cell {
        let position = self.normalize_position(position);

        &self.cells[position.y as usize][position.x as usize]
    }

    fn get_mut_cell(&mut self, position: &Position) -> &mut Cell {
        let position = self.normalize_position(position);

        &mut self.cells[position.y as usize][position.x as usize]
    }

    pub fn set_occupied(&mut self, position: &Position) {
        let position = self.normalize_position(position);

        self.get_mut_cell(&position).occupied = true;
    }

    pub fn set_structure(&mut self, position: &Position, structure: Structure) {
        let position = self.normalize_position(position);

        self.get_mut_cell(&position).structure = structure;
    }

    pub fn set_halite(&mut self, position: &Position, halite: u32) {
        let position = self.normalize_position(position);

        self.get_mut_cell(&position).halite = halite;
    }

    pub fn is_occupied(&self, position: &Position) -> bool {
        let position = self.normalize_position(position);

        self.get_cell(&position).occupied
    }

    pub fn has_structure(&self, position: &Position) -> bool {
        let position = self.normalize_position(position);

        match self.get_cell(&position).structure {
            Structure::Nothing => false,
            _ => true,
        }
    }

    pub fn normalize_position(&self, position: &Position) -> Position {
        let width = self.width as i32;
        let height = self.height as i32;

        let x = ((position.x % width) + width) % width;
        let y = ((position.y % height) + height) % height;

        Position { x, y }
    }

    pub fn unsafe_direction_to(&self, source: &Position, destination: &Position) -> Vec<Direction> {
        let normalized_source = self.normalize_position(source);
        let normalized_destination = self.normalize_position(destination);

        let dx = (normalized_source.x - normalized_destination.x).abs();
        let dy = (normalized_source.y - normalized_destination.y).abs();

        let wrapped_dx = self.width as i32 - dx;
        let wrapped_dy = self.height as i32 - dy;

        let mut possible_moves: Vec<Direction> = Vec::new();

        if normalized_source.x < normalized_destination.x {
            possible_moves.push(if dx > wrapped_dx {
                Direction::West
            } else {
                Direction::East
            });
        } else if normalized_source.x > normalized_destination.x {
            possible_moves.push(if dx < wrapped_dx {
                Direction::West
            } else {
                Direction::East
            });
        }

        if normalized_source.y < normalized_destination.y {
            possible_moves.push(if dy > wrapped_dy {
                Direction::North
            } else {
                Direction::South
            });
        } else if normalized_source.y > normalized_destination.y {
            possible_moves.push(if dy < wrapped_dy {
                Direction::North
            } else {
                Direction::South
            });
        }

        possible_moves
    }

    pub fn direction_to(&self, source: &Position, destination: &Position) -> Vec<Direction> {
        self.unsafe_direction_to(source, destination)
            .into_iter()
            .filter(|dir| !self.is_occupied(&source.with_direction(dir)))
            .collect()
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.occupied = false;
                cell.structure = Structure::Nothing;
            });
        });
    }
}
