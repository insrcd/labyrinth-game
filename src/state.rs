/// Module for all state
/// 
/// 

use bevy::{prelude::*, render::*};
use crate::{assets::SpriteLibrary, menu::*};

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
    mut state : ResMut<GameState>,
    mut sprites : Res<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<Assets<Font>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query : Query<(Changed<SceneState>,)>
) {
    for (state,) in &mut query.iter() {
        let font_handle = asset_server
            .load_sync(&mut fonts, "resources/fonts/FiraSans-Bold.ttf");

        println!("{:?}", font_handle);
            
       /*let material_handle = asset_server
            .load_sync(&mut materials, "resources/sprites/world.png")
            .unwrap();*/

        //println!("material handle {:?}", material_handle);

       
        match &state.next_state {

            StateType::Init => { 
                let it = sprites.make_string("This is a test!".to_string(), Vec3::new(0.,96.,100.)).into_iter();
        
                commands
                    .spawn_batch(
                       it
                    );
                   /* commands
                    .spawn(ImageComponents {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Auto),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(10.0),
                                right: Val::Px(10.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: material_handle,
                        ..Default::default()
                    });*/
            }
            StateType::Menu(menu) => {}
            StateType::CharacterScreen => {}
            StateType::Map => {}
            StateType::Combat => {}
        }
    }
}
