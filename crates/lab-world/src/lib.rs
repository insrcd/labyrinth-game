use lab_core::prelude::*;
use std::{fmt::Debug};
use lab_sprites::SpriteInfo;

mod systems;

pub mod settings {
    pub const TILE_SIZE : f32 = 16.;
    pub const WORLD_TILE_SIZE : f32 = 16.;
    pub const PLAYER_SPEED : f32 = 48.;
}

/// Plugin that will setup all of the rules of the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(InteractionCatalog::<TileInteraction, TileComponents, Vec<TileInteractionResult>>::default())
            .add_resource(UiTextState::default())
            .add_resource(InteractionState::default())
            .add_event::<TextChangeEvent>()
            .add_event::<InteractionEvent>()
            .add_event::<TileInteractionResultEvent>()
            //.add_system(systems::add_world_sprites_system.system())
            //.add_system(systems::add_interaction_sprites_system.system())    
            .add_system_to_stage(lab_core::stages::PRE_UPDATE, systems::zoom_system.system())
            .add_system(systems::save_world_system.thread_local_system())
            .add_system(systems::collision_system.system())            
            .add_system(systems::sprite_despawn_system.system())
            .add_system_to_stage(lab_core::stages::POST_UPDATE, systems::interaction_system.system())
            .add_system_to_stage(lab_core::stages::POST_UPDATE, systems::process_interaction_result_system.system())
            .add_system(systems::add_text_to_adventure_log.system())
            .add_system_to_stage(lab_core::stages::POST_UPDATE, systems::static_text_system.system())
            .add_system(systems::camera_tracking_system.system());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

impl CatalogItem for TileComponents {
    fn category(&self) -> String {
        self.sprite.category.clone()
    }
    fn name(&self) -> String {
        self.sprite.name.clone()
    }
}

pub struct TileInteractionResultEvent{
    _source: Entity,
    destination: Entity,
    result: TileInteractionResult
}
#[derive(Clone, Debug, PartialEq)]
pub enum TileInteractionResult {    
    Damage(Entity, Entity, f32),
    ChangeSprite(Entity, SpriteInfo),
    ChangeInventory(Entity,Inventory),
    AddItem(Entity, ItemComponents),
    ChangeState(Entity, ObjectState),
    Move(Entity, Location),
    Despawn,
    Log(String),
    Message(String),
    Menu(MenuDefinition),
    Block(Entity),
    None
}

impl Default for TileInteractionResult {

    fn default() -> Self {
        TileInteractionResult::None
    }
}

impl From<TileInteractionResult> for Vec<TileInteractionResult> {
    fn from( n : TileInteractionResult) -> Self {
        vec![n]
    }
}

#[derive(Copy, Clone)]
pub struct TileInteraction {
    pub description: &'static str,
    pub caller : fn (InteractionContext<TileInteraction, TileComponents, Vec<TileInteractionResult>>) -> Vec<TileInteractionResult>
}

impl Debug for TileInteraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interaction")
        .field("description", &self.description)
        .finish()
    }
}


impl Interact <TileComponents, Vec<TileInteractionResult>> for TileInteraction {

    fn interact(&self, ctx : InteractionContext<Self, TileComponents, Vec<TileInteractionResult>>) -> Vec<TileInteractionResult> {
        (self.caller)(ctx)
    }
}

impl Default for TileInteraction {

    fn default() -> Self {
        TileInteraction {
            description:"Default Interaction",
            caller : |_| TileInteractionResult::None.into()
        }
    }
}

#[derive(Bundle, Debug)]
pub struct TileComponents {
    pub name: Named,
    pub location: Location,
    pub sprite: SpriteInfo,
    pub state: ObjectState,
    pub zoomable: Zoomable,
    pub handle: WorldHandle<Tile>,
    pub interaction: WorldHandle<TileInteraction>
}
impl Clone for TileComponents {
    fn clone(&self) -> Self {
        TileComponents {
            name: self.name.clone(),
            location: self.location.clone(),
            sprite: self.sprite.clone(),
            state: self.state.clone(),
            zoomable: self.zoomable.clone(),
            handle: WorldHandle::default(),
            interaction: self.interaction.clone()
        }
    }
}
impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            name: Named::default(),
            location: Location::default(),
            sprite: SpriteInfo::default(),
            zoomable: Zoomable,
            state: ObjectState::default(),
            handle: WorldHandle::default(),
            interaction: WorldHandle::default()
        }
    }
}

pub struct TextChangeEvent {
    pub text: String,
    pub name: String
}
#[derive(Default)]
pub struct UiTextState {
    pub change_events: EventReader<TextChangeEvent>, 
}
#[derive(Default)]
pub struct InteractionState {
    pub interaction_events: EventReader<InteractionEvent>, 
    pub interaction_results: EventReader<TileInteractionResultEvent>
}
pub struct MoveTimer(pub Timer);
pub struct DialogTimer(pub Timer);

pub type TilePalette = InteractionCatalog<TileInteraction, TileComponents, Vec<TileInteractionResult>>;
