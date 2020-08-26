/// Module for game state

use bevy::{prelude::*, render::*};
use lab_entities::{Named, world::*};
use lab_input::prelude::*;
use lab_sprites::*;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum StateType {
    Init,
    Paused,
    Menu(Menu),
    CharacterScreen,
    Map,
    Combat
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct GameState {
    pub current_state: StateType,
    pub edit_mode: bool
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            current_state: StateType::Init,
            edit_mode: false
        }
    }
}

pub struct SceneState {
    pub next_state: StateType
}

pub fn state_transition (
    mut commands : Commands,
    windows : Res<Windows>,
    sprite_library: Res<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<Assets<Font>>,
    mut query : Query<(Changed<SceneState>,)>
) {
    for (state,) in &mut query.iter() {
       
        match &state.next_state {

            StateType::Init => { 
                
            }
            StateType::Menu(menu) => {}
            StateType::CharacterScreen => {}
            StateType::Map => {}
            StateType::Combat => {}
            _ => {}
        }
    }
}
