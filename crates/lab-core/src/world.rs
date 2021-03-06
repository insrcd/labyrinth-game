use crate::interaction::*;
use bevy::prelude::*;
use defaults::*;
use lab_data::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{
        hash_map::{Keys, Values},
        HashMap,
    },
    fmt::Debug,
    hash::Hash,
    hash::Hasher,
    marker::PhantomData,
    sync::Arc,
};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorldLocation {
    World,
    Inventory,
    Labyrinth,
    BarRoom,
}
// Component to work with Translations to get "World Locations"
#[derive(Clone, Debug, Copy, PartialEq, Properties)]
pub struct Location(
    pub f32,
    pub f32,
    pub f32,
    #[property(ignore)] pub WorldLocation,
);

impl Location {
    pub fn new(xform: Transform, loc: WorldLocation) -> Location {
        Location(
            xform.translation().x(),
            xform.translation().y(),
            xform.translation().z(),
            loc,
        )
    }
}

impl Default for Location {
    fn default() -> Self {
        return Location(0., 0., 0., WorldLocation::World);
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
pub struct InteractionCatalog<I, T: CatalogItem + Sync + Send + Clone, R: Sync + Send + Clone>
where
    I: 'static + Interact<T, R>,
{
    item: I,
    data: PhantomData<R>,
    pub cur_id: u32,
    pub components: HashMap<String, T>,
    pub interactions: HashMap<WorldHandle<I>, Arc<I>>,
}

impl<I, T: CatalogItem + Sync + Send + Clone, R: Sync + Send + Clone> InteractionCatalog<I, T, R>
where
    I: Interact<T, R>,
{
    pub fn get_interaction(&self, handle: WorldHandle<I>) -> Option<Arc<I>> {
        //println!("looking for interaction for {}", name);
        if let Some(interact) = self.interactions.get(&handle) {
            Some(interact.clone())
        } else {
            None
        }
    }
    pub fn add_interaction(&mut self, interaction: I) -> WorldHandle<I> {
        let handle = WorldHandle::<I> {
            id: HandleId::new(),
            ..Default::default()
        };

        self.interactions.insert(handle, Arc::new(interaction));

        handle.clone()
    }

    pub fn names(&self) -> Keys<'_, String, T> {
        self.components.keys()
    }

    pub fn iter(&self) -> Values<'_, String, T> {
        self.components.values()
    }

    pub fn categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .components
            .values()
            .map(|m| m.category().clone())
            .collect();

        categories.sort();
        categories.dedup();

        categories
    }

    pub fn items_in_category(&self, category: &str) -> Vec<&T> {
        self.components
            .values()
            .filter(|p| p.category() == category)
            .collect()
    }
}

impl Location {
    pub fn normalize(window: &Window, cam_transition: &Transform, position: &Vec2) -> Vec2 {
        let camera_offset_x: f32 = cam_transition.translation().x();
        let camera_offset_y: f32 = cam_transition.translation().y();

        let x_window_offset = window.width;
        let y_window_offset = window.height;

        let normalized_x = position.x() + camera_offset_x - (x_window_offset / 2) as f32;
        let normalized_y = position.y() + camera_offset_y - (y_window_offset / 2) as f32;

        return Vec2::new(normalized_x, normalized_y);
    }
}

impl From<Transform> for Location {
    fn from(t: Transform) -> Self {
        Location::new(t, WorldLocation::World)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Visible;

#[derive(Copy, Clone, Debug)]
pub struct Solid;

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
    Undefined,
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
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Tile;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Item;

#[derive(Clone, Debug, PartialEq, Default, Bundle)]
pub struct ItemComponents {
    pub name: crate::Named,
    pub weight: Weight,
    pub handle: WorldHandle<Item>,
    pub item_type: ItemType,
    pub item_slot: ItemSlot,
    pub description: ItemDefinition,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Property)]
pub struct HandleId(pub Uuid);

impl HandleId {
    pub fn new() -> HandleId {
        HandleId(Uuid::new_v4())
    }
}

#[derive(Properties)]
pub struct WorldHandle<T>
where
    T: 'static,
{
    pub id: HandleId,
    #[property(ignore)]
    marker: PhantomData<T>,
}
impl<T> Hash for WorldHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for WorldHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for WorldHandle<T> {}

impl<T> Debug for WorldHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let name = std::any::type_name::<T>().split("::").last().unwrap();
        write!(f, "WorldHandle<{}>({:?})", name, self.id.0)
    }
}

impl<T> Default for WorldHandle<T> {
    fn default() -> Self {
        WorldHandle {
            id: HandleId::new(),
            marker: PhantomData,
        }
    }
}

impl<T> Clone for WorldHandle<T> {
    fn clone(&self) -> Self {
        WorldHandle {
            id: self.id,
            marker: PhantomData,
        }
    }
}
impl<T> Copy for WorldHandle<T> {}

// SAFE: T is phantom data and Handle::id is an integer
unsafe impl<T> Send for WorldHandle<T> {}
unsafe impl<T> Sync for WorldHandle<T> {}

#[derive(Copy, Clone, Debug, Properties, PartialEq, Default)]
pub struct Weight(pub f32);

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Inventory(pub Vec<WorldHandle<Item>>);

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ItemData {
    handle: WorldHandle<ItemData>,
    name: String,
    description: String,
}
