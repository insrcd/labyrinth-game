mod player;
mod world;
mod assets;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use assets::*;
use player::*;
use world::*;

use world::Tile;
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
    .add_system(add_player.system())
    .add_system(make_room.system())
    
    //.add_system(test.system())
    .run();
}

fn test (
    player: &Player,
    name: &Named
) {
    println!("{} {}", player, name.0)
}



fn add_player(mut commands: Commands,
    sprites : ResMut<world::SpriteLibrary>,
    mut query: Query<(Added<Player>, &Named, &world::Location)>
) {
    for (player, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
        
        commands
        .spawn(SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        });
    }
}

fn make_room (
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,    
    mut query: Query<(Added<Tile>, &Named, &mut TextureAtlasSprite,  &Handle<TextureAtlas>)>,
) {
    for (tile, name , sprite, atlas) in &mut query.iter() {
        println!("Found a tile named {:?} {}", tile.0, name.0);

        let offset = match tile.0 {
            world::TileType::Floor => 16.0*6.0,
            _ => 0.
        };

        for i in 0..10 {
            commands
            .spawn(SpriteSheetComponents {
                translation: Translation(Vec3::new(-450. + (16.0*6.*i as f32), offset + (16.0*6.), 0.0)),
                scale: Scale(6.0),
                draw: Draw { is_visible: true, ..Default::default() },
                sprite: TextureAtlasSprite::new(sprite.index),
                texture_atlas: atlas.clone(),
                ..Default::default()
            });
        }
    
    }
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
    Player::add_to_world(commands, "Adam");
}