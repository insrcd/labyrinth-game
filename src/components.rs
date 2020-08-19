/// Generic Components for multiple types.
/// 
/// 

use bevy::{
    prelude::*
};

use serde::{Deserialize, Serialize};

#[derive(Properties, Serialize)]
pub struct Named(pub String);

impl Named {
    fn new(name : &str) -> Named {
        Named(name.to_string())
    }
}

impl Default for Named {
    fn default() -> Self {
        Named ("No Name".to_string())
    }
}