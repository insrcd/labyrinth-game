
use bevy::{
    prelude::*
};

mod state;

use lab_entities::prelude::*;
use lab_input::*;

const TILE_SIZE : f32 = 96.;

#[allow(dead_code)]
pub mod stage {
    pub const POSTSETUP: &'static str = "post";
}

fn main() {
    App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_plugin(lab_sprites::SpritesPlugin)
    .add_plugin(lab_input::InputPlugin)
    .add_plugin(lab_world::WorldPlugin)
    .add_plugin(lab_builder::BuilderPlugin)
    //.add_plugin(demo::DemoPlugin)
    .add_system(state::state_transition.system())
    //.add_system(test.system())
    .run();
}



fn setup (
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    

    commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
   
    .spawn(( state::SceneState { next_state: state::StateType::Init }, ))
    .spawn(( Mouse { position: Vec2::new(0.,0.)},))
    .spawn((Player { god_mode: false }, Named("Adam".to_string()), Location(0., 0., 51.)))
    .spawn((NonPlayer, Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50.)));
    //Player::add_to_world(commands, "Adam");
}
