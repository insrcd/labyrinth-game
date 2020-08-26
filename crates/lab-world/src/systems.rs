use bevy::{prelude::*, render::camera::Camera, type_registry::TypeRegistry};
use lab_entities::prelude::*;

use lab_entities::{player::Direction as Dir, Named};

use lab_sprites::{SpriteInfo, SpriteLibrary, StationaryLetter, TileAnimation};

use crate::{settings::*, *};
use lab_core::{Moveable, Zoomable};

#[derive(Debug)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Unknown,
}
// resource for current location
pub fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2, d: bool) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    if d {
        println!("a: {} {} b: {} {}", a_min, a_max, b_min, b_max);
    }
    // check to see if the two rectangles are intersecting
    if a_min.x() <= b_max.x()
        && a_max.x() >= b_min.x()
        && a_min.y() <= b_max.y()
        && a_max.y() >= b_min.y()
    {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) =
            if a_min.x() < b_min.x() && a_max.x() > b_min.x() && a_max.x() < b_max.x() {
                (Some(Collision::Left), b_min.x() - a_max.x())
            } else if a_min.x() > b_min.x() && a_min.x() < b_max.x() && a_max.x() > b_max.x() {
                (Some(Collision::Right), a_min.x() - b_max.x())
            } else {
                (None, 0.0)
            };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) =
            if a_min.y() < b_min.y() && a_max.y() > b_min.y() && a_max.y() < b_max.y() {
                (Some(Collision::Bottom), b_min.y() - a_max.y())
            } else if a_min.y() > b_min.y() && a_min.y() < b_max.y() && a_max.y() > b_max.y() {
                (Some(Collision::Top), a_min.y() - b_max.y())
            } else {
                (None, 0.0)
            };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        match (x_collision, y_collision) {
            (Some(x_collision), Some(y_collision)) => {
                if y_depth < x_depth {
                    Some(y_collision)
                } else {
                    Some(x_collision)
                }
            }
            (Some(x_collision), None) => Some(x_collision),
            (None, Some(y_collision)) => Some(y_collision),
            (None, None) => Some(Collision::Unknown),
        }
    } else {
        None
    }
}

pub fn camera_tracking_system(
    mut player_moved: Query<With<Player, (Entity, &mut Translation)>>,
    mut camera_query: Query<(&Camera, &mut Translation)>,
) {
    for (e, player_translation) in &mut player_moved.iter() {
        for (c, mut cam_trans) in &mut camera_query.iter() {
            if *(c.name.as_ref()).unwrap_or(&"".to_string()) != "UiCamera" {
                *cam_trans.0.x_mut() = player_translation.0.x();
                *cam_trans.0.y_mut() = player_translation.0.y();
            }
        }
    }
}
pub fn object_interaction_system(
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>,
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut placeable: Query<(
        Entity,
        &Interactable,
        &mut Translation,
        &Interaction,
        &mut TileAnimation,
    )>,
    mut moveables: Query<Without<Player, (&Moveable, &mut Translation, Mutated<Movement>)>>,
    mut player_moved: Query<With<Player, (Entity, &mut Translation, Mutated<Movement>)>>,
) {
    // may split out CD and interaction at some point.
}
/// Tile Interaction System - system which allows for tiles to change when they are interacted with
/// Also includes collision detection.
///
pub fn tile_interaction_system(
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>,
    mut text_update: ResMut<Events<TextChangeEvent>>,
    mut interaction_event: ResMut<Events<InteractionEvent>>,
    tile_palette: ResMut<TilePalette>,
    mut wall_query: Query<(
        Entity,
        &mut TileType,
        &mut TileAttributes,
        &mut Translation,
        &crate::Interaction,
        &mut SpriteInfo,
    )>,
    mut moveables: Query<
        Without<
            Player,
            (
                Entity,
                &Moveable,
                &mut Translation,
                Mutated<Movement>,
                &SpriteInfo,
                &Scale,
            ),
        >,
    >,
    mut player_moved: Query<
        With<
            Player,
            (
                Entity,
                &Scale,
                &mut Translation,
                Mutated<Movement>,
                &mut Inventory,
                &mut SpriteInfo,
            ),
        >,
    >,
) {
    for (tile_entity, _tt, tile_attributes, tile_translation, i, mut tile_sprite) in
        &mut wall_query.iter()
    {
        for (mov_entity, _m, mut move_transition, movement, move_sprite, scale) in
            &mut moveables.iter()
        {
            let collision = collide(
                move_transition.0,
                (move_sprite.size() * scale.0) - Vec2::new(16. * scale.0, 16. * scale.0),
                tile_translation.0,
                tile_sprite.size() * scale.0,
                false,
            );

            if let Some(collision) = collision {
                match collision {
                    _ => interaction_event.send(InteractionEvent {
                        source: mov_entity,
                        destination: tile_entity,
                        interaction_type: InteractionType::Collision,
                    }),
                }
            } else {
                //    player_collision = Some(move_transition.clone());
            }
        }
        for (e, scale, mut move_translation, movement, mut inventory, mut sprite) in
            &mut player_moved.iter()
        {
            let collision = collide(
                move_translation.0,
                Vec2::new(8. * scale.0, 8. * scale.0),
                tile_translation.0,
                tile_sprite.size() * scale.0,
                false,
            );

            if let Some(collision) = collision {
                match collision {
                    _ => interaction_event.send(InteractionEvent {
                        source: e,
                        destination: tile_entity,
                        interaction_type: InteractionType::Collision,
                    }),
                }
            }
        }
    }
}

pub fn interaction_system(
    mut commands: Commands,
    sprites: ResMut<SpriteLibrary>,
    interaction_events: ResMut<Events<InteractionEvent>>,
    mut text_update: ResMut<Events<TextChangeEvent>>,
    mut state: ResMut<InteractionState>,
    tile_palette: ResMut<TilePalette>,
    mut wall_query: Query<(
        Entity,
        &mut TileType,
        &mut TileAttributes,
        &mut Translation,
        &crate::Interaction,
        &mut SpriteInfo,
    )>,
    mut moveables: Query<
        Without<
            Player,
            (
                Entity,
                &Moveable,
                &mut Translation,
                &Movement,
                &SpriteInfo,
                &Scale,
            ),
        >,
    >,
    mut player_moved: Query<
        With<
            Player,
            (
                Entity,
                &Scale,
                &mut Translation,
                &Movement,
                &mut Inventory,
                &mut SpriteInfo,
            ),
        >,
    >,
) {
    for event in state.interaction_events.iter(&interaction_events) {
        match event.interaction_type {
            InteractionType::Collision => {
                if let Ok(src_move) = player_moved.get_mut::<Movement>(event.source) {
                    if let Ok(tile_interaction) =
                        wall_query.get_mut::<crate::Interaction>(event.destination)
                    {
                        let mut inventory =
                            player_moved.get_mut::<Inventory>(event.source).unwrap();
                        let mut move_translation =
                            player_moved.get_mut::<Translation>(event.source).unwrap();
                        let mut tile_sprite =
                            wall_query.get_mut::<SpriteInfo>(event.destination).unwrap();
                        let mut tile_attributes = wall_query
                            .get_mut::<TileAttributes>(event.destination)
                            .unwrap();
                        let tile_location =
                            wall_query.get::<Translation>(event.destination).unwrap();

                        for r in (tile_interaction.call)(InteractionContext {
                            interaction_location: Some(Location::from(*tile_location)),
                            inventory: Some(&mut inventory),
                            player: Some(event.source),
                            player_location: Some(src_move.0.into()),
                            tile_attributes: Some(&mut tile_attributes),
                            tile_palette: Some(&*tile_palette),
                            sprite_info: Some(&*tile_sprite),
                        }) {
                            match r {
                                InteractionResult::ChangeTile(attr) => {
                                    println!("Got change tile: {:?}", attr);

                                    //TODO: clean this up so a change to spriteinfo will change the tile
                                    tile_sprite.atlas_sprite = attr.sprite_idx.unwrap();

                                    commands.insert(
                                        event.destination,
                                        (
                                            attr,
                                            Location::from(*tile_location),
                                            attr,
                                            TextureAtlasSprite::new(attr.sprite_idx.unwrap()),
                                        ),
                                    );
                                }
                                InteractionResult::Damage(_) => {}
                                InteractionResult::ChangeSprite(_) => {}
                                InteractionResult::Move(_) => {}
                                InteractionResult::PickUp(_) => {
                                    commands.despawn(event.destination);
                                }
                                InteractionResult::Block => {
                                    *move_translation.0.x_mut() = (src_move.0).0;
                                    *move_translation.0.y_mut() = (src_move.0).1;
                                }
                                InteractionResult::None => {}
                                InteractionResult::Log(_) => {}
                                InteractionResult::Message(message) => {
                                    println!("Sending text change event {}", message);
                                    text_update.send(TextChangeEvent {
                                        text: message.to_string(),
                                        name: "main".to_string(),
                                    });
                                }
                                InteractionResult::Menu(_) => {}
                            };
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn save_world_system(world: &mut World, resources: &mut Resources) {
    /*let type_registry = resources.get::<TypeRegistry>();
    let input = resources.get::<Input<KeyCode>>();
    let scene = Scene::from_world(&world, &type_registry.component.read());

    use std::fs;

    // Scenes can be serialized like this:
    if input.just_pressed(KeyCode::F1) {
        let scene_ron = scene
        .serialize_ron(&type_registry.property.read())
        .unwrap();
        fs::write("scenes/saved.scn", scene_ron).expect("Unable to write file");
    }*/
}

/**
 * I tried using camera scale, but it doesn't seem to be able to zoom in when scale < 1
 */
pub fn zoom_system(
    mut scroll: ResMut<lab_input::ScrollState>,
    mut query: Query<(&mut Scale, &mut Translation, &Zoomable)>,
) {
    for (mut scale, mut trans, _tt) in &mut query.iter() {
        if scroll.y != 0. {
            // ease in the zoom by about .25 of the scroll intensity
            let ease: f32 = 0.25;

            let factor = (scroll.y.clone() * ease) + 1.;

            *scale = Scale(scale.0 * factor);

            *trans.x_mut() *= factor;
            *trans.y_mut() *= factor;

            scroll.current_scale = scale.0;
        }
    }
}
/// Super Basic right now, Move all NPCs in the scene every 1.5 seconds
pub fn npc_move_system(
    time: Res<Time>,
    mut query: Query<(
        &NonPlayer,
        &mut MoveTimer,
        &mut Translation,
        &mut Movement,
        &mut Moveable,
    )>,
) {
    for (_npc, mut timer, mut trans, mut m, _mm) in &mut query.iter() {
        timer.0.tick(time.delta_seconds);
        if timer.0.finished {
            let old_loc = Location::from(*trans);
            let direction = rand::random::<Dir>();

            match direction {
                Dir::Left => *trans.0.x_mut() -= WORLD_TILE_SIZE,
                Dir::Up => *trans.0.y_mut() += WORLD_TILE_SIZE,
                Dir::Down => *trans.0.y_mut() -= WORLD_TILE_SIZE,
                Dir::Right => *trans.0.x_mut() += WORLD_TILE_SIZE,
                Dir::Stationary => {}
            }

            *m = Movement(old_loc, Location::from(*trans), direction);

            timer.0.reset();
        }
    }
}

pub fn sprite_despawn_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Draw,
        &SpriteInfo,
        &lab_core::Despawn,
        &Timer,
        &mut Translation,
    )>,
) {
    for (e, sprite, s, _dspawn, timer, mut translation) in &mut query.iter() {
        if timer.finished {
            commands.despawn(e);
        }
    }
}

pub fn static_text_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Text, &mut Translation)>,
    mut player_query: Query<(Entity, &Player, Changed<Movement>)>,
) {
    for (e, _player, movement) in &mut player_query.iter() {
        for (e, _letter, mut translation) in &mut query.iter() {
            let old_loc = movement.0;
            let new_loc = movement.1;

            let x_change = old_loc.0 - new_loc.0;
            let y_change = old_loc.1 - new_loc.1;

            *translation.x_mut() -= x_change;
            *translation.y_mut() -= y_change;
        }
    }
}

pub fn update_ui_text_system(
    mut state: Local<UiTextState>,
    my_events: Res<Events<TextChangeEvent>>,
    mut text_element_query: Query<(&mut Text, &Named)>,
) {
    for event in state.change_events.iter(&my_events) {
        println!("Got text change event");
        for (mut t, n) in &mut text_element_query.iter() {
            if n.0 == event.name {
                (*t).value = event.text.clone();
            }
        }
    }
}
