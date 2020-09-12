

use bevy::{ prelude::* };
use lab_core::prelude::*;
use std::time::Duration;


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
pub struct PlayerComponents {
    player : Player,
    job : Job,
    inventory : Inventory,
    stats : Stats,
    abilities : Abilities,
    skills : Skills,
    named : Named,
    location: Location,
    movement: Movement,
    zoomable: Zoomable,
    input_timer: InputTimer
}

impl PlayerComponents {
    pub fn new(name : &'static str) -> PlayerComponents{
        PlayerComponents {
            named: Named(String::from(name)),
            ..Default::default()
        }
    }
}

impl Default for PlayerComponents {
    fn default() -> Self {
        PlayerComponents {
            player : Player { god_mode: false },
            job : Job::Undecided,
            inventory : Inventory(Vec::new()),
            stats : Stats::new(),
            abilities : Abilities::new(),
            skills : Skills::new(),
            named: Named("Unnamed".to_string()),            
            location: Location::default(),
            input_timer: InputTimer(Timer::new(Duration::from_millis(100), false)),
            movement: Movement::default(),
            zoomable: Zoomable
        }
    }
    
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct Stats {
    strength: u32,
    dextarity: u32,
    wit: u32,
    creativity: u32,
    wisdom: u32,
    charisma: u32
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct Abilities {
    magic_power: u32,
    brewing_power: u32
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct Skills {
    brewing: u32,
    stealth: u32,
    melee: u32,
    throwing: u32,
    diplomacy: u32
}

impl Abilities {
    pub fn new() -> Abilities {
        Abilities { ..Default::default() }
    }
}

impl Skills {
    pub fn new() -> Skills {
        Skills { ..Default::default() }
    }
}

impl Stats {
    pub fn new() -> Stats {
        Stats { ..Default::default() }
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided,
    Custom(Stats, Abilities)
}


#[allow(dead_code)]
pub struct Power <'a, T> {
    name: String,
    cost: u32,
    effect: fn() -> &'a T
}


#[allow(dead_code)]
pub struct Damage (f32);

#[derive(Copy, Clone, Default, Debug)]
pub struct NonPlayer;