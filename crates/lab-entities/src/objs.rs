use lab_core::{WorldHandle, Item};
use bevy::prelude::*;
use defaults::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub enum WeaponSpecialPowers {
    Keen(u32),
    PlusDamage(u32),
    Cursed(String),
}

#[allow(dead_code)]
pub struct Weapon {
    attack_power : u32,
    special_powers: Vec<WeaponSpecialPowers>
}

#[allow(dead_code)]
struct Brew {
    name: String,
    ingredients: Vec<Box<dyn Mixable>>
}

trait Mixable {
    fn mix_with(&self, item : &mut WorldHandle<Item>);
}

struct Herb;
struct Grain;
struct Extract;
struct Fluid;

impl Mixable for Herb {
    fn mix_with(&self, _item : &mut WorldHandle<Item>) {
        todo!()
    }
}
impl Mixable for Grain {    
    fn mix_with(&self, _item : &mut WorldHandle<Item>) {
        todo!()
    } 
}
impl Mixable for Extract { 
    fn mix_with(&self, _item : &mut WorldHandle<Item>) {
        todo!()
    }
}

impl Mixable for Fluid { 
    fn mix_with(&self, _item : &mut WorldHandle<Item>) {
        todo!()
    }
}

