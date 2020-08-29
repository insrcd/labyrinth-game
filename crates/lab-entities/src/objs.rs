use bevy::prelude::*;
use defaults::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Defaults)]
#[def = "Misc"]
pub enum ItemType {
    Weapon,
    Potion,
    Brew,
    Armor,
    Ingredient,
    Key,
    Misc,
    Undefined
}
#[derive(Clone, Debug, PartialEq, Defaults)]
#[def = "None"]
pub enum ItemSlot {
    LeftHand,
    RightHand,
    Head,
    Body,
    Legs,
    Magic,
    None
}

pub struct ItemLibrary {
    items : HashMap<String, ItemDefinition>
}

#[derive(Clone, Debug, Properties, PartialEq, Default)]
pub struct ItemDefinition {
    pub id: u64,
    pub name: String,
    pub weight: Weight,
    #[property(ignore)]
    pub item_type: ItemType,
    #[property(ignore)]
    pub item_slot: ItemSlot
}


#[derive(Clone,Copy,Default,Debug)]
pub struct ItemHandle {
    pub item_id : u64
}

#[derive(Copy, Clone, Debug, Properties, PartialEq, Default)]
pub struct Weight (pub f32);
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
    fn mix_with(&self, item : &mut ItemHandle);
}

struct Herb;
struct Grain;
struct Extract;
struct Fluid;

impl Mixable for Herb {
    fn mix_with(&self, _item : &mut ItemHandle) {
        todo!()
    }
}
impl Mixable for Grain {    
    fn mix_with(&self, _item : &mut ItemHandle) {
        todo!()
    } 
}
impl Mixable for Extract { 
    fn mix_with(&self, _item : &mut ItemHandle) {
        todo!()
    }
}

impl Mixable for Fluid { 
    fn mix_with(&self, _item : &mut ItemHandle) {
        todo!()
    }
}

