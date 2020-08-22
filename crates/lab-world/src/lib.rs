use bevy::{prelude::*, math::Vec2, ecs::{DynamicBundle, Bundle}, prelude::Properties,
     render::camera::Camera, type_registry::TypeRegistry};

use strum_macros::EnumIter;
use lab_core::stages;
mod systems;

#[allow(dead_code)]
pub mod stage {
    pub const WORLD: &'static str = "world";
}

pub mod settings {
    pub const TILE_SIZE : f32 = 96.;
    pub const WORLD_TILE_SIZE : f32 = 96.;
    pub const PLAYER_SPEED : f32 = 48.;
}

/// Plugin that will setup all of the rules of the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(systems::npc_move_system.system())
            .add_system(systems::add_world_sprites_system.system())
            .add_system(systems::add_interaction_sprites_system.system())
            .add_system(systems::save_world_system.thread_local_system())
            .add_system(systems::tile_interaction_system.system())            
            .add_system(systems::sprite_despawn_system.system())
            .add_system_to_stage(stage::PROCESSING, systems::static_text_system.system());
            .add_system(systems::object_interaction_system.system());
    }
}