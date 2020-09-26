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
  _text_query: Query<&mut Text>,
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
pub struct UiHelper {
  button_material: Handle<ColorMaterial>,
  material: Handle<ColorMaterial>,
  font_handle: Handle<Font>
}

impl UiHelper {
  pub fn container(&mut self, size: Size<Val>) -> NodeComponents {
    NodeComponents {
      style: Style {
          size: size,
          align_self: AlignSelf::FlexStart,
          ..Default::default()
      },
      material: self.material,
      ..Default::default()
    }
  }
  pub fn flex_container(&mut self, size: Size<Val>, material: Handle<ColorMaterial>) -> NodeComponents {
    NodeComponents {
      style: Style {
          size: size,
          align_self: AlignSelf::FlexStart,
          align_content: AlignContent::FlexStart,
          justify_content: JustifyContent::FlexStart,
          flex_wrap: FlexWrap::WrapReverse,
          ..Default::default()
      },
      material: material,
      ..Default::default()
    }
  }

  pub fn vert_container(&mut self, size: Size<Val>, material: Handle<ColorMaterial>) -> NodeComponents {
    NodeComponents {
      style: Style {
          size: size,
          flex_direction: FlexDirection::ColumnReverse,
          align_content: AlignContent::FlexStart,
          justify_content: JustifyContent::FlexStart,
          align_items: AlignItems::Center,
          ..Default::default()
      },
      material: material,
      ..Default::default()
    }
  }
  pub fn build_button(&mut self, builder: &mut ChildBuilder, text: &str){
      builder.spawn(ButtonComponents {
        style: Style {
            align_self:AlignSelf::Center,
            size: Size::new(Val::Percent(80.0), Val::Px(50.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: Rect {
              top: Val::Px(20.0),
              ..Default::default()
            },
            ..Default::default()
        },
        material: self.button_material,
        ..Default::default()
      }).with_children(|p2| {
        p2.spawn(TextComponents {
            text: Text {
                value: text.into(),
                font: self.font_handle,
                style: TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                },
            },
            ..Default::default()
        });
      });
  }
  pub fn text(&mut self, text: &str) -> TextComponents {
    TextComponents {
      style: Style {
        ..Default::default()
      },
      text: Text {
          value: text.into(),
          font: self.font_handle,
          style: TextStyle {
              font_size: 40.0,
              color: Color::WHITE,
          },
      },
      draw: Draw {is_visible: true, is_transparent: true, ..Default::default()},
      ..Default::default()
    }
  }
  fn build_item(&self, parent : &mut ChildBuilder, sprite: &SpriteInfo, 
    text: String, 
    font_handle: Handle<Font>,
    material: Handle<ColorMaterial>) 
  {
    parent.spawn(NodeComponents {
      style: Style {
          size: Size::new(Val::Px(100.0), Val::Px(100.0)),
          justify_content: JustifyContent::Center,
          align_content: AlignContent::Center,
          margin: Rect::all(Val::Px(2.0)),
          ..Default::default()
      },
      material: material,
      ..Default::default()
    }).with_children(|p2| {
      p2.spawn(NodeComponents {
        style: Style {
          position_type: PositionType::Absolute,
          position: Rect { 
            top: Val::Px(40.0),
            ..Default::default()
          },
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


type HColor = Handle<ColorMaterial>;
pub struct UiColors {
  grey : HColor,
  grey2 : HColor,
  blue : HColor,
  _blue2 : HColor,
  _blue3 :HColor,
  _black :HColor,
  white :HColor
}

impl UiColors{
  fn new( mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
    UiColors {
      grey : materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
      grey2 : materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
      blue : materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
      _blue2 : materials.add(Color::rgb(0.3, 0.3, 1.0).into()),
      _blue3 :materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
      _black :materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
      white :materials.add(Color::rgb(1.0, 1.0, 1.0).into())
    }
  }
}

pub fn inventory_ui_system (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut assets: ResMut<Assets<Font>>,
    items: ResMut<Items>,
    button_materials: Res<ButtonMaterials>,
    mut state_query: Query<(Entity, Changed<UiState>)>,
    item_query: Query<(Entity, &WorldHandle<Item>, &Named)>,
    _tile_query: Query<(&WorldHandle<Tile>, &SpriteInfo)>,
    mut ui_query: Query<(Entity, &InventoryUi)>,
    mut inventory_query: Query<(Entity, &Player, &mut Inventory)>
) {
  let font_handle = asset_server.load_sync(&mut assets, "resources/fonts/FiraSans-Bold.ttf").unwrap();
  let colors = UiColors::new(materials);
  for (_entity, state_change) in &mut state_query.iter() {
    for  (_e, _player, inv)  in &mut inventory_query.iter(){
      if let UiState::Inventory = *state_change {
        let mut ui = UiHelper { 
          button_material: button_materials.normal,
          material: colors.white, 
          font_handle: font_handle
        };
        let whole_size = Size::new(Val::Percent(100.0), Val::Percent(100.0));
        let button_column = Size::new(Val::Percent(20.0), Val::Percent(100.0));
        let inventory_column = Size::new(Val::Percent(80.0), Val::Percent(100.0));

        commands
          .spawn(ui.container(whole_size))
          .with(InventoryUi)
          .with_children(|c| {
            c.spawn(ui.vert_container(button_column, colors.blue))
              .with_children(|c| {
                c.spawn(ui.text("Inventory"));
                ui.build_button(c, "Items");
              });
            c.spawn(ui.flex_container(inventory_column, colors.grey2))
            .with_children(|parent| {
              for item in inv.0.iter(){
                let entity = items.items.get(item).unwrap();
                // and are named
                let name = item_query.get::<Named>(*entity).unwrap();
                // all tiles have sprites
                let sprite = item_query.get::<SpriteInfo>(*entity).unwrap();
    
                ui.build_item(parent, 
                  &*sprite, 
                  name.0.clone(), 
                  font_handle, 
                  colors.grey);  
              };
            });
        });
      } else {
        for (e, _i) in &mut ui_query.iter(){
            println!("Invetory shown, removing it");
            commands.despawn_recursive(e);
        }
      }
    }
  }
}