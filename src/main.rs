use bevy::prelude::*;

mod player;

enum GameState {
    Init,
    MainMenu,
    CharacterScreen,
    Map
}

// resource for current location


fn main() {
    App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_startup_system(load_tiles.system())
    .add_system(test.system())
    .run();
}

fn test (
    player: &player::Player,
    name: &player::Named
) {
    println!("{} {}", player, name.0)
}


fn load_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let texture_handle = asset_server.load("resources/sprites/world.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

// generate a simple map

fn simple_map() {

}

fn simple_movement() {

}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    player::Player::add_to_world(commands, "Adam");
}