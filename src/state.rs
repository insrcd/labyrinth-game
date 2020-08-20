/// Module for all state
/// 
/// 

use bevy::{prelude::*, render::*};
use crate::{assets::SpriteLibrary, menu::*, world::Despawn};
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
    mut state : ResMut<GameState>,
    mut lib : Res<SpriteLibrary>,
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
                
                let window = windows.iter().last().unwrap();
                let sprites = lib.make_string("This is a test & Stuff".to_string(), 
                                Vec3::new(0. - window.width as f32/2. + 48. , window.height as f32/2. as f32 -96.,100.)).into_iter();
                for it in sprites
                 {
                    commands
                        .spawn(
                        it
                        ).with(Timer::new(Duration::from_secs(2)))
                        .with(Despawn);
                    
                    lib.write_despawning_text(&mut commands, "Welcome to Labyrinth, the Game!".to_string(), 
                        Duration::from_secs(5), 
                        Vec3::new(16. - (window.width/2) as f32, 16. - (window.height/2) as f32, 100.)
                    )
                    
                }
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
