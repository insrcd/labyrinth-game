use bevy::sprite::collide_aabb::*;

use lab_entities::prelude::*;
use lab_core::prelude::*;
use lab_sprites::SpriteInfo;
use crate::{TextChangeEvent, TileComponents, InteractionState, UiTextState, TileInteractionResult, TileInteraction};
use std::{rc::Rc, borrow::Cow, sync::Arc};

pub fn camera_tracking_system(
    mut player_moved: Query<With<Player, (Entity, Mutated<Translation>)>>,
    mut camera_query: Query<(&Camera, &mut Translation)>,
) {
    for (_e, player_translation) in &mut player_moved.iter() {
        for (c, mut cam_trans) in &mut camera_query.iter() {
            if *(c.name.as_ref()).unwrap_or(&"".to_string()) != "UiCamera" {
                *cam_trans.0.x_mut() = player_translation.0.x();
                *cam_trans.0.y_mut() = player_translation.0.y();
            }
        }
    }
}
/// object Interaction System - system which allows for tiles to change when they are interacted with
/// Also includes collision detection.
///
pub fn collision_system(
    mut interaction_event: ResMut<Events<InteractionEvent>>,
    mut wall_query: Query<(
        Entity,
        &Translation,
        &SpriteInfo,
        &InteractableType
    )>,
    mut moveables: Query<(Entity,Mutated<Translation>,&Scale)>)
{
    for (mov_entity, move_translation, scale) in
        &mut moveables.iter()
    {
            for (destination_entity, tile_translation, tile_sprite, interact_type) in
            &mut wall_query.iter()
        {
            if *interact_type == InteractableType::None {
                continue
            }
            if mov_entity == destination_entity {
                continue
            }
            let collision = collide(
                move_translation.0,
                Vec2::new(8. * scale.0, 8. * scale.0),
                tile_translation.0,
                tile_sprite.size() * scale.0,
            );

            if let Some(collision) = collision {
                println!("Collision detected");
                match collision {
                    _ => interaction_event.send(InteractionEvent {
                        source: mov_entity,
                        destination: destination_entity,
                        interaction_type: InteractionType::Collision
                    }),
                }
            } 
        }
    }
}

pub fn interaction_system(
    mut commands: Commands,
    mut state: ResMut<InteractionState>,
    interaction_events: ResMut<Events<InteractionEvent>>,
    mut text_update: ResMut<Events<TextChangeEvent>>,    
    world_catalog: Res<InteractionCatalog<TileInteraction, TileComponents, Vec<TileInteractionResult>>>,
    interactable_query: Query<(
        Entity,
        &mut ObjectState,
        &mut Translation,
        &mut SpriteInfo,
        &mut Inventory,
        &InteractableType,
        &Named        
    )>,
    entity_query: Query<
        (
            Entity,
            &Scale,
            &mut Translation,
            &Movement,
            &mut Inventory,
            &mut SpriteInfo,
            &mut ObjectState
        ),
    >,
) {
    for event in state.interaction_events.iter(&interaction_events) {        
        match event.interaction_type {
            InteractionType::Collision => {
                if event.source == event.destination {
                    panic!("A entity collided with itself, this should not happen")
                }
                let interaction_name =  interactable_query.get::<Named>(event.source).expect("Entity invovled in an interaction without a name");
                let source_type = interactable_query.get::<InteractableType>(event.source).expect("Source entity without an interaction type");
                let dst_type = interactable_query.get::<InteractableType>(event.destination).expect("Destination entity without an interaction type");
                let src_state = interactable_query.get_mut::<ObjectState>(event.source).expect("Source entity without an state");
                let dst_state = interactable_query.get_mut::<ObjectState>(event.destination).expect("Destination entity without a state");
                let src_trans = interactable_query.get::<Translation>(event.source).expect("Source entity without an state");
                let dst_trans = interactable_query.get::<Translation>(event.destination).expect("Destination entity without a state");

                // collision implies movement
                if let Ok(src_move) = entity_query.get_mut::<Movement>(event.source) {
                    if let Some(tile_interaction) =
                        world_catalog.get_interaction(&interaction_name.0)
                    {
                        let mut inventory =
                            entity_query.get_mut::<Inventory>(event.source).ok();
                        let mut move_translation =
                            entity_query.get_mut::<Translation>(event.source).unwrap();
                        let mut dst_sprite =
                            entity_query.get_mut::<SpriteInfo>(event.destination).unwrap();
                        let mut dst_inventory =  entity_query.get_mut::<Inventory>(event.destination).ok();

                        for r in tile_interaction.interact(InteractionContext {
                            source: &Interactable { 
                                entity: event.source, 
                                interactable_type: *source_type, 
                                location: src_move.start.into(),
                                inventory: inventory,
                                tile_state: Some(src_state.clone())
                            },
                            destination: &Interactable { 
                                entity: event.source, 
                                interactable_type: *dst_type, 
                                location: (*dst_trans).into(),
                                inventory: dst_inventory,
                                tile_state: Some(dst_state.clone())
                            },
                            world_catalog:world_catalog.clone()
                        }).iter() {
                            match r {
                                TileInteractionResult::ChangeSprite(sprite_info) => {
                                    commands.insert(
                                        event.destination,
                                        (
                                            TextureAtlasSprite::new(sprite_info.atlas_sprite),
                                        ),
                                    );
                                }
                                TileInteractionResult::Damage(_) => {}
                                TileInteractionResult::ChangeSprite(_) => {}
                                TileInteractionResult::ChangeState(_) => {
                                    // commit state changes in this comp
                                }
                                TileInteractionResult::Move(_) => {}
                                TileInteractionResult::Despawn => {
                                    commands.despawn(event.destination);
                                }
                                TileInteractionResult::Block => {
                                    *move_translation.0.x_mut() = src_move.start.0;
                                    *move_translation.0.y_mut() = src_move.start.1;
                                }
                                TileInteractionResult::None => {}
                                TileInteractionResult::Log(_) => {}
                                TileInteractionResult::Message(message) => {
                                    text_update.send(TextChangeEvent {
                                        text: message.to_string(),
                                        name: "main".to_string(),
                                    });
                                }
                                TileInteractionResult::Menu(_) => {}
                            };
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

// TODO Re-implement
pub fn save_world_system(_world: &mut World, _resources: &mut Resources) {
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
 * I tried using camera scale, but it doesn't seem to be able to zoom in when scale < 1. This seems to be a bug 
 * 
 */
pub fn zoom_system(
    windows: ResMut<Windows>,
    mut scroll: ResMut<lab_input::ScrollState>,
    mut query: Query<(Entity,&mut Scale, &mut Translation, &Zoomable)>,
    movement_query: Query<(Entity,&mut Movement)>
) {
    
    let window = windows.iter().last().unwrap();

    let mut entities_changed : Vec<(Entity, Location, Location)> = Vec::new();
    
    for (entity, mut scale, mut trans, _tt) in &mut query.iter() {
        if scroll.y != 0. {
            
            // ease in the zoom by about .25 of the scroll intensity
            let ease: f32 = 0.25;

            let factor = (scroll.y.clone() * ease) + 1.;

            let translation_before = trans.clone();

            if scale.0 * factor > 6. {
                return;
            }

            *scale = Scale(scale.0 * factor);

            *trans.x_mut() *= factor;
            *trans.y_mut() *= factor;

            entities_changed.push((entity, translation_before.into(), (*trans).into()));

            scroll.current_scale = scale.0;
        }
    }


    for e in entities_changed {
        match &mut movement_query.get_mut::<Movement>(e.0) {
            Ok(movement) => {
                //println!("Setting movement for {:?}", e.0);
                movement.start = e.1;
                movement.end = e.2;
                movement.direction = CardinalDirection::None;                    
            }
            Err(_err) => {                    
                // this will happen if the entity doesn't have a movement
            }
        }
    }
    /*
    for (mut _entity, _style, mut _zoom, mut _lt, mut trans, dialog, mut text) in &mut text_query.iter() {
        if scroll.y != 0. {            
            
            if let Ok(tl) = query.get::<Translation>(dialog.entity) {                
                let sprite_info = query.get::<SpriteInfo>(dialog.entity).unwrap();
                
                let sprite_scaled_size = sprite_info.scaled_size(scroll.current_scale);        
                let x = tl.x() + (window.width/2) as f32+sprite_scaled_size.x();
                let y = tl.y() + (window.height/2) as f32 + (sprite_scaled_size.y() * 0.8);

                *trans.x_mut() = x;
                *trans.y_mut() = y;

                (*text).style.font_size = 10. * scroll.current_scale 
            }
        }
    }*/
}

pub fn sprite_despawn_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &lab_core::Despawn,
        &Timer,
        &mut Translation,
    )>,
) {
    for (e, _, timer, _translation) in &mut query.iter() {
        if timer.finished {
            commands.despawn(e);
        }
    }
}

pub fn static_text_system(
    mut query: Query<(Entity, &Text, &mut Translation, &StaticLocation)>,
    mut player_query: Query<(Entity, &Player, &Movement, Changed<Translation>)>,
) {
    for (_e, _player, movement, t) in &mut player_query.iter() {
        for (_e, _letter, mut translation, _st) in &mut query.iter() {            
            let change : Vec2 = Vec2::from(movement.start) - Vec2::from(movement.end);

            // make sure there actually was a movement change in translation
            if Vec2::new(0.,0.) != change {
                *translation.x_mut() -= change.x();
                *translation.y_mut() -= change.y();
            }
        }
    }
}

pub fn add_text_to_adventure_log(
    mut state: Local<UiTextState>,
    mut adventure_log: ResMut<AdventureLog>,
    my_events: Res<Events<TextChangeEvent>>,
    mut text_element_query: Query<(&mut Text, &lab_core::Named)>
) {
    for event in state.change_events.iter(&my_events) {
        adventure_log.add_message(event.text.clone());

        for (mut line, n) in &mut text_element_query.iter() {
            if n.0.starts_with("log_line_") {
               let line_comp_id :usize = n.0.split("_").into_iter().last().unwrap().parse().unwrap();
               if let Some(log) = adventure_log.last(line_comp_id) {
                   line.value = log.to_string();
               }
            }
        }
    
    }
}
