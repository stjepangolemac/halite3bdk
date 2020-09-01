use crate::command::Command;
use crate::communication::Communication;
use crate::configuration::Configuration;
use crate::dropoff::Dropoff;
use crate::logger;
use crate::map::{Map, Structure};
use crate::position::Position;
use crate::ship::Ship;
use crate::shipyard::Shipyard;
use std::collections::HashMap;

pub struct Game {
    communication: Communication,
    configuration: Configuration,
    pub turn: u32,
    num_players: u32,
    my_id: u32,
    pub map: Map,
    shipyards: Vec<Shipyard>,
    ships: Vec<Ship>,
    dropoffs: Vec<Dropoff>,
    player_halite: HashMap<u32, u32>,
    commands: Vec<Command>,
}

impl Game {
    pub fn init(bot_name: &str) -> Self {
        logger::init();

        let mut communication = Communication::new();

        let configuration = communication.get_configuration();
        let player_data = communication.get_players();
        let map_data = communication.get_map();

        let mut map = Map::new(map_data);
        let shipyards: Vec<Shipyard> = player_data
            .shipyards
            .iter()
            .map(|(player_id, shipyard_position)| {
                Shipyard::new(*player_id, shipyard_position.clone())
            })
            .collect();

        shipyards.iter().for_each(|shipyard| {
            map.set_structure(&shipyard.position, Structure::Shipyard(shipyard.owner_id))
        });

        communication.send_name(bot_name);

        Game {
            communication,
            configuration,
            turn: 0,
            num_players: player_data.count,
            my_id: player_data.my_id,
            map,
            shipyards,
            ships: vec![],
            dropoffs: vec![],
            player_halite: HashMap::new(),
            commands: vec![],
        }
    }

    pub fn next_turn(&mut self) {
        let turn_data = self.communication.get_turn(self.num_players);

        self.turn = turn_data.turn;

        let mut ships = vec![];
        let mut dropoffs = vec![];

        turn_data
            .player_updates
            .iter()
            .for_each(|(player_id, player_update)| {
                player_update
                    .ships
                    .iter()
                    .for_each(|(ship_id, (position, ship_halite))| {
                        ships.push(Ship::new(
                            *ship_id,
                            *player_id,
                            position.clone(),
                            *ship_halite,
                        ));
                    });

                player_update
                    .dropoffs
                    .iter()
                    .for_each(|(dropoff_id, position)| {
                        dropoffs.push(Dropoff::new(*dropoff_id, *player_id, position.clone()));
                    });

                self.player_halite.insert(*player_id, player_update.halite);
            });

        self.ships = ships;
        self.dropoffs = dropoffs;

        self.map.clear();

        let mut structure_positions: Vec<(&Position, Structure)> = vec![];
        self.shipyards.iter().for_each(|shipyard| {
            structure_positions.push((&shipyard.position, Structure::Shipyard(shipyard.owner_id)))
        });
        self.dropoffs.iter().for_each(|dropoff| {
            structure_positions.push((&dropoff.position, Structure::Dropoff(dropoff.id)))
        });

        let mut occupied_positions = vec![];
        self.ships
            .iter()
            .for_each(|ship| occupied_positions.push(&ship.position));

        for sp in structure_positions {
            self.map.set_structure(sp.0, sp.1);
        }
        for op in occupied_positions {
            self.map.set_occupied(op);
        }

        turn_data
            .map_updates
            .iter()
            .for_each(|map_update| self.map.set_halite(&map_update.0, map_update.1));
    }

    pub fn player_ships(&self, player_id: u32) -> Vec<Ship> {
        self.ships
            .iter()
            .filter(|ship| ship.owner_id == player_id)
            .map(|ship| ship.clone())
            .collect()
    }

    pub fn player_dropoffs(&self, player_id: u32) -> Vec<Dropoff> {
        self.dropoffs
            .iter()
            .filter(|dropoff| dropoff.owner_id == player_id)
            .map(|dropoff| dropoff.clone())
            .collect()
    }

    pub fn player_shipyard(&self, player_id: u32) -> Option<Shipyard> {
        self.shipyards
            .iter()
            .find(|shipyard| shipyard.owner_id == player_id)
            .and_then(|shipyard| Some(shipyard.clone()))
    }

    pub fn my_halite(&self) -> u32 {
        *self.player_halite.get(&self.my_id).unwrap()
    }

    pub fn my_ships(&self) -> Vec<Ship> {
        self.player_ships(self.my_id)
    }

    pub fn my_dropoffs(&self) -> Vec<Dropoff> {
        self.player_dropoffs(self.my_id)
    }

    pub fn my_shipyard(&self) -> Shipyard {
        self.player_shipyard(self.my_id).unwrap()
    }

    pub fn register_commands(&mut self, cmds: Vec<Command>) {
        self.commands = cmds;
    }

    pub fn end_turn(&mut self) {
        self.communication.send_commands(&self.commands);
        self.commands = vec![];
    }
}
