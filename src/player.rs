use crate::dropoff::Dropoff;
use crate::ship::Ship;
use crate::shipyard::Shipyard;

pub struct Player<'p> {
    pub id: u32,
    pub shipyard: &'p Shipyard,
    pub ships: Vec<&'p Ship>,
    pub dropoffs: Vec<&'p Dropoff>,
    pub halite: u32,
}

impl<'p> Player<'p> {
    pub fn new<'n>(id: u32, shipyard: &'n Shipyard) -> Player<'n> {
        Player {
            id,
            shipyard,
            ships: vec![],
            dropoffs: vec![],
            halite: 0,
        }
    }

    pub fn update_ships(&mut self, new_ships: Vec<&'p Ship>) {
        self.ships = new_ships;
    }

    pub fn update_dropoffs(&mut self, new_dropoffs: Vec<&'p Dropoff>) {
        self.dropoffs = new_dropoffs;
    }

    pub fn update_halite(&mut self, halite: u32) {
        self.halite = halite;
    }
}
