
use bevy::{
    prelude::*
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
use crate::components::*;

const TILE_SIZE : f32 = 96.;

#[allow(dead_code)]
pub mod stage {
    pub const POSTSETUP: &'static str = "post";
}

fn main() {
    App::build()
    .init_resource::<input::State>()
    .init_resource::<state::GameState>()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_startup_system(load_world_sprites.system())
    .add_plugin(demo::DemoPlugin)
    .add_plugin(input::InputPlugin)
    .add_plugin(WorldPlugin)
   // .add_system(state::state_transition.system())
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
    .spawn(TextComponents {
        text: Text {
            font: asset_server.load("resources/fonts/FiraSans-Bold.ttf").unwrap(),
            value: "Score:".to_string(),
            style: TextStyle {
                color: Color::rgb(0.2, 0.2, 0.8).into(),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .spawn(( state::SceneState { next_state: state::StateType::Init }, ))
    .spawn(( input::Mouse { position: Vec2::new(0.,0.)},))
    .spawn((Player { god_mode: false }, Named("Adam".to_string()), Location(0., 0., 51.)))
    .spawn((NonPlayer, Named("OldDude".to_string()), Location(TILE_SIZE, -TILE_SIZE, 50.)));
    //Player::add_to_world(commands, "Adam");
}
