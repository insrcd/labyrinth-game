
use bevy::{
    prelude::*
};

mod dialog;
mod state;

use lab_entities::prelude::*;
use lab_input::*;
use dialog::*;
use lab_sprites::SpriteLibrary;
use lab_core::stage;

const TILE_SIZE : f32 = 96.;
fn main() {
    App::build()
    .add_default_plugins()    
    .add_startup_stage(stage::INIT)
    .add_startup_stage(stage::POST_INIT)
    .add_stage_after(stage::UPDATE, stage::PROCESSING)
    .add_stage_after(stage::PROCESSING, stage::POST_UPDATE)   
    .add_plugin(lab_sprites::SpritesPlugin)
    .add_plugin(lab_input::InputPlugin)
    .add_plugin(lab_world::WorldPlugin)
    .add_plugin(lab_builder::BuilderPlugin)
    .add_startup_system_to_stage(stage::POST_INIT, setup.system())        
    .add_system(npc_dialog_system.system())
    .add_system_to_stage(stage::PROCESSING, dialog_system.system())
    //.add_plugin(demo::DemoPlugin)
    .add_system(state::state_transition.system())
    //.add_system(test.system())
    .run();
}



fn setup (
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>,
) {
    
    let mut sprite = sprites.get("npc").unwrap_or_else(|| panic!("Cannot find NPC sprite")).clone();
    sprite.height = 48;
    sprite.width = 48;

    commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
   
    .spawn(( state::SceneState { next_state: state::StateType::Init }, ))
    .spawn(( Mouse { position: Vec2::new(0.,0.)},))
    .spawn((Player { god_mode: false }, Named("Adam".to_string()), Location(0., 0., 51.)))
    .spawn((NonPlayer, Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50.), sprite));
}
