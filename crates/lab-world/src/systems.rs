use bevy::{prelude::*, math::Vec2, ecs::{DynamicBundle, Bundle}, prelude::Properties, render::camera::Camera, type_registry::TypeRegistry};

use strum_macros::EnumIter;

#[allow(dead_code)]
pub mod stage {
    pub const WORLD: &'static str = "world";
}

pub mod settings {
    pub const TILE_SIZE : f32 = 96.;
}

/// Plugin that will setup all of the rules of the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(npc_move.system())
            .add_system(make_room.system())
            .add_system(add_player_sprites.system())
            .add_system(save_world.thread_local_system())
            .add_system(collision_detection.system());
    }
}