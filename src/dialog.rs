use bevy::{prelude::*};
use lab_sprites::SpriteLibrary;
use std::time::Duration;
use lab_entities::prelude::{Movement, NonPlayer};

pub struct Dialog {
    text : String,
    sprites : Option<Vec<Entity>>
}
pub fn npc_dialog_system (  
    mut commands : Commands,  
    mut query : Query<Without<Dialog,(Entity, &NonPlayer, &Translation)>>){
    for (entity, np, t) in &mut query.iter(){
    // if a npc doesn't have dialog, make them say something
        
        commands.insert_one(entity, Dialog { text: "I'm an npc that talks and moves!".to_string(), sprites:None });
    }
}

pub fn dialog_system (
    mut commands : Commands,
    sprite_library: Res<SpriteLibrary>,
    mut query : Query<(Entity, &mut Dialog, &NonPlayer, &Translation)>,
    mut m_query: Query<(Entity, &NonPlayer, &Translation, &mut Dialog, Changed<Movement>)>
) {
    for (entity, mut dialog, sprite, translation) in &mut query.iter() {       
        //println!("Writing text");
        if let None = dialog.sprites {
            let mut loc = translation.0.clone();
            
            *loc.x_mut() += 48.;
            *loc.y_mut() += 48.;

            dialog.sprites = Some(sprite_library.write_text(&mut commands, dialog.text.clone(), loc));   
        }
    }
    for (entity, np, sprite, dialog, mov) in &mut m_query.iter() {        
        if let Some(sprites) = &dialog.sprites {
            for e in sprites.iter() {
                commands.despawn(*e);
            }
        };
        commands.remove_one::<Dialog>(entity);
    }
}
