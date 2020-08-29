use lab_core::prelude::*;

mod scripting;
mod systems;
mod dialog;

pub mod prelude {
  pub use crate::scripting::*;
}

impl Plugin for NpcPlugin {
  fn build(&self, app: &mut AppBuilder) {
      app
        .init_resource::<dialog::DialogState>()
        .init_resource::<NpcState>()
        .add_event::<NpcEvent>()
  }
}

#[derive(Default)]
struct NpcState {
  
}
#[derive(Default)]
struct NpcEvent {
  
}