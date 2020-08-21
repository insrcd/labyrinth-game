use bevy::prelude::Bundle;
use crate::world::{Visible, Location, Interaction, InteractionResult};

#[allow(dead_code)]
pub enum ItemType {
    Weapon,
    Potion,
    Brew,
    Armor,
    Ingredient
}

#[allow(dead_code)]
pub struct Item;

#[derive(Copy, Clone, Debug)]
pub struct Interactable;
#[derive(Bundle, Copy, Clone, Debug)]
pub struct InteractableComponents {
    pub interactable: Interactable,
    pub location: Location,
    pub visible: Visible,
    pub interaction: Interaction
}

impl InteractableComponents {
   
}

impl Default for InteractableComponents {
    fn default() -> Self {
        InteractableComponents {
            location: Location::default(),
            visible: Visible,
            interaction: Interaction { call: |_attributes| { InteractionResult::None } },
            interactable: Interactable
        }
    }
}
#[allow(dead_code)]
pub struct Weight (f32);
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
    fn mix_with(&self, item : &mut Item);
}

struct Herb;
struct Grain;
struct Extract;
struct Fluid;

impl Mixable for Herb {
    fn mix_with(&self, _item : &mut Item) {
        todo!()
    }
}
impl Mixable for Grain {    
    fn mix_with(&self, _item : &mut Item) {
        todo!()
    } 
}
impl Mixable for Extract { 
    fn mix_with(&self, _item : &mut Item) {
        todo!()
    }
}

impl Mixable for Fluid { 
    fn mix_with(&self, _item : &mut Item) {
        todo!()
    }
}

