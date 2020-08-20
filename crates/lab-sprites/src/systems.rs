
use lab_entities::prelude::*;
use bevy::prelude::*;


pub fn text_despawn(
    mut commands: Commands,
    mut query : Query<(Entity, &TextureAtlasSprite, &world::Despawn, &Timer)>
){
for (e, sprite, _dspawn, timer) in &mut query.iter(){
    if timer.finished {
        commands.despawn(e);
    }
}
}