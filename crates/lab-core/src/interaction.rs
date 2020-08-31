/// This module will define all interaction concepts and components
/// 
/// 
use crate::prelude::*;
use std::borrow::Cow;


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
pub trait CatalogItem {
    fn name(&self) -> String;
    fn category(&self) -> String;
}

pub trait Interact <T : CatalogItem + Sync + Send + Clone + ?Sized, R : Sync + Send + Clone + ?Sized> 
where Self : Clone + Sized {
    fn interact(ctx : InteractionContext<Self, T, R>) -> R;
} 

pub struct InteractionContext <'a, I, T : CatalogItem + Send + Sync + Clone, R: Send + Sync + Clone> 
where I : Interact<T, R> {
    pub source: &'a Interactable <'a>,
    pub destination: &'a Interactable <'a>,
    // resources
    pub world_catalog: Option<&'a InteractionCatalog<I, T, R>>
}


#[derive(Debug)]
pub struct Interactable <'a> {
    pub entity: Entity,
    pub inventory: Option<RefMut<'a, crate::world::Inventory>>,
    pub interactable_type: InteractableType,
    pub location: Location,
    pub tile_state: Option<&'a Cow<'a, ObjectState>>
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