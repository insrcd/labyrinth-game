use lab_input::ScrollState;
use lab_sprites::SpriteInfo;
use lab_core::prelude::*;
use lab_entities::prelude::*;

use crate::{ButtonMaterials, UiState};

pub fn ui_key_system (
  mut commands : Commands,
  keyboard_input: Res<Input<KeyCode>>,
  mut query : Query<(Entity, &UiState)>
){

  if keyboard_input.just_pressed(KeyCode::I) {
    for (e, state) in &mut query.iter(){
      println!("{:?}", e);
      if let UiState::Inventory = state { 
        
        println!("Changing to main");
        commands.remove_one::<UiState>(e);
        commands.insert_one(e, UiState::Main);
      } else {
        println!("Changing to inventory");
        
        commands.remove_one::<UiState>(e);
        commands.insert_one(e, UiState::Inventory);
      }
    }
  }
}

pub fn button_system(
  button_materials: Res<ButtonMaterials>,
  mut interaction_query: Query<(
      &Button,
      Mutated<Interaction>,
      &mut Handle<ColorMaterial>
  )>,
  text_query: Query<&mut Text>,
) {
  for (_button, interaction, mut material) in &mut interaction_query.iter() {
     //let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
     
      match *interaction {
          Interaction::Clicked => {
             // text.value = "Press".to_string();
              *material = button_materials.pressed;
          }
          Interaction::Hovered => {
             // text.value = "Hover".to_string();
              *material = button_materials.hovered;
          }
          Interaction::None => {
             // text.value = "Button".to_string();
              *material = button_materials.normal;
          }
      }
  }
}

pub struct InventoryUi;

impl InventoryUi {
  fn get_main(materials : Handle<ColorMaterial>) -> NodeComponents {
    NodeComponents {
      style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

          ..Default::default()
      },
      material: materials,
      ..Default::default()
    }
  }
  fn get_item_box (parent : &mut ChildBuilder, materials : Handle<ColorMaterial>,  f: impl FnMut(&mut ChildBuilder)) {
    parent.spawn(NodeComponents {
      style: Style {
          align_self: AlignSelf::FlexEnd,
          justify_content: JustifyContent::SpaceEvenly,
          size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
          align_items: AlignItems::Stretch,
          flex_wrap: FlexWrap::WrapReverse,
          direction: Direction::LTR,
          ..Default::default()
      },
      material: materials,
      ..Default::default()
    }).with_children(f);
  }
  fn make_item(parent : &mut ChildBuilder, sprite: &SpriteInfo, 
       text: String, 
       font_handle: Handle<Font>,
       material: Handle<ColorMaterial>) {
    parent.spawn(NodeComponents {
      style: Style {
          size: Size::new(Val::Px(100.0), Val::Px(100.0)),
          justify_content: JustifyContent::Center,
          align_content: AlignContent::Center,
          margin: Rect::all(Val::Px(10.0)),
          ..Default::default()
      },
      material: material,
      ..Default::default()
    }).with_children(|p2| {
      p2.spawn(NodeComponents {
        style: Style {
          position_type: PositionType::Absolute,
          position: Rect { 
            top: Val::Px(0.0),
            ..Default::default()
          },
          size: Size::new(Val::Px(100.0), Val::Px(80.0)),
          ..Default::default()
        },
        ..Default::default()
      }).with_bundle(sprite.to_node_components());
      p2.spawn(TextComponents {
        style: Style {
          //size: Size::new(Val::Px(100.0), Val::Px(100.0)),
          //justify_content:JustifyContent::Center,
          margin: Rect { 
            top: Val::Px(80.0),
            ..Default::default()
          },
          align_self: AlignSelf::Center,
          ..Default::default()
        },
        text: Text {
            value: text.clone(),
            font: font_handle,
            style: TextStyle {
                font_size: 13.0,
                color: Color::WHITE,
            },
        },
        draw: Draw {is_visible: true, ..Default::default()},
        ..Default::default()
      });
    });
  }
}

pub fn inventory_ui_system (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut assets: ResMut<Assets<Font>>,
    button_materials: Res<ButtonMaterials>,
    mut state_query: Query<(Entity, Changed<UiState>)>,
    item_query: Query<(Entity, &WorldHandle<Item>, &Named)>,
    tile_query: Query<(&WorldHandle<Tile>, &SpriteInfo)>,
    mut ui_query: Query<(Entity, &InventoryUi)>,
    mut inventory_query: Query<(Entity, &Player, &mut Inventory)>
) {
  let font_handle = asset_server.load_sync(&mut assets, "resources/fonts/FiraSans-Bold.ttf").unwrap();

  for (_entity, state_change) in &mut state_query.iter() {
    for  (_e, _player, inv)  in &mut inventory_query.iter(){
      if let UiState::Inventory = *state_change {
        commands
        .spawn(InventoryUi::get_main(button_materials.normal))
        .with_children(|parent| {
          InventoryUi::get_item_box(parent, materials.add(Color::rgb(0.2, 0.2, 1.0).into()), 
          |p2| {
            for item in inv.0.iter(){
              // all items have a tile
              let tile_handle = item_query.get::<WorldHandle<Tile>>((*item).entity).unwrap();
              // and are named
              let name = item_query.get::<Named>((*item).entity).unwrap();
              // all tiles have sprites
              let sprite = tile_query.get::<SpriteInfo>((*tile_handle).entity).unwrap();

              InventoryUi::make_item(p2, 
                &*sprite, 
                name.0.clone(), 
                font_handle, 
                materials.add(Color::rgb(0.08, 0.08, 1.0).into()));     
              }
          });
        })
        .with(InventoryUi);
      } else {
        for (e, _i) in &mut ui_query.iter(){
            println!("Invetory shown, removing it");
            commands.despawn_recursive(e);
        }
      }
    }
  }
}