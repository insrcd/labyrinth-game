



use bevy::{ prelude::* , prelude::Properties};



use crate::world::Location;

struct Position(u32, u32);

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

impl Player {
    pub fn add_to_world(mut commands: Commands, name: &str) {
        commands.spawn((Player { god_mode: false }, crate::Named(name.to_string()), Job::BeerWizard));
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Stationary,
    Up,
    Down,
    Left,
    Right
}

pub enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided
}