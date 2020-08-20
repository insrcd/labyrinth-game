/// Module for all state
/// 
/// 

use bevy::{prelude::*, render::*};
use crate::{assets::SpriteLibrary, menu::*};
use lab_entities::world::Despawn;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum StateType {
    Init,
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
                
                let window = windows.iter().last().unwrap();
                let sprites = sprite_library.make_string("This is a test & Stuff".to_string(), 
                                Vec3::new(0. - window.width as f32/2. + 48. , window.height as f32/2. as f32 -96.,100.)).into_iter();
                for it in sprites
                 {
                    commands
                        .spawn(
                        it
                        )
                        .with(Timer::new(Duration::from_secs(2)))
                        .with(Despawn);
                    
                    sprite_library.write_despawning_text(&mut commands, "Welcome to Labyrinth, the Game!".to_string(), 
                        Duration::from_secs(5), 
                        Vec3::new(16. - (window.width/2) as f32, 16. - (window.height/2) as f32, 100.));   
                }
            }
            StateType::Menu(menu) => {}
            StateType::CharacterScreen => {}
            StateType::Map => {}
            StateType::Combat => {}
        }
    }
}
