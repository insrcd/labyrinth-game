

const PLAYER_SPEED : f32  = 8.;

use bevy::{
    prelude::*,
    render::{camera::Camera},
    input::{keyboard::KeyCode, Input },
};

use lab_sprites::*;

use crate::*;
use lab_core::WorldSettings;

/// System to sample mouse wheel events and update athe ScrollState
/// resource that can be used by other systems.

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

                // scale by whole numbers
                scroll_state.y = (scroll_state.y + mw.y).round();
                scroll_state.x = (scroll_state.x + mw.x).round();
                
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
    scroll_state : Res<ScrollState>,
    world : Res<WorldSettings>,
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

        let window = windows.iter().last().unwrap();
        let x_window_offset = window.width;
        let y_window_offset = window.height;
        
        for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
            
            let mut normalized_x = event.position.x() + camera_offset_x - (x_window_offset/2) as f32;
            let mut normalized_y = event.position.y() + camera_offset_y - (y_window_offset/2) as f32;
        
            // snap to grid

            let grid_x = normalized_x  / (world.tile_size * scroll_state.current_scale);
            let grid_y = normalized_y  / (world.tile_size * scroll_state.current_scale);    

            normalized_x = grid_x.round() * world.tile_size * scroll_state.current_scale;
            normalized_y = grid_y.round() * world.tile_size * scroll_state.current_scale;
            
            // backwards compat            
            mouse.position = Vec2::new(normalized_x, normalized_y);

            // fields that give us both ui and world positions
            mouse.world_position = Vec3::new(normalized_x, normalized_y, 0.);
            mouse.ui_position = event.position.clone();

            log::trace! ("Mouse position: {:?}", *mouse);
        }
}


pub fn mouse_click_system (
    mouse_input: Res<Input<MouseButton>>,
    mut events : ResMut<Events<MouseClickEvent>>,
    time : Res<Time>,
    mouse : ResMut<Mouse>
) {    
    let button = if mouse_input.just_pressed(MouseButton::Left) {
        Some(MouseButton::Left)
    } else if mouse_input.just_pressed(MouseButton::Right)  {
        Some(MouseButton::Right)
    } else if mouse_input.just_pressed(MouseButton::Middle)  {
        Some(MouseButton::Middle)
    } else { None };

    if let Some(button) = button { 
        events.send(MouseClickEvent {  
            timestamp: time.seconds_since_startup,
            button : button,
            ui_position: mouse.ui_position,
            world_position: mouse.world_position}) 
    };

}

pub fn player_movement_system (
    time : Res<Time>,
    world_settings : Res<WorldSettings>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&player::Player, &Scale, &mut Translation, &mut Movement, &mut MoveAnimation, &mut TextureAtlasSprite, &mut lab_core::InputTimer, &mut Handle<TextureAtlas>)>) {


    let mut direction = CardinalDirection::None;

    if keyboard_input.pressed(KeyCode::W) {
        direction = CardinalDirection::North;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction = CardinalDirection::South;
    }

    if keyboard_input.pressed(KeyCode::A) {
        direction = CardinalDirection::West;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction = CardinalDirection::East;
    }

    let player_speed = world_settings.base_player_speed;

    for (_player, scale, mut loc, mut movement, mut animation, mut texture_sprite, mut timer, mut atlas) in &mut query.iter() {   
        timer.0.tick(time.delta_seconds);
        if  timer.0.finished {
            let old_loc = Location::from(*loc);

            let sprite = match direction {
                CardinalDirection::North => {
                    *loc.0.y_mut() += player_speed * scale.0;
                    animation.count = (animation.count + 1) % animation.up.len();
                    Some(animation.up[animation.count].clone())
                },
                CardinalDirection::South => {
                    *loc.0.y_mut() -= player_speed * scale.0;
                    animation.count = (animation.count + 1) % animation.down.len();
                    Some(animation.down[animation.count].clone())
                },
                CardinalDirection::West => {
                    *loc.0.x_mut() -= player_speed * scale.0;
                    animation.count = (animation.count + 1) % animation.left.len();
                    Some(animation.left[animation.count].clone())
                },
                CardinalDirection::East => {
                    *loc.0.x_mut() += player_speed * scale.0;
                    animation.count = (animation.count + 1) % animation.right.len();
                    Some(animation.right[animation.count].clone())
                },
                CardinalDirection::None => {
                    None
                }
            };

            if direction != CardinalDirection::None {
                *movement = Movement::new(old_loc, Location::from(*loc), direction);

                if let Some(s) = sprite {
                    *atlas = s.atlas_handle;
                    (*texture_sprite).index = s.atlas_sprite;
                }
            }
            timer.0.reset();
        }
    }      
}