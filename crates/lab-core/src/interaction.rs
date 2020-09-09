/// This module will define all interaction concepts and components
/// 
/// 
use crate::prelude::*;

use bevy::ecs::ResMut;


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
    fn interact(&self, ctx : InteractionContext<Self, T, R>) -> R;
} 

pub struct InteractionContext <I, T : CatalogItem + Send + Sync + Clone, R: Send + Sync + Clone> 
where I : Interact<T, R> {
    pub source: Interactable,
    pub destination: Interactable,
    // resources
    pub world_catalog: InteractionCatalog<I, T, R>,
    pub item_storage: ItemStorage
}


#[derive(Debug)]
pub struct Interactable {
    pub entity: Entity,
    pub inventory: crate::world::Inventory,
    pub interactable_type: InteractableType,
    pub location: Location,
    pub tile_state: Option<ObjectState>
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