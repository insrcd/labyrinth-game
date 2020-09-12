use lab_core::prelude::*;
use lab_entities::prelude::*;
use crate::dialog::*;
use rand::*;


/// Super Basic right now, Move all NPCs in the scene every n seconds
pub fn npc_move_system(
    mut commands : Commands,
    time: Res<Time>,
    settings: Res<WorldSettings>,
    mut query: Query<(
        Entity,
        &NonPlayer,
        &Scale, 
        &mut Timer,
        &mut Translation
    )>,
) {
    for (entity, _np,  scale, mut timer, mut trans) in &mut query.iter() {
        timer.tick(time.delta_seconds);
        if timer.finished {
            let old_loc = Location::from(*trans);
            let direction = rand::random::<CardinalDirection>();
            match direction {
                CardinalDirection::West => *trans.0.x_mut() -= settings.base_npc_speed * scale.0, // replace with npc speed
                CardinalDirection::North => *trans.0.y_mut() += settings.base_npc_speed * scale.0,
                CardinalDirection::South => *trans.0.y_mut() -= settings.base_npc_speed * scale.0,
                CardinalDirection::East => *trans.0.x_mut() += settings.base_npc_speed * scale.0,
                CardinalDirection::None => {}
            }

            commands.insert(entity, ( 
                    Movement { 
                        start:old_loc, 
                        end:Location::from(*trans), 
                        direction: direction, 
                        legal: Some(true)
                    },
                )
            );

            timer.reset();
        }
    }
}