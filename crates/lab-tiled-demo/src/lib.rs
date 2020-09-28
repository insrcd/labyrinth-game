use bevy::prelude::*;
use bevy_tiled::{Map};
use lab_core::{InteractableType, Inventory, ObjectState};
use lab_world::{TileInteraction, TileInteractionResult};
use std::collections::{HashMap, HashSet};
use tiled::Object;
pub struct TiledDemoPlugin;

/// Load a demo that displays the basic functionality of the
/// Game framework. The demo uses the basic map builder
impl Plugin for TiledDemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_tiled::TiledMapPlugin)
        .init_resource::<Objects>()
        .add_startup_stage_before("startup", "register")
        .add_startup_stage_after("register", "create_map")
        .add_startup_system_to_stage("create_map", setup.system())
        .add_system(register_objects.system());
    }
}

#[derive(Default)]
pub struct Objects {
    registry: HashMap<String, Object> 
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(bevy_tiled::TiledMapComponents {
            map_asset: asset_server.load("resources/maps/Lab_16_2.tmx").unwrap(),
            center: false,
            origin: Transform::from_scale(3.),
            ..Default::default()
        })
        .spawn(Camera2dComponents::default());
}
#[derive(Default)]
pub struct MapResourceProviderState {
    map_event_reader: EventReader<AssetEvent<Map>>,
}

#[derive(Bundle)]
pub struct ObjectComponents {
    object : Object,
    transform : Transform,
    interaction_type: InteractableType,
    interaction: TileInteraction,
    size: Size,
    inventory : Inventory,
    object_state: ObjectState
}
fn register_objects(mut commands : Commands,
    mut objects: ResMut<Objects>,
    mut state: Local<MapResourceProviderState>,
    mut maps: ResMut<Assets<Map>>,
    map_events: Res<Events<AssetEvent<Map>>>,
    mut query : Query<&Handle<Map>>){
    
        let mut changed_maps = HashSet::<Handle<Map>>::new();
    for event in state.map_event_reader.iter(&map_events) {
        match event {
            AssetEvent::Created { handle } => {
                changed_maps.insert(*handle);
            }
            AssetEvent::Modified { handle } => {
                //changed_maps.insert(*handle);
            }
            AssetEvent::Removed { handle } => {
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.remove(handle);
            }
        }
    }
    for handle in &mut changed_maps.iter() {
        println!("Got map handle");
        if let Some(map) = maps.get(handle) {
            println!("Got map");
            for og in &map.map.object_groups {
                println!("Objects: {:?}", og);
                for object in &og.objects {
                    commands
                        .spawn(ObjectComponents {
                            interaction: TileInteraction {
                                caller: |ctx| TileInteractionResult::Block(ctx.source).into(),
                                description: "Bump",
                            },
                            object: object.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                object.x, 
                                -object.y, 
                                og.layer_index.unwrap_or_else(|| 0) as f32)),
                            interaction_type: InteractableType::Tile,
                            size: Size::new(object.width, object.height),
                            inventory: Inventory::default(),
                            object_state: ObjectState::default()
                        }).for_current_entity(|e| {
                            println!("Created object entity {:?}", e);
                        });
                }
            }
        }
    }
}