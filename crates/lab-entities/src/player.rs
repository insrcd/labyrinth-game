

use bevy::{ prelude::* };
use crate::objs::Item;
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

use rand::distributions::{Standard, Distribution};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Stationary,
    Up,
    Down,
    Left,
    Right
}


impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 5) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::Stationary
        }
    }
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
pub struct Power <'a, T> {
    name: String,
    cost: u32,
    effect: fn() -> &'a T
}


#[allow(dead_code)]
pub struct Damage (f32);

#[derive(Copy, Clone)]
pub struct NonPlayer;