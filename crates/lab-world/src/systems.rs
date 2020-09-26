use bevy::sprite::collide_aabb::*;

use lab_entities::prelude::*;
use lab_core::prelude::*;
use lab_sprites::SpriteInfo;
use crate::{TextChangeEvent, TileComponents, InteractionState, UiTextState, TileInteractionResult, TileInteraction, TileInteractionResultEvent};

pub fn camera_tracking_system(
    mut player_moved: Query<With<Player, (Entity, Mutated<Transform>)>>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    for (_e, player_translation) in &mut player_moved.iter() {
        for (c, mut cam_trans) in &mut camera_query.iter() {
            if *(c.name.as_ref()).unwrap_or(&"".to_string()) != "UiCamera" {
                *cam_trans.translation_mut().x_mut() = player_translation.translation().x();
                *cam_trans.translation_mut().y_mut() = player_translation.translation().y();
            }
        }
    }
}
/// Collision System
/// This system sends an event whenever two Interactables touch.
pub fn collision_system(
    mut interaction_event: ResMut<Events<InteractionEvent>>,
    mut wall_query: Query<(
        Entity,
        &Transform,
        &SpriteInfo,
        &InteractableType
    )>,
    mut moveables: Query<(Entity,Mutated<Transform>)>)
{
    for (mov_entity, move_translation) in
        &mut moveables.iter()
    {
            //println!("checking colision move: {:?}", *move_translation);
            for (destination_entity, tile_translation, tile_sprite, interact_type) in
            &mut wall_query.iter()
        {
            if *interact_type == InteractableType::None {
                continue
            }
            if mov_entity == destination_entity {
                continue
            }
            
            
            //println!("checking colision tile: {:?}", tile_translation);
            let collision = collide(
                move_translation.translation(),
                Vec2::new(8., 16.) * move_translation.scale().truncate(),
                tile_translation.translation(),
                tile_sprite.size() * tile_translation.scale().truncate(),
            );

            if let Some(collision) = collision {
                //println!("Collision detected");
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

/// Interaction System
/// This sytem will try and see if there is a registered Interaction handler for a 
/// object that has experienced a collision. If there is, it is ran
/// and the result is sent as an event to be processed.

pub fn interaction_system(
    items : ResMut<Items>,
    mut result_events : ResMut<Events<TileInteractionResultEvent>>,
    mut state: ResMut<InteractionState>,
    interaction_events: ResMut<Events<InteractionEvent>>,
    world_catalog: Res<InteractionCatalog<TileInteraction, TileComponents, Vec<TileInteractionResult>>>,
    item_query: Query<(Entity, &ItemType, &Named)>,
    interactable_query: Query<(
        Entity,
        &InteractableType,
        &Named,
        &ObjectState,
        &WorldHandle<TileInteraction>,
        &WorldHandle<Tile>,
        &Inventory
    )>
) {
    for event in state.interaction_events.iter(&interaction_events) {        
        match event.interaction_type {
            InteractionType::Collision => {
                if event.source == event.destination {
                    panic!("A entity collided with itself, this should not happen")
                }
               
                let tile_handle_r =  interactable_query.get::<WorldHandle<TileInteraction>>(event.destination);

                if let Some(tile_handle) = tile_handle_r.ok() {
                    
                    //println!("{:?} interacted with {:?} name: {:?}", event.source, event.destination, tile_handle);
                    
                    if let Some(tile_interaction) =
                            world_catalog.get_interaction(*tile_handle)
                    {   
                        let ctx = InteractionContext {
                            source: event.source,
                            destination: event.destination,
                            world_catalog:world_catalog.clone(),
                            interaction_query: &interactable_query,
                            item_query: &item_query,
                            items: &items
                        };
                        for r in tile_interaction.interact(ctx).iter() {
                            result_events.send(TileInteractionResultEvent { 
                                _source: event.source, 
                                destination: event.destination, 
                                result: r.clone() 
                            })
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn process_interaction_result_system (
    mut commands : Commands,
    interaction_events : ResMut<Events<TileInteractionResultEvent>>,
    mut items : ResMut<Items>,
    mut state: ResMut<InteractionState>,
    mut text_update: ResMut<Events<TextChangeEvent>>,
    _tile_query: Query<
        (
            Entity,
            &Draw
        ),
    >,
    entity_query: Query<
        (
            Entity,         
            &mut Transform,
            &mut Movement,
            &mut Inventory,
            &mut Draw
        ),
    >){
    for event in state.interaction_results.iter(&interaction_events) {  
        match event.result.clone() {
            TileInteractionResult::ChangeSprite(entity, sprite_info) => {
                commands.insert(
                    entity,
                    (
                        TextureAtlasSprite::new(sprite_info.atlas_sprite),
                    ),
                );
            }
            TileInteractionResult::Damage(_src, dst, amount) => {
                if let Ok(mut state) = entity_query.get_mut::<ObjectState>(dst) {

                    let hp : i32 = state.get("hitpoints".into()).unwrap().into();

                    state.set_int("hitpoints".into(), hp - amount as i32);
                }
            }
            TileInteractionResult::ChangeInventory(entity, inv) => {
                if let Ok(mut inventory) = entity_query.get_mut::<Inventory>(entity) {
                    inventory.0 = inv.0.clone();
                }
            },
            TileInteractionResult::ChangeState(entity, state) => {
                // commit state changes in this 
                if let Ok(mut dstate) = entity_query.get_mut::<ObjectState>(entity) {
                    dstate.values = state.values;
                }
            }
            TileInteractionResult::Move(entity, location) => {
                if let Ok(mut new_location) = entity_query.get_mut::<Transform>(entity) {
                    *new_location.translation_mut().x_mut() = location.0;
                    *new_location.translation_mut().y_mut() = location.1;
                }
            }
            TileInteractionResult::Despawn => {
                commands.remove_one::<Draw>(event.destination);
                commands.remove_one::<Transform>(event.destination);
                
            }
            TileInteractionResult::Block(entity) => {
                println!("Got block");
                if let Ok(mut translation) = entity_query.get_mut::<Transform>(entity) {
                    if let Ok(src_move) = entity_query.get::<Movement>(entity) {
                        *translation.translation_mut().x_mut() = src_move.start.x();
                        *translation.translation_mut().y_mut() = src_move.start.y();
                    }
                }
            }
            TileInteractionResult::None => {}
            TileInteractionResult::Log(_) => {}
            TileInteractionResult::Message(message) => {
                text_update.send(TextChangeEvent {
                    text: message.to_string(),
                    name: "main".to_string(),
                });
            }
            TileInteractionResult::Menu(_) => {},
            
            TileInteractionResult::AddItem(dst, item) => {
            
                // Add an item to the destination entities inventory                
                if let Ok(mut inventory) = entity_query.get_mut::<Inventory>(dst) {
                    inventory.0.push(item.handle.clone());

                    let handle = item.handle;

                    commands
                        .spawn( item)
                        .for_current_entity(|e| {
                            // register the entity with the items resource.
                            items.items.insert(handle, e);
                         } );                                        
                }
            }
        };
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
    mut query: Query<(Entity,&mut Transform, &Zoomable)>,
    movement_query: Query<(Entity,&mut Movement)>
) {
    
    let _window = windows.iter().last().unwrap();

    let mut entities_changed : Vec<(Entity, Location, Location)> = Vec::new();
    
    for (entity, mut trans, _tt) in &mut query.iter() {
        if scroll.y != 0. {
            
            // ease in the zoom by about .25 of the scroll intensity
            let ease: f32 = 0.25;

            let factor = (scroll.y.clone() * ease) + 1.;

            let translation_before = trans.clone();

            if trans.scale().x() * factor > 6. {
                return;
            }

            let y_scale = trans.scale().x();

            trans.set_scale(y_scale * factor);

            *trans.translation_mut().x_mut() *= factor;
            *trans.translation_mut().y_mut() *= factor;

            entities_changed.push((entity, translation_before.into(), (*trans).into()));

            scroll.current_scale = trans.scale().x();
        }
    }


    for (e,l1,l2) in entities_changed {
        match &mut movement_query.get_mut::<Movement>(e) {
            Ok(movement) => {
                //println!("Setting movement for {:?}", e.0);
                movement.start = l1.into();
                movement.end = l2.into();
                movement.direction = Vec3::zero();                    
            }
            Err(_err) => {                    
                // this will happen if the entity doesn't have a movement
            }
        }
    }
    /*
    for (mut _entity, _style, mut _zoom, mut _lt, mut trans, dialog, mut text) in &mut text_query.iter() {
        if scroll.y != 0. {            
            
            if let Ok(tl) = query.get::<Transform>(dialog.entity) {                
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
        &mut Transform,
    )>,
) {
    for (e, _, timer, _translation) in &mut query.iter() {
        if timer.finished {
            commands.despawn(e);
        }
    }
}

pub fn static_text_system(
    mut query: Query<(Entity, &Text, &mut Transform, &StaticLocation)>,
    mut player_query: Query<(Entity, &Player, &Movement, Changed<Transform>)>,
) {
    for (_e, _player, movement, _) in &mut player_query.iter() {
        for (_e, _letter, mut translation, _st) in &mut query.iter() {            
            let change : Vec2 = movement.start.truncate() - movement.end.truncate();

            // make sure there actually was a movement change in translation
            if Vec2::new(0.,0.) != change {
                *translation.translation_mut().x_mut() -= change.x();
                *translation.translation_mut().y_mut() -= change.y();
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
