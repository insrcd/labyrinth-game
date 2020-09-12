
use bevy::{
    prelude::*
};

mod state;

use lab_entities::prelude::*;
use lab_input::*;
use lab_sprites::*;
use lab_core::{stage,*};
use lab_world::{TextChangeEvent};

pub mod layers {
    // z indexes of sprites 
    pub const TOP : f32 = 999.;
    pub const BOTTOM : f32 = -999.;
    pub const ITEM : f32 = 30.;
    pub const TILE : f32    = 1.;
    pub const PLAYER : f32 = 35.;
    pub const NPC : f32    = 40.;
    pub const ABOVE_PLAYER : f32 = 40.;
}

fn main() {
    App::build()
    .add_default_plugins()    
    .add_startup_stage(stages::INIT)
    .add_startup_stage(stages::POST_INIT)
    .add_stage_before(stages::UPDATE, stages::VALIDATION)
    .add_stage_after(stages::UPDATE, stages::POST_UPDATE)
    .add_plugin(lab_core::CorePlugin)
    .add_plugin(lab_sprites::SpritesPlugin)
    .add_plugin(lab_input::InputPlugin)
    .add_plugin(lab_world::WorldPlugin)
    .add_plugin(lab_builder::BuilderPlugin)
    .add_plugin(lab_ai::AiPlugin)
    .add_startup_system_to_stage(stages::POST_INIT, setup.system())     
    .add_plugin(lab_demo::DemoPlugin)
    .add_system(state::state_transition.system())
    .run();
}



fn setup (
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>,
    mut log: ResMut<AdventureLog>,
    mut text_change: ResMut<Events<TextChangeEvent>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Font>>
) {
    
    let walk_left = sprites.sprites_in_category("walk_left");        
    let walk_right = sprites.sprites_in_category("walk_right");

    let player_sprite = walk_left[0].clone();

    let font_handle = asset_server.load_sync(&mut assets, "resources/fonts/FiraSans-Bold.ttf").unwrap();


    log
        .make(&mut commands, font_handle, 4)
        .add_message("Welcome to Labyrinth".to_string())
        .add_message("This is a demo at this point".to_string())
        .add_message("But enjoy testing it out".to_string());

        text_change.send(TextChangeEvent { name: "".to_string(), text: "Get Adventuring!".to_string()} );

    commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
   
    .spawn(( state::SceneState { next_state: state::StateType::Init }, ))
    .spawn( (Mouse::default(),) )
    .spawn( 
        PlayerComponents::new("Adam"))
        .with_bundle( player_sprite.to_components(Vec3::new(-64., -64., layers::PLAYER), Scale(1.)))
        .with_bundle(Interactable::new(InteractableType::Player))
        .with(MoveAnimation {
            up: walk_right[3..6].to_vec(), 
            down: walk_left[0..4].to_vec(),
            left: walk_left[0..4].to_vec(),
            right: walk_right[3..6].to_vec(),
            ..Default::default()});
    
    /*
    for _n in 0..50 {
        commands.spawn((NonPlayer, Inventory::new() , Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50., world::WorldLocation::World), sprite.clone()),);
    }*/
}
