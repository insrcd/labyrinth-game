/// This module will define all interaction concepts and components
/// 
/// 
use crate::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InteractableType {
    Player,
    Npc,
    Item,
    Spell,
    Weapon,
    Tile,
    None
}

impl Default for InteractableType {
    fn default() -> Self {
        return Self::None
    }
}


pub struct InteractionContext <'a, T : CatalogItem + Send + Sync + Clone> {
    pub source: &'a Interactable <'a>,
    pub destination: &'a Interactable <'a>,
    // resources
    pub world_catalog: Option<&'a InteractionCatalog<'a, T>>
}


#[derive(Debug)]
pub struct Interactable <'a> {
    pub entity: Entity,
    pub inventory: Option<RefMut<'a, crate::world::WorldLocation>>,
    pub interactable_type: InteractableType,
    pub location: Location,
    pub tile_state: Option<&'a RefMut<'a, ObjectState>>
}

/// Events
pub enum InteractionType {
    Collision,
    Action(String)
}
pub struct InteractionEvent {
    pub source : Entity,
    pub destination : Entity,
    pub interaction_type: InteractionType
}