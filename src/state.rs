/// Module for game state
use bevy::prelude::*;
use lab_input::prelude::*;
use lab_sprites::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum StateType {
    Init,
    Paused,
    Menu(Menu),
    CharacterScreen,
    Map,
    Combat,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct GameState {
    pub current_state: StateType,
    pub edit_mode: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            current_state: StateType::Init,
            edit_mode: false,
        }
    }
}

pub struct SceneState {
    pub next_state: StateType,
}

#[allow(dead_code, unused_variables, unused_mut)]
pub fn state_transition(
    mut commands: Commands,
    windows: Res<Windows>,
    sprite_library: Res<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<Assets<Font>>,
    mut query: Query<(Changed<SceneState>,)>,
) {
    for (state,) in &mut query.iter() {
        match &state.next_state {
            StateType::Init => {}
            #[allow(unused_variables)]
            StateType::Menu(menu) => {}
            StateType::CharacterScreen => {}
            StateType::Map => {}
            StateType::Combat => {}
            _ => {}
        }
    }
}
