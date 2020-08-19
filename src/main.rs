
use bevy::{
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    input::{keyboard::KeyCode, Input, mouse::{MouseButtonInput, MouseMotion}}, type_registry::TypeRegistry,
};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 

mod world;
mod assets;
mod scripting;
mod player;
mod objs;
mod demo;
mod input;
mod state;
mod menu;
mod components;

use crate::assets::*;
use crate::player::*;
use crate::world::*;
use crate::state::*;
use crate::menu::*;
use crate::components::*;

const tile_size : f32 = 96.;

pub mod stage {
    pub const POSTSETUP: &'static str = "post";
}
fn main() {
    App::build()
    .init_resource::<input::State>()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_startup_system(load_world_sprites.system())
    //.add_plugin(demo::DemoPlugin)
    .add_plugin(input::InputPlugin)
    .add_plugin(WorldPlugin)
    //.add_system(test.system())
    .run();
}



fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
    .spawn(( input::Mouse { position: Vec2::new(0.,0.)},))
    .spawn((Player { god_mode: false }, Named("Adam".to_string()), Location(0., 0., 0.)))
    .spawn((NonPlayer, Named("OldDude".to_string()), Location(tile_size*3., -tile_size*4., 0.)));
    //Player::add_to_world(commands, "Adam");
}
