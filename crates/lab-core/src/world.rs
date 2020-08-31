use bevy::prelude::*;
use std::{marker::PhantomData, collections::{btree_map::{Keys, Values}, BTreeMap, HashMap}, sync::{Arc, Mutex}, rc::Rc};
use defaults::*;
use crate::interaction::*;

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorldLocation {
    World,
    Inventory,
    Labyrinth,
    BarRoom
}
// Component to work with Translations to get "World Locations"
#[derive(Clone, Debug, Copy, PartialEq, Properties)]
pub struct Location (pub f32, pub f32, pub f32, 
    #[property(ignore)] pub WorldLocation);


impl Default for Location {
    fn default() -> Self {
        return Location(0.,0.,0.,WorldLocation::World)
    }
    
}
impl From<Location> for Vec3 {
    fn from(x: Location) -> Self {
        Vec3::new(x.0, x.1, x.2)
    }
    
}
impl From<Location> for Vec2 {
    fn from(x: Location) -> Self {
        Vec2::new(x.0, x.1)
    }    
}
#[derive(Default, Clone, Debug)]
pub struct InteractionCatalog <I, T : CatalogItem + Sync + Send + Clone, R: Sync + Send + Clone> 
where I : Interact<T, R> {
    item : I,
    data : PhantomData<R>,
    pub components: BTreeMap<String, T>,
    pub interactions: BTreeMap<String, Arc<I>>
}

impl <I, T : CatalogItem + Sync + Send + Clone, R : Sync + Send + Clone> InteractionCatalog<I, T, R>
 where I : Interact <T,R> {    /// if there's a tile named and an interaction for tha tile, return it, if not None
    pub fn get_interaction(&self, name: &String) -> Option<Arc<I>> {
        if let Some(interact)  = self.interactions.get(name) {
            Some(interact.clone())
        } else {
            None
        }
    }
    pub fn names(&self) -> Keys<'_, String, T>{
        self.components.keys()
    }

    pub fn iter(&self) -> Values<'_, String, T> {
        self.components.values()
    }

    pub fn categories(&self) -> Vec<String> {
        let mut categories : Vec<String> = self.components.values().map(|m| m.category().clone()).collect();
        
        categories.sort();
        categories.dedup();
        
        categories
    }

    pub fn items_in_category(&self, category : &str) -> Vec<&T> {
        self.components.values().filter(|p| p.category() == category).collect()        
    }

    pub fn update(&mut self, comp : T) {

        if let Some(tc) = self.components.get_mut(&comp.name()) {
           *tc = comp.clone();
        } else {
            self.components.insert(comp.name().clone(), comp.clone());
        }
    }
}

impl Location {
    pub fn normalize( window: &Window, 
            cam_transition: &Translation,  
            position : &Vec2) -> Vec2 {

        let camera_offset_x : f32 = cam_transition.x();
        let camera_offset_y : f32 = cam_transition.y() ;
    
        let x_window_offset = window.width;
        let y_window_offset = window.height;
        
        let normalized_x = position.x() + camera_offset_x - (x_window_offset/2) as f32;
        let normalized_y = position.y() + camera_offset_y - (y_window_offset/2) as f32;

        return Vec2::new(normalized_x, normalized_y);
    }
}

impl From<Translation> for Location {
    fn from(t : Translation) -> Self {
        Location (t.0.x(), t.0.y(), t.0.z(), WorldLocation::World)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Visible;

#[derive(Copy, Clone, Debug)]
pub struct Solid;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inventory {
    pub items: Vec<ItemHandle>
}


impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::<ItemHandle>::new()
        }
    }
    pub fn has(&self, predicate: fn (&ItemHandle) -> bool) -> bool {
        self.items.iter().any(|i| predicate(i))
    }
}


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

#[derive(Clone, Debug)]
pub struct ItemStorage {
    pub items : HashMap<u64, ItemDefinition>
}

impl ItemStorage {
    pub fn forge(&mut self, item : ItemDefinition) -> ItemHandle {
        let id : u64 = rand::thread_rng().gen();

        self.items.insert(id, item);
    
        ItemHandle { item_id: id }
    }

    pub fn get(&mut self, handle : ItemHandle) -> ItemDefinition {
        self.items
            .get(&handle.item_id)
            .expect("Item not found in storage, this shouldn't happen")
            .clone()
    }
}

#[derive(Clone,Copy,Default,Debug, PartialEq)]
pub struct ItemHandle {
    pub item_id : u64
}

#[derive(Copy, Clone, Debug, Properties, PartialEq, Default)]
pub struct Weight (pub f32);