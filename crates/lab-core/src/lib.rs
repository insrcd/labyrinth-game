use bevy::{prelude::*};

mod systems;

use systems::*;

use bevy::prelude::*;

#[allow(dead_code)]
pub mod stage {
    /// Stage for initializing resources (used for startup systems)
    pub const INIT: &'static str = "init";
    /// Stage after initializing resources (used for startup systems)
    pub const POST_INIT: &'static str = "post_init";
    pub const PRE_UPDATE: &'static str = "pre_update";
    /// Default stage
    pub const UPDATE: &'static str = "update";
    /// Stage for processing after an update
    pub const PROCESSING: &'static str = "processing";
    /// Stage after update / processing    
    pub const POST_UPDATE: &'static str = "postupdate";
}
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        
    }
}
