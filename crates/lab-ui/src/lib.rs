use lab_core::prelude::*;

mod systems;
use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(ui_startup_system.system())
            .add_system_to_stage(stage::FIRST, ui_key_system.system())
            .add_system_to_stage(stage::PRE_UPDATE, inventory_ui_system.system())
            .add_system_to_stage(stage::UPDATE, button_system.system());
    }
}

#[derive(Debug)]
pub enum UiState {
    Inventory,
    Main,
    Start,
}

fn ui_startup_system(mut commands: Commands) {
    commands.spawn((UiState::Main,));
}

pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}
