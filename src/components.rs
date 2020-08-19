/// Generic Components for multiple types.
/// 
/// 

use bevy::{
    prelude::*
};

use serde::Serialize;

#[derive(Properties, Serialize, Debug)]
pub struct Named(pub String);

impl Default for Named {
    fn default() -> Self {
        Named ("No Name".to_string())
    }
}