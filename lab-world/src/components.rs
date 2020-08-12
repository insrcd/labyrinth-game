use std::collections::hashmap::HashMap;

pub enum ItemType {
    Weapon,
    Potion,
    Brew,
    Armor,
    Ingredient
}

struct Item {
    id: u32,
    type: ItemType,
    weight: f64,
    attributes: Box<HashMap<String, String>>
}

struct Location {
    floor: u32,
    x: u32,
    y: u32
}

struct Placeable {
    location: Location
}

struct Tile {

}

struct Collidable {
    
}

struct LocationMap {
    tiles: &[Tile]
    placeable: &[Placeable]
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

use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};