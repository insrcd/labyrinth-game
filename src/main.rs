mod player;
mod world;
mod assets;
mod scripting;

use bevy::{
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    sprite::collide_aabb::{collide, Collision},
    input::{keyboard::KeyCode, Input}, type_registry::TypeRegistry,
};

use assets::*;
use player::*;
use world::*;
use scripting::*;

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
    .add_startup_system(save_world.thread_local_system())
    
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
    for (_player, name , loc) in &mut query.iter() {
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
    mut query: Query<(Entity, Added<Tile>, &Visible, &Location)>,
    mut p_query: Query<(Entity, Added<Pushable>, &Visible, &Location)>,
) {
    for (e, push, vis, &loc) in &mut p_query.iter() {
        let sprite = sprites.get("chair");
        
        commands.insert(e, SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        });
    }
    for (e, tile, vis, loc) in &mut query.iter() {
        println!("Adding a tile entity {:?} {:?}", tile.0, loc);    

        let sprite = match tile.0 {
            TileType::Wall => sprites.get("wall"),
            _ => sprites.get("floor"),
        };

        commands.insert(e, SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        });
    
    }
}
// generate a simple map

fn simple_map(mut commands: Commands) {

    let mut mb = MapBuilder::new(Vec2::new(96.,96.), Location(-450.,300.,0.));

    mb.add_tiles(RelativePosition::RightOf, 5, TileType::Wall);
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall);
    mb.add_tiles(RelativePosition::Below, 1, TileType::Floor);
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall);
    mb.add_tiles(RelativePosition::LeftOf, 5, TileType::Wall);
    mb.add_tiles(RelativePosition::Above, 5, TileType::Wall);

    mb.add_tiles_to_area(Location(-450.,300.,0.), Area(5., 5.), TileType::Floor);

    for (tile, location) in mb.iter() {
        commands.spawn((Visible, tile.clone(), location.clone()));
    }

    commands.spawn((Pushable, Location(-450. + 96.*2.,300. - 96.*2.,50.), Visible));
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
            for (push, mut push_translation) in &mut pushable.iter() {             
                let collision = collide(player_translation.0, Vec2::new(32.,32.), push_translation.0, Vec2::new(96.,96.0));
                if let Some(collision) = collision {
                    match collision {
                        Collision::Left => { 
                            *push_translation.0.x_mut() += 32.;
                        }, 
                        Collision::Right => { 
                            *push_translation.0.x_mut() -= 32.;  
                        },
                        Collision::Top => { 
                            *push_translation.0.y_mut() -= 32.; 
                        },
                        Collision::Bottom => {
                            *push_translation.0.y_mut() += 32.; 
                        },
                    }
                } 
            }
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

    for (push, mut push_translation) in &mut pushable.iter() {         
        for (tile, mut tile_translation) in &mut wall_query.iter() {
                
            if tile.0 != TileType::Wall {
                continue;
            }    
            let collision = collide(tile_translation.0, Vec2::new(96.,96.), push_translation.0, Vec2::new(96.,96.0));
            if let Some(collision) = collision {
                match collision {
                    Collision::Left => { 
                        *push_translation.0.x_mut() += 32.;
                    }, 
                    Collision::Right => { 
                        *push_translation.0.x_mut() -= 32.;  
                    },
                    Collision::Top => { 
                        *push_translation.0.y_mut() -= 32.; 
                    },
                    Collision::Bottom => {
                        *push_translation.0.y_mut() += 32.; 
                    },
                }
            } 
        }
    }
}

fn save_world(world: &mut World, resources: &mut Resources) {
    let type_registry = resources.get::<TypeRegistry>().unwrap();
    let input = resources.get::<Input<KeyCode>>().unwrap();
    let scene = Scene::from_world(&world, &type_registry.component.read().unwrap());

    // Scenes can be serialized like this:
    if input.just_pressed(KeyCode::F1) {
        println!(
            "{}",
            scene
                .serialize_ron(&type_registry.property.read().unwrap())
                .unwrap()
        );
    }
}

fn keyboard_input_system(
    type_registry: Res<TypeRegistry>,
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
        if keyboard_input.just_pressed(KeyCode::Up) {
            //*loc.0.y_mut() += 32.;
            *cam_trans.0.y_mut() += 32.;
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            //*loc.0.y_mut() -= 32.;
            *cam_trans.0.y_mut() -= 32.;    
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            //*loc.0.x_mut() -= 32.;
            *cam_trans.0.x_mut() -= 32.;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            //*loc.0.x_mut() += 32.;
            *cam_trans.0.x_mut() += 32.;
        }
    }

}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
    Player::add_to_world(commands, "Adam");
}