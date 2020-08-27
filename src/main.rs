
use bevy::{
    prelude::*
};

mod dialog;
mod state;

use lab_entities::prelude::*;
use lab_input::*;
use dialog::*;
use lab_sprites::*;
use lab_core::{stage,*};
use lab_world::StaticText;

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
    .add_startup_stage(stage::INIT)
    .add_startup_stage(stage::POST_INIT)
    .add_stage_after(stage::UPDATE, stage::PROCESSING)
    .add_stage_after(stage::PROCESSING, stage::POST_UPDATE)
    .add_plugin(lab_core::CorePlugin)   
    .add_plugin(lab_sprites::SpritesPlugin)
    .add_plugin(lab_input::InputPlugin)
    .add_plugin(lab_world::WorldPlugin)
    .add_plugin(lab_builder::BuilderPlugin)
    .add_startup_system_to_stage(stage::POST_INIT, setup.system())        
    .add_system(npc_dialog_system.system())
    .add_system_to_stage(stage::POST_UPDATE, dialog_system.system())
    .add_plugin(lab_demo::DemoPlugin)
    .add_system(state::state_transition.system())
    //.add_system(update_ui_text_system.system())
    //.add_system(test.system())
    .run();
}



fn setup (
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Font>>
) {
    
    let npc_sprite = sprites.get("mob_0").unwrap_or_else(|| panic!("Cannot find NPC sprite")).clone();   
    
    let walk_left = sprites.sprites_in_category("walk_left");        
    let walk_right = sprites.sprites_in_category("walk_right");

    let player_sprite = walk_left[0].clone();

    let font_handle = asset_server.load_sync(&mut assets, "resources/fonts/FiraSans-Bold.ttf").unwrap();

    commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
   
    .spawn(( state::SceneState { next_state: state::StateType::Init }, ))
    .spawn( (Mouse::default(),) )
    .spawn( 
        PlayerComponents::new("Adam"))
        .with_bundle(player_sprite.to_components(Vec3::new(-64., -64., layers::PLAYER), Scale(1.)))
        .with( MoveAnimation {
            up: walk_right[3..6].to_vec(), 
            down: walk_left[0..4].to_vec(),
            left: walk_left[0..4].to_vec(),
            right: walk_right[3..6].to_vec(),
            ..Default::default()
        }).with(player_sprite)
    .spawn( (NonPlayer, Timer::from_seconds(5., false), Inventory::new() , Named("mob".to_string()), npc_sprite.clone(), Zoomable, Movement,Moveable))
    .with_bundle(npc_sprite.to_components(Location(100., 100., layers::PLAYER,world::WorldLocation::World).into(), Scale(1.)))
    .spawn(TextComponents {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {bottom:Val::Px(5.), left:Val::Px(0.), ..Default::default()},
            ..Default::default()
        },
        text: Text {
            value: "Welcome to Labyrinth".to_string(),
            font: font_handle,
            style: TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
            },
        },
        draw: Draw {is_visible: true, ..Default::default()},
        ..Default::default()
    }).with_bundle((Named("main".to_string()), StaticText, Translation::new(60.,0.,0.)));    
    
    /*
    for _n in 0..50 {
        commands.spawn((NonPlayer, Inventory::new() , Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50., world::WorldLocation::World), sprite.clone()),);
    }*/
}
