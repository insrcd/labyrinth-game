/// Module for all state
/// 
/// 

use crate::menu::*;

#[derive(Clone, Debug)]
pub enum StateType {
    Init,
    Menu(Menu),
    CharacterScreen,
    Map,
    Combat
}

#[derive(Clone)]
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
