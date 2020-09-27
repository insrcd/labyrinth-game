use bevy::prelude::*;
pub struct TiledDemoPlugin;

/// Load a demo that displays the basic functionality of the
/// Game framework. The demo uses the basic map builder
impl Plugin for TiledDemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_tiled::TiledMapPlugin)
        .add_startup_system(setup.system());
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(bevy_tiled::TiledMapComponents {
            map_asset: asset_server.load("resources/maps/Lab_16_2.tmx").unwrap(),
            center: true,
            origin: Transform::from_scale(3.),
            ..Default::default()
        })
        .spawn(Camera2dComponents::default());
}
