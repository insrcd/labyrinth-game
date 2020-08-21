

use lab_world::settings::PLAYER_SPEED;
use bevy::{
    prelude::*,
    render::{camera::Camera},
    input::{keyboard::KeyCode, Input, mouse::{MouseButtonInput, MouseMotion} },
};

use crate::*;

pub fn track_mouse_movement_system(
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut state: ResMut<State>,
    windows: Res<Windows>,
    mut mouse_query: Query<&mut Mouse>,
    mut camera_query: Query<(&Camera, &Translation)>) {
        let mut camera_offset_x : f32 = 0.;
        let mut camera_offset_y : f32 = 0.;
        
        for (c, t) in &mut camera_query.iter(){
            if *(c.name.as_ref()).unwrap_or(&"".to_string()) == "UiCamera" {
                camera_offset_x = t.x();
                camera_offset_y = t.y();
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

            for mut mouse in &mut mouse_query.iter(){
                mouse.position = Vec2::new(event.position.x() + camera_offset_x - (x_window_offset/2) as f32, event.position.y() + camera_offset_y - (y_window_offset/2) as f32);
            }
        }
}





pub fn player_movement_system (
    mut commands: Commands,
    windows : Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut selected_tile: ResMut<SelectedTile>, 
    mut query: Query<(&player::Player, &mut Translation, &mut player::Movement)>) {

    let mut movement = player::Direction::Stationary;

    let window = windows.iter().last().unwrap();

    if keyboard_input.just_pressed(KeyCode::W) {
        movement = player::Direction::Up;
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        movement = player::Direction::Down;
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        movement = player::Direction::Left;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        movement = player::Direction::Right;
    }

    for (_player, mut loc, mut Movement) in &mut query.iter() {   
        let old_loc = Location::from(*loc);

        match movement {
            player::Direction::Up => *loc.0.y_mut() += PLAYER_SPEED,
            player::Direction::Down => *loc.0.y_mut() -= PLAYER_SPEED,
            player::Direction::Left => *loc.0.x_mut() -= PLAYER_SPEED,
            player::Direction::Right => *loc.0.x_mut() += PLAYER_SPEED,
            player::Direction::Stationary => {
            }
        }

        if movement != player::Direction::Stationary {
            *Movement = player::Movement(old_loc, Location::from(*loc), movement);
        }
    }      
}