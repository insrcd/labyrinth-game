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

pub struct InteractionContext <'a, I, T : CatalogItem + Send + Sync + Clone, R: Send + Sync + Clone> 
where I : Interact<T, R> + 'static {
    pub source: Entity,
    pub destination: Entity,
    // resources
    pub items: &'a ResMut<'a, Items>,
    pub world_catalog: InteractionCatalog<I, T, R>,
    pub interaction_query: &'a Query<'a, (
        Entity,
        &'a InteractableType,
        &'a Named,
        &'a ObjectState,
        &'a WorldHandle<I>,
        &'a WorldHandle<Tile>,
        &'a Inventory
    )>,
    pub item_query: &'a Query<'a, (
        Entity,
        &'a ItemType,
        &'a Named,
    )>
}

#[derive(Debug, Bundle, Default)]
pub struct Interactable {
    pub inventory: Inventory,
    pub interactable_type: InteractableType,
    pub state: ObjectState
}

impl Interactable {
    pub fn new(interact_type: InteractableType) -> Interactable {
        Interactable {
            interactable_type: interact_type,
            ..Default::default()
        }
    }
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