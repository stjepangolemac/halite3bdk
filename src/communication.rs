use crate::command::Command;
use crate::configuration::Configuration;
use crate::input::Input;
use crate::logger;
use crate::output::Output;
use crate::position::Position;
use serde_json;
use std::collections::HashMap;

pub struct PlayerData {
    pub count: u32,
    pub my_id: u32,
    pub shipyards: HashMap<u32, Position>,
}

pub struct MapData {
    pub width: u32,
    pub height: u32,
    pub halite: Vec<Vec<u32>>,
}

pub struct PlayerUpdate {
    pub num_ships: u32,
    pub num_dropoffs: u32,
    pub ships: HashMap<u32, (Position, u32)>,
    pub dropoffs: HashMap<u32, Position>,
    pub halite: u32,
}

pub struct TurnData {
    pub turn: u32,
    pub player_updates: HashMap<u32, PlayerUpdate>,
    pub map_updates: Vec<(Position, u32)>,
}

#[derive(Debug, PartialEq)]
pub enum CommunicationState {
    Initial,
    Configuration,
    Players,
    Map,
    Turn,
}

pub struct Communication {
    state: CommunicationState,
    input: Input,
    output: Output,
}

impl Communication {
    pub fn new() -> Self {
        Communication {
            state: CommunicationState::Initial,
            input: Input::new(),
            output: Output::new(),
        }
    }

    fn bad_state(&self, msg: &str) -> ! {
        logger::abort(&format!(
            "invalid action, current state is {:?}: {}",
            self.state, msg
        ));
    }

    pub fn get_configuration(&mut self) -> Configuration {
        if self.state != CommunicationState::Initial {
            self.bad_state("cannot get configuration");
        }
        self.state = CommunicationState::Configuration;

        let json = self.input.read_line();

        match serde_json::from_str(&json) {
            Ok(configuration) => configuration,
            Err(msg) => logger::abort(&format!("could not read configuration: {}", msg)),
        }
    }

    pub fn get_players(&mut self) -> PlayerData {
        if self.state != CommunicationState::Configuration {
            self.bad_state("cannot get players");
        }
        self.state = CommunicationState::Players;

        self.input.parse_line();
        let num_players = self.input.next_u32();
        let my_id = self.input.next_u32();

        let mut shipyards = HashMap::new();
        for _ in 0..num_players {
            self.input.parse_line();
            let owner_id = self.input.next_u32();
            let shipyard_x = self.input.next_u32();
            let shipyard_y = self.input.next_u32();

            shipyards.insert(
                owner_id,
                Position::new(shipyard_x as i32, shipyard_y as i32),
            );
        }

        PlayerData {
            count: num_players,
            my_id,
            shipyards,
        }
    }

    pub fn get_map(&mut self) -> MapData {
        if self.state != CommunicationState::Players {
            self.bad_state("cannot get map");
        }
        self.state = CommunicationState::Map;

        self.input.parse_line();
        let width = self.input.next_u32();
        let height = self.input.next_u32();

        let mut halite = vec![];
        for _ in 0..height {
            self.input.parse_line();

            let mut row = vec![];
            for _ in 0..width {
                row.push(self.input.next_u32());
            }

            halite.push(row);
        }

        MapData {
            width,
            height,
            halite,
        }
    }

    pub fn get_turn(&mut self, num_players: u32) -> TurnData {
        if !(self.state == CommunicationState::Map || self.state == CommunicationState::Turn) {
            self.bad_state("cannot get turn");
        }
        self.state = CommunicationState::Turn;

        self.input.parse_line();
        let turn_num = self.input.next_u32();

        let mut players = HashMap::new();
        for _ in 0..num_players {
            self.input.parse_line();
            let player_id = self.input.next_u32();
            let num_ships = self.input.next_u32();
            let num_dropoffs = self.input.next_u32();
            let halite = self.input.next_u32();

            let mut ships = HashMap::new();
            for _ in 0..num_ships {
                self.input.parse_line();
                let ship_id = self.input.next_u32();
                let ship_x = self.input.next_u32();
                let ship_y = self.input.next_u32();
                let ship_halite = self.input.next_u32();

                ships.insert(
                    ship_id,
                    (Position::new(ship_x as i32, ship_y as i32), ship_halite),
                );
            }

            let mut dropoffs = HashMap::new();
            for _ in 0..num_dropoffs {
                self.input.parse_line();
                let dropoff_id = self.input.next_u32();
                let dropoff_x = self.input.next_u32();
                let dropoff_y = self.input.next_u32();

                dropoffs.insert(
                    dropoff_id,
                    Position::new(dropoff_x as i32, dropoff_y as i32),
                );
            }

            players.insert(
                player_id,
                PlayerUpdate {
                    num_ships,
                    num_dropoffs,
                    halite,
                    ships,
                    dropoffs,
                },
            );
        }

        self.input.parse_line();
        let map_updates_count = self.input.next_u32();

        let mut map_updates = vec![];
        for _ in 0..map_updates_count {
            self.input.parse_line();
            let x = self.input.next_u32();
            let y = self.input.next_u32();
            let halite = self.input.next_u32();

            map_updates.push((Position::new(x as i32, y as i32), halite));
        }

        TurnData {
            turn: turn_num,
            player_updates: players,
            map_updates,
        }
    }

    pub fn send_name(&self, name: &str) {
        self.output.send(name);
    }

    pub fn send_commands(&self, cmds: &[Command]) {
        cmds.iter().for_each(|command| print!("{} ", command));
        println!();
    }
}
