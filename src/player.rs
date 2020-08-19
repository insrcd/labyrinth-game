

use bevy::{ prelude::* , prelude::Properties};


use crate::objs::*;
use crate::world::Location;

#[derive(PartialEq, Debug)]
pub struct Moving(pub Location, pub Location, pub Direction);

impl Default for Moving {
    fn default() -> Moving {
        Moving(Location::default(), Location::default(), Direction::Stationary)
    }
}

#[derive(Clone, Copy, Debug, Properties)]
pub struct Player { 
    pub god_mode : bool
}

impl Default for Player {
    fn default() -> Player {
        Player {
            god_mode: false
        }
    }
}

#[derive(Debug, Bundle)]
struct PlayerComponents {
    player : Player,
    job : Job,
    inventory : Inventory
}

#[derive(Debug)]
struct Inventory {
    items: Vec<Handle<Item>>
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Stationary,
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided
}


#[allow(dead_code)]
pub struct Power <'a> {
    name: String,
    cost: u32,
    effect: fn(crate::world::Solid) -> &'a crate::world::InteractionResult
}


#[allow(dead_code)]
pub struct Damage (f32);

#[derive(Copy, Clone)]
pub struct NonPlayer;