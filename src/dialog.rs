use bevy::{prelude::*, render::camera::Camera};
use lab_sprites::SpriteInfo;
use lab_entities::prelude::{Player, NonPlayer, Location};
use lab_world::{TextChangeEvent, Dialog};
use lab_core::{Named, Zoomable};

pub fn npc_dialog_system (  
    mut commands : Commands,  
    mut query : Query<(Entity, &NonPlayer, &Translation, &SpriteInfo, &Scale)>,    
    mut m_query: Query<(Entity, &NonPlayer, &Translation, &Timer)>,
    mut d_query: Query<(Entity,&Dialog)>)
{
    for (entity, _np, _translation, timer) in &mut m_query.iter() {       
        if timer.elapsed == 0. {
            if let Some(d) = d_query.iter().into_iter().filter(|i| i.1.entity == entity).last(){         
                commands.despawn(d.0);
            }          
        }
    }       
    for (entity, _np, t, sprite_info, scale) in &mut query.iter(){
        // if a npc doesn't have dialog, make them say something
        if let None = d_query.iter().into_iter().filter(|i| i.1.entity == entity).last() {     

            commands.spawn((scale.clone(), sprite_info.clone(), t.clone(), Dialog { text: "I'm an NPC".to_string(), entity:entity },));
        }       
    }

    //commands.insert_children(entity, 0, &[e]);
}


pub fn dialog_system (
    mut text: ResMut<Events<TextChangeEvent>>,
    windows: Res<Windows>,
    mut player_query : Query<(Entity, &Player, &Translation)>,
    mut m_query: Query<(Entity, &NonPlayer, &Translation, &Named)>,
    mut query : Query<(Entity, Added<Dialog>, &Translation)>,
    mut camera_query: Query<(&Camera, &Translation)>
) {
    let window = windows.iter().last().unwrap();
    let c_trans = camera_query.iter()
        .into_iter()
        .filter_map(|(c,t)| if c.name == Some("Camera2d".to_string()) { Some(t) } else {None} )
        .last();
         
    if let Some(t) = player_query.iter().into_iter().last() {
        for (_entity, dialog, translation) in &mut query.iter() {  
            let distance = Vec2::new((*t.2).x(), (*t.2).y()) - Vec2::new(translation.x(), translation.y());
            if distance.x().abs() + distance.y().abs() < 100. { 
                if let Ok(named) = &mut m_query.get::<Named>(dialog.entity) {     
                    text.send(TextChangeEvent { text: format!("{} says: \"{}\"", named.0, dialog.text).to_string(), name: "log".to_string()});
                }
            }

        }
    }
    
    
        
/*
        let text = Entity::new();
        commands.spawn_as_entity(text, TextComponents {        
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
        }).with_bundle((dialog.clone(),Zoomable,));*/

//        commands.insert_children(dialog.entity, 0, &[text]);
        //commands.push_children(   dialog.entity, &[text]);
        //Translation::new(loc.x() as f32 + (window.width/2) as f32+sprite_scaled_size.x(),loc.y() as f32 + (window.height/2) as f32 + (sprite_scaled_size.y() * 0.6),0.))
}
