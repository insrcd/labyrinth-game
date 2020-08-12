mod player;
mod world;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

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
    player: &player::Player,
    name: &player::Named
) {
    println!("{} {}", player, name.0)
}


fn load_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) { 
    let texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/sprites/world.png",
    )
    .unwrap();
    let player_texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/sprites/gabe-idle-run.png",
    )
    .unwrap();

    let texture = textures.get(&texture_handle).unwrap();
    let player_texture = textures.get(&player_texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 4);
    
    let player_texture_atlas = TextureAtlas::from_grid(player_texture_handle, player_texture.size, 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);

    let mut sprite_lib = world::SpriteLibrary::new();

    sprite_lib.add("wall", world::Sprite::new("wall", 1, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add("floor", world::Sprite::new("floor", 2, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add("player", world::Sprite::new("player", 0, player_texture_atlas_handle.clone(), player_texture.size.x() as u32, player_texture.size.y() as u32));

    commands
    .insert_resource(sprite_lib)
    .spawn(Camera2dComponents::default())
    .spawn(UiCameraComponents::default())
    .spawn((player::Player, player::Named("Adam".to_string()), world::Location(0., 0., 0.)));
/*    .spawn(SpriteSheetComponents {
        translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
        scale: Scale(6.0),
        sprite: TextureAtlasSprite::new(1 as u32),
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    }).with(world::Tile(world::TileType::Wall)).with(player::Named("wall".to_string()))
    .spawn(SpriteSheetComponents {
        translation: Translation(Vec3::new(16.0*6., 16.0*6., 0.0)),
        scale: Scale(6.0),
        sprite: TextureAtlasSprite::new(2 as u32),
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    }).with(world::Tile(world::TileType::Floor)).with(player::Named("rock".to_string()));*/

    /*
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            translation:  Translation(Vec3::new(300.0, 100.0, 0.0)),
            ..Default::default()
        });*/
}

fn add_player(mut commands: Commands,
    sprites : ResMut<world::SpriteLibrary>,
    mut query: Query<(Added<player::Player>, &player::Named, &world::Location)>
) {
    for (tile, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        println!("got sprite {}", sprite.name);
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
    mut query: Query<(Added<Tile>, &player::Named, &mut TextureAtlasSprite,  &Handle<TextureAtlas>)>,
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
    player::Player::add_to_world(commands, "Adam");
}