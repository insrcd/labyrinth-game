
use bevy::{
    prelude::*
};

mod dialog;
mod state;

use lab_entities::prelude::*;
use lab_input::*;
use dialog::*;
use lab_sprites::*;
use lab_core::stage;


const TILE_SIZE : f32 = 16.;
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
    //.add_plugin(lab_demo::DemoPlugin)
    .add_system(state::state_transition.system())
    //.add_system(test.system())
    .run();
}



fn setup (
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>
) {
    
    let mut npc_sprite = sprites.get("mob_0").unwrap_or_else(|| panic!("Cannot find NPC sprite")).clone();
    let mut player_sprite = sprites.get("move_down_1").unwrap_or_else(|| panic!("Cannot find Player sprite")).clone();

    commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
   
    .spawn(( state::SceneState { next_state: state::StateType::Init }, ))
    .spawn(( Mouse { position: Vec2::new(0.,0.)}, Translation::new(0.,0.,0.)))
    .spawn( 
        PlayerComponents::new("Adam", 
        Location(-TILE_SIZE, -TILE_SIZE, 51.,world::WorldLocation::World)))
        .with_bundle(player_sprite.to_components(Location(-TILE_SIZE, -TILE_SIZE, 51.,world::WorldLocation::World).into(), Scale(1.)))
        .with( MoveAnimation {
            up: vec![sprites.get("move_up_1").unwrap().clone(), sprites.get("move_up_2").unwrap().clone()], 
            down: vec![sprites.get("move_down_1").unwrap().clone(), sprites.get("move_down_2").unwrap().clone()],
            left: vec![sprites.get("move_left_1").unwrap().clone(), sprites.get("move_left_2").unwrap().clone()],
            right: vec![sprites.get("move_right_1").unwrap().clone(), sprites.get("move_right_2").unwrap().clone()],
            ..Default::default()
        }).with(player_sprite)
    .spawn( (NonPlayer, Inventory::new() , Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50., world::WorldLocation::World), npc_sprite.clone()),);
    
    /*
    for _n in 0..50 {
        commands.spawn((NonPlayer, Inventory::new() , Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50., world::WorldLocation::World), sprite.clone()),);
    }*/
}
