use std::collections::hashmap::HashMap;

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
    attributes: Box<HashMap<String, String>>
}

impl Map {
    fn load_map_from_file(filename : &str) -> Map {

    }
}
 
enum WeaponSpecialPowers {
    Keen(u32),
    PlusDamage(u32),
    Cursed(&str),
}

struct Weapon {
    attack_power : u32,
    special_powers: &[WeaponSpecialPowers]
}

struct Brew {
    name: &str,
    ingredients: &[dyn Mixable]
}

trait Mixable {
    fn mix_with(item : &mut Item);
}

pub struct ObjectMaker;

impl ObjectMaker {
    pub fn make_tile(&commands: Commands, texture_name: &str){
        
    }
}
