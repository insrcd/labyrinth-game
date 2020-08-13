mod player;
mod world;
mod assets;

use bevy::{
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    sprite::collide_aabb::{collide, Collision},
    input::{keyboard::KeyCode, Input},
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
    .add_startup_system(load_world_sprites.system())
    .add_startup_system(simple_map.system())
    .add_system(keyboard_input_system.system())
    .add_system(collision_detection.system())
    .add_system(make_room.system())
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
    sprites : ResMut<assets::SpriteLibrary>,
    mut query: Query<(Added<Player>, &Named, &world::Location)>
) {
    for (player, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
        
        commands
        .spawn(SpriteSheetComponents {
            translation: Translation(Vec3::new(-300., loc.1, 100.)),
            scale: Scale(3.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        }).with(Player);
    }
}
// adds the sprites for the tiles
fn make_room (
    mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,
    texture_atlases: Res<Assets<TextureAtlas>>,    
    mut query: Query<(Added<Tile>, &Visible)>,
) {
    for (tile, vis) in &mut query.iter() {
        //println!("Found a tile named {:?} {}", tile.0, name.0);    

        let sprite = match tile.0 {
            TileType::Wall => sprites.get("wall"),
            _ => sprites.get("floor"),
        };

        let loc = &tile.1;

        commands
        .spawn(SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        }).with(tile.clone());
    
    }
}
// generate a simple map

fn simple_map(mut commands: Commands) {
    let comp = commands
        .spawn((AreaMap, Visible));

        let mut mb = MapBuilder::new(Vec2::new(96.,96.), Location(-450.,300.,0.));

        mb.add_tiles(RelativePosition::RightOf, 5, TileType::Wall);
        mb.add_tiles(RelativePosition::Below, 2, TileType::Wall);
        mb.add_tiles(RelativePosition::Below, 1, TileType::Floor);
        mb.add_tiles(RelativePosition::Below, 2, TileType::Wall);
        mb.add_tiles(RelativePosition::LeftOf, 5, TileType::Wall);
        mb.add_tiles(RelativePosition::Above, 5, TileType::Wall);
        mb.add_tiles_to_area(Location(-450.,300.,0.), Area(5., 5.), TileType::Floor);

        for n in mb.iter() {
            comp.with_children(|parent| {
                parent.spawn((Visible, n.clone()));                            
            });
        }
}

fn collision_detection(
    mut commands: Commands,
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut wall_query: Query<(&Tile, &mut Translation)>,
    mut pushable: Query<(&Pushable, &mut Translation)>,
    mut sprite_query: Query<(&Player, &mut Translation)>
) {
    
    for (cam, mut cam_trans) in &mut camera_query.iter(){
        for (player, mut player_translation) in &mut sprite_query.iter() {
            for (tile, mut tile_translation) in &mut wall_query.iter() {
                
                if tile.0 != TileType::Wall {
                    continue;
                }
                let collision = collide(player_translation.0, Vec2::new(32.,32.), tile_translation.0, Vec2::new(96.,96.0));
                if let Some(collision) = collision {
                    match collision {
                        Collision::Left => { 
                            *player_translation.0.x_mut() -= 32.;
                        }, 
                        Collision::Right => { 
                            *player_translation.0.x_mut() += 32.;  
                        },
                        Collision::Top => { 
                            *player_translation.0.y_mut() += 32.; 
                        },
                        Collision::Bottom => {
                            *player_translation.0.y_mut() -= 32.; 
                        },
                    }
                } else {       
                    *cam_trans.0.x_mut() = player_translation.0.x();             
                    *cam_trans.0.y_mut() = player_translation.0.y();
                }
            }
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut query: Query<(&Player, &mut Translation)>) {
    for (player, mut loc) in &mut query.iter() {   
        if keyboard_input.just_pressed(KeyCode::W) {
            *loc.0.y_mut() += 32.;
            //*cam_trans.0.y_mut() += 32.;
        }

        if keyboard_input.just_pressed(KeyCode::S) {
            *loc.0.y_mut() -= 32.;
            //*cam_trans.0.y_mut() -= 32.;
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            *loc.0.x_mut() -= 32.;
            //*cam_trans.0.x_mut() -= 32.;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            *loc.0.x_mut() += 32.;
            //*cam_trans.0.x_mut() += 32.;
        }
    }      

    for (camera, mut cam_trans) in &mut camera_query.iter() {    
        if keyboard_input.just_pressed(KeyCode::W) {
            //*loc.0.y_mut() += 32.;
            *cam_trans.0.y_mut() += 32.;
        }

        if keyboard_input.just_pressed(KeyCode::S) {
            //*loc.0.y_mut() -= 32.;
            *cam_trans.0.y_mut() -= 32.;
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            //*loc.0.x_mut() -= 32.;
            *cam_trans.0.x_mut() -= 32.;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            //*loc.0.x_mut() += 32.;
            *cam_trans.0.x_mut() += 32.;
        }
    }

}

fn simple_movement() {

}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    
    commands.spawn(Camera2dComponents::default())
    .spawn(UiCameraComponents::default());
    Player::add_to_world(commands, "Adam");
}