

const PLAYER_SPEED : f32  = 8.;

use bevy::{
    prelude::*,
    render::{camera::Camera},
    input::{keyboard::KeyCode, Input, mouse::{MouseButtonInput, MouseMotion} },
};

use lab_sprites::*;
use lab_core::*;

use crate::*;

pub fn mouse_wheel_system (
    time : Res<Time>,
    mut state: ResMut<State>,
    mut scroll_state : ResMut<ScrollState>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mut query : Query<&mut ScrollTimer> // timer samples every .1 seconds
) {    
    
    for mut timer in &mut query.iter() {
        timer.0.tick(time.delta_seconds);
        if timer.0.finished {
            for event in state.mouse_wheel_event_reader.iter(&mouse_wheel) {
            
                let mw : &MouseWheel = event.into();

                scroll_state.y += mw.y;
                scroll_state.x += mw.x;
                
                timer.0.reset()
            }
        } else {
            scroll_state.y = 0.;
            scroll_state.x = 0.;
        }
    }
}

pub fn track_mouse_movement_system(
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut state: ResMut<State>,
    mut mouse: ResMut<Mouse>,
    windows: Res<Windows>,
    mut camera_query: Query<(&Camera, &Translation)>) {
        let mut camera_offset_x : f32 = 0.;
        let mut camera_offset_y : f32 = 0.;
        
        for (c, t) in &mut camera_query.iter(){
            if *(c.name.as_ref()).unwrap_or(&"".to_string()) != "UiCamera" {
                camera_offset_x = t.x();
                camera_offset_y = t.y() ;
            }
        }

        /*for window in windows.iter() {
            println!("{:?}",window);
        }*/

        let window = windows.iter().last().unwrap();
        let x_window_offset = window.width;
        let y_window_offset = window.height;
        
        for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
            //println!("{},{} - {},{}", camera_offset_x, camera_offset_y, event.position.x(), event.position.y() );

            mouse.position = Vec2::new(event.position.x() + camera_offset_x - (x_window_offset/2) as f32, event.position.y() + camera_offset_y - (y_window_offset/2) as f32);
        }
}




pub fn player_movement_system (
    mut commands: Commands,
    windows : Res<Windows>,
    time : Res<Time>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut selected_tile: ResMut<SelectedTile>, 
    mut query: Query<(&player::Player, &Scale, &mut Translation, &mut player::Movement, &mut MoveAnimation, &mut TextureAtlasSprite, &mut lab_core::InputTimer, &mut Handle<TextureAtlas>)>) {


    let mut direction = player::Direction::Stationary;

    let window = windows.iter().last().unwrap();

    if keyboard_input.pressed(KeyCode::W) {
        direction = player::Direction::Up;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction = player::Direction::Down;
    }

    if keyboard_input.pressed(KeyCode::A) {
        direction = player::Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction = player::Direction::Right;
    }

    for (_player, scale, mut loc, mut Movement, mut animation, mut texture_sprite, mut timer, mut atlas) in &mut query.iter() {   
        timer.0.tick(time.delta_seconds);
        if  timer.0.finished {
            let old_loc = Location::from(*loc);

            let sprite = match direction {
                player::Direction::Up => {
                    *loc.0.y_mut() += PLAYER_SPEED * scale.0;
                    animation.count = (animation.count + 1) % animation.up.len();
                    Some(animation.up[animation.count].clone())
                },
                player::Direction::Down => {
                    *loc.0.y_mut() -= PLAYER_SPEED * scale.0;
                    animation.count = (animation.count + 1) % animation.down.len();
                    Some(animation.down[animation.count].clone())
                },
                player::Direction::Left => {
                    *loc.0.x_mut() -= PLAYER_SPEED * scale.0;
                    animation.count = (animation.count + 1) % animation.left.len();
                    Some(animation.left[animation.count].clone())
                },
                player::Direction::Right => {
                    *loc.0.x_mut() += PLAYER_SPEED * scale.0;
                    animation.count = (animation.count + 1) % animation.right.len();
                    Some(animation.right[animation.count].clone())
                },
                player::Direction::Stationary => {
                    None
                }
            };

            if direction != player::Direction::Stationary {
                *Movement = player::Movement(old_loc, Location::from(*loc), direction);

                if let Some(s) = sprite {
                    *atlas = s.atlas_handle;
                    *texture_sprite = TextureAtlasSprite::new(s.atlas_sprite);
                }
            }
            timer.0.reset();
        }
    }      
}