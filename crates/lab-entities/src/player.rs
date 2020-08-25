

use bevy::{ prelude::* };
use crate::objs::Item;
use crate::{Named, world::Location, prelude::Visible};


use rand::distributions::{Standard, Distribution};
use rand::Rng;
use std::{time::Duration, marker::PhantomData};
use lab_core::{Zoomable, InputTimer};

#[derive(PartialEq, Debug)]
pub struct Movement(pub Location, pub Location, pub Direction);

impl Default for Movement {
    fn default() -> Movement {
        Movement(Location::default(), Location::default(), Direction::Stationary)
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
pub struct PlayerComponents {
    player : Player,
    job : Job,
    inventory : Inventory,
    stats : Stats,
    abilities : Abilities,
    skills : Skills,
    named : Named,
    visible: Visible,
    location: Location,
    input_timer: InputTimer,
    movement: Movement,
    zoomable: Zoomable
}

impl PlayerComponents {
    pub fn new(name : &'static str, loc : Location) -> PlayerComponents{
        PlayerComponents {
            named: Named(String::from(name)),       
            location: loc,     
            ..Default::default()
        }
    }
}

impl Default for PlayerComponents {
    fn default() -> Self {
        PlayerComponents {
            player : Player { god_mode: false },
            job : Job::Undecided,
            inventory : Inventory::new(),
            stats : Stats::new(),
            abilities : Abilities::new(),
            skills : Skills::new(),
            named: Named("Unnamed".to_string()),
            visible: Visible,
            location: Location::default(),
            input_timer: InputTimer (Timer::new(Duration::from_millis(110), false)),
            movement: Movement::default(),
            zoomable: Zoomable
        }
    }
    
}

#[derive(Debug, Default, Clone)]
pub struct Inventory {
    pub items: Vec<Item>
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::<Item>::new()
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

#[derive(Copy, Clone)]
pub struct NonPlayer;