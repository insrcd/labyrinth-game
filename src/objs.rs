
use std::collections::HashMap;

pub enum ItemType {
    Weapon,
    Potion,
    Brew,
    Armor,
    Ingredient
}

pub struct Item {
    id: u32,
    item_type: ItemType,
    weight: f64,
    attributes: HashMap<String, String>
}

pub struct Weight {

}
pub enum WeaponSpecialPowers {
    Keen(u32),
    PlusDamage(u32),
    Cursed(String),
}

pub struct Weapon {
    attack_power : u32,
    special_powers: Vec<WeaponSpecialPowers>
}

struct Brew {
    name: String,
    ingredients: Vec<Box<dyn Mixable>>
}

trait Mixable {
    fn mix_with(&self, item : &mut Item);
}

struct Herb { 
    
}
struct Grain { 

}
struct Extract { }
struct Fluid { }

impl Mixable for Herb {
    fn mix_with(&self, item : &mut Item) {
        todo!()
    }
}
impl Mixable for Grain {    
    fn mix_with(&self, item : &mut Item) {
        todo!()
    } 
}
impl Mixable for Extract { 
    fn mix_with(&self, item : &mut Item) {
        todo!()
    }
}

impl Mixable for Fluid { 
    fn mix_with(&self, item : &mut Item) {
        todo!()
    }
}

