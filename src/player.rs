
use bevy::prelude::*;
use std::fmt::*;

struct Position(u32, u32);

pub struct Named(pub String);

impl Named {
    fn new(name : &str) -> Named {
        Named(name.to_string())
    }
}

pub struct Player;


impl Player {
    pub fn add_to_world(mut commands: Commands, name: &str) {
        commands.spawn((Player, Named(name.to_string()), Job::BeerWizard));
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "A player!")
    }
}

enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided
}