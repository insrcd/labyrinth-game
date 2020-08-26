use bevy::{prelude::*};
use lab_sprites::{SpriteInfo, SpriteLibrary};
use std::time::Duration;
use lab_entities::prelude::{Movement, NonPlayer};

pub struct Dialog {
    text : String,
    entity: Entity
}
pub fn npc_dialog_system (  
    mut commands : Commands,  
    mut query : Query<(Entity, Added<NonPlayer>, &Translation, &SpriteInfo, &Scale)>){
    for (entity, np, t, sprite_info, scale) in &mut query.iter(){
    // if a npc doesn't have dialog, make them say something
        println!("Adding dialog");       

        let e = Entity::new();

        commands.spawn_as_entity(e,(scale.clone(), sprite_info.clone(), t.clone(), Dialog { text: "I'm an npc that talks and moves!".to_string(), entity:entity },));
        //commands.insert_children(entity, 0, &[e]);
    }
}

pub fn dialog_system (
    mut commands : Commands,
    sprite_library: Res<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut assets: ResMut<Assets<Font>>,
    mut query : Query<Without<Text, (Entity, &mut Dialog, &SpriteInfo, &Translation, &Scale)>>,
    mut m_query: Query<(Entity, &NonPlayer, &Translation, &Movement)>
) {
    for (entity, mut dialog, sprite_info, translation, scale) in &mut query.iter() {       
    
        let font_handle = asset_server.load_sync(&mut assets, "resources/fonts/FiraSans-Bold.ttf").unwrap();
        let sprite_scaled_size = sprite_info.scaled_size(scale.0);

        let window = windows.iter().last().unwrap();

        let mut loc = translation.0.clone() + sprite_scaled_size;

        commands.spawn(  TextComponents {        
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect { top:Val::Px(loc.x() -(window.width/4) as f32), left: Val::Px(loc.y()-(window.height/2) as f32) , ..Default::default()},

                ..Default::default()
            },
            text: Text {
                value: dialog.text.to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            },
            draw: Draw {is_visible: true, ..Default::default()},
            ..Default::default()
        });
    }
    /*for (entity, np, sprite, dialog, mov) in &mut m_query.iter() {                
        commands.remove_one::<Dialog>(entity);
    }*/
}
