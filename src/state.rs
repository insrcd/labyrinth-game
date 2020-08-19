/// Module for all state
/// 
/// 

use crate::menu::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
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
    state: StateType,
    edit_mode: bool
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            state: StateType::Init,
            edit_mode: false
        }
    }
    
}
