use bevy::{prelude::*};
use lab_sprites::{SpriteInfo, SpriteLibrary};
use std::time::Duration;
use lab_entities::prelude::{Movement, NonPlayer};
use lab_world::Dialog;
use lab_core::Zoomable;

pub fn npc_dialog_system (  
    mut commands : Commands,  
    mut query : Query<(Entity, &NonPlayer, &Translation, &SpriteInfo, &Scale)>,    
    mut m_query: Query<(Entity, &NonPlayer, &Translation, &Timer)>,
    mut d_query: Query<(Entity,&Dialog)>)
{
    for (entity, np, sprite, timer) in &mut m_query.iter() {       
        if timer.elapsed == 0. {
            println!("timer finished");
            if let Some(d) = d_query.iter().into_iter().filter(|i| i.1.entity == entity).last(){         
                commands.despawn(d.0);
            }          
        }
    }       
    for (entity, np, t, sprite_info, scale) in &mut query.iter(){
        // if a npc doesn't have dialog, make them say something
        if let None = d_query.iter().into_iter().filter(|i| i.1.entity == entity).last() {
            println!("Adding dialog");       

            commands.spawn((scale.clone(), sprite_info.clone(), t.clone(), Dialog { text: "I'm an NPC".to_string(), entity:entity },));
        }       
    }

    //commands.insert_children(entity, 0, &[e]);
}

pub fn dialog_system (
    mut commands : Commands,
    sprite_library: Res<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut assets: ResMut<Assets<Font>>,
    mut d_query: Query<(Entity,&Dialog)>,
    mut query : Query<(Entity, Added<Dialog>, &SpriteInfo, &Translation, &Scale)>
) {
    for (entity, mut dialog, sprite_info, translation, scale) in &mut query.iter() {       
    
        let font_handle = asset_server.load_sync(&mut assets, "resources/fonts/FiraSans-Bold.ttf").unwrap();
        let sprite_scaled_size = sprite_info.scaled_size(scale.0);

        let window = windows.iter().last().unwrap();

        let mut loc = translation.0.clone();

        commands.spawn(  TextComponents {        
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect { bottom:Val::Px(loc.y()), left: Val::Px(loc.x()) , ..Default::default()},

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
        }).with_bundle((dialog.clone(),Zoomable,Translation::new(loc.x() as f32 + (window.width/2) as f32+sprite_scaled_size.x(),loc.y() as f32 + (window.height/2) as f32 + (sprite_scaled_size.y() * 0.6),0.)));
    }    
}
